import math
import threading
import typing
from dataclasses import dataclass, field
from importlib.resources import files

from mujoco import MjData, MjModel

from aemulus.device import Device
from aemulus.lab import sbs_96_offset

TIP_50UL_LENGTH = 0.0504
PLUNGER_UL_PER_M = 1000.0

_CHANNELS = 8
_A1_X, _A1_Y = sbs_96_offset("A1")
_PITCH = sbs_96_offset("A2")[0] - _A1_X


class _StarletState:
    def __init__(self) -> None:
        self.lock = threading.Lock()
        self.initialized = False
        self.channels: list[tuple[int, int, int, int, int]] = []
        self.tips: dict[int, tuple[str, float]] = {}
        self.max_tip_top_z = 0.0
        self.containers: list[tuple[str, int, bool, float, float, float]] = []
        self.attached: dict[int, int] = {}
        self.attached_ids: set[int] = set()
        self.ejected: set[int] = set()
        self.prev_plunger = [0.0] * _CHANNELS
        self.channel_volume_ul = [0.0] * _CHANNELS
        self.volumes_ul: dict[tuple[str, str], float] = {}


@typing.final
@dataclass(frozen=True)
class STARlet(Device):
    mjcf = str(files("aemulus") / "assets/hamilton_starlet/starlet.xml")
    sim_server_name = "starlet"
    _state: _StarletState = field(
        default_factory=_StarletState, init=False, repr=False, compare=False
    )

    def set_liquid(self, labware: str, volume_ul: float, well: str = "") -> None:
        with self._state.lock:
            self._state.volumes_ul[(labware, well.upper())] = float(volume_ul)

    def volume_ul(self, labware: str, well: str = "") -> float:
        with self._state.lock:
            return self._state.volumes_ul.get((labware, well.upper()), 0.0)

    def attached_tip(self, channel: int) -> str | None:
        with self._state.lock:
            tip_id = self._state.attached.get(channel)
            return None if tip_id is None else self._state.tips[tip_id][0]

    def channel_volume_ul(self, channel: int) -> float:
        with self._state.lock:
            return self._state.channel_volume_ul[channel]

    def _init_indices(self, model: MjModel, data: MjData) -> None:
        state = self._state
        prefix = f"{self.device_name}/"
        state.channels = [
            (
                model.body(f"{prefix}pipette{ch}").id,
                int(model.joint(f"{prefix}z{ch}").dofadr[0]),
                int(model.joint(f"{prefix}plunger{ch}").qposadr[0]),
                int(model.joint(f"{prefix}eject{ch}").qposadr[0]),
                model.equality(f"{prefix}tip_weld_{ch}").id,
            )
            for ch in range(_CHANNELS)
        ]
        state.prev_plunger = [float(data.qpos[p]) for _, _, p, _, _ in state.channels]
        for body_id in range(model.nbody):
            name = model.body(body_id).name
            if "/tip_" in name and model.body_jntnum[body_id] == 1:
                geom_id = model.body_geomadr[body_id]
                tip_length = float(model.geom_size[geom_id][1] * 2.0)
                state.tips[body_id] = (name, tip_length)
                state.max_tip_top_z = max(
                    state.max_tip_top_z, float(data.xpos[body_id, 2]) + tip_length + 0.004
                )
            elif name.endswith("/nunc_96_plate") or name.endswith("/reservoir"):
                # Collision geom is last on reservoir (after visual walls) and sole geom on plate.
                geom_id = model.body_geomadr[body_id] + model.body_geomnum[body_id] - 1
                hx, hy, hz = model.geom_size[geom_id]
                state.containers.append(
                    (
                        name.split("/", 1)[0],
                        body_id,
                        name.endswith("/nunc_96_plate"),
                        float(hx),
                        float(hy),
                        float(hz * 2.0) + 0.006,
                    )
                )
        state.initialized = True

    def _container_at(
        self, data: MjData, x: float, y: float, z: float
    ) -> tuple[str, str] | None:
        xpos = data.xpos
        for labware_name, body_id, is_plate, hx, hy, max_z in self._state.containers:
            pos = xpos[body_id]
            cx, cy, cz = float(pos[0]), float(pos[1]), float(pos[2])
            if not (abs(x - cx) <= hx and abs(y - cy) <= hy and cz - 0.002 <= z <= cz + max_z):
                continue
            if not is_plate:
                return (labware_name, "")
            col = round((x - cx - _A1_X) / _PITCH)
            row = round((_A1_Y - (y - cy)) / _PITCH)
            if 0 <= row < 8 and 0 <= col < 12:
                well = f"{chr(ord('A') + row)}{col + 1}"
                wx, wy = sbs_96_offset(well)
                if math.hypot(x - (cx + wx), y - (cy + wy)) <= 0.004:
                    return (labware_name, well)
        return None

    def on_step(self, model: MjModel, data: MjData) -> None:
        state = self._state
        with state.lock:
            if not state.initialized:
                self._init_indices(model, data)

            xpos = data.xpos
            qpos = data.qpos
            qvel = data.qvel
            for ch, (pipette_id, z_dof, plunger_adr, eject_adr, weld_id) in enumerate(
                state.channels
            ):
                pos = xpos[pipette_id]
                px, py, pz = float(pos[0]), float(pos[1]), float(pos[2])
                eject = float(qpos[eject_adr])
                plunger = float(qpos[plunger_adr])
                dp = plunger - state.prev_plunger[ch]
                state.prev_plunger[ch] = plunger

                tip_id = state.attached.get(ch)
                if tip_id is not None:
                    # Release weld and restore gravity when eject sleeve extends.
                    if eject >= 0.005:
                        data.eq_active[weld_id] = 0
                        model.body_gravcomp[tip_id] = 0.0
                        del state.attached[ch]
                        state.attached_ids.discard(tip_id)
                        state.ejected.add(tip_id)
                        state.channel_volume_ul[ch] = 0.0
                        continue

                    if abs(dp) > 1e-7 and abs(float(qvel[z_dof])) < 0.02:
                        _, tip_length = state.tips[tip_id]
                        target = self._container_at(data, px, py, pz - tip_length)
                        if target is not None:
                            dv = dp * PLUNGER_UL_PER_M
                            if dv > 0.0:
                                available = state.volumes_ul.get(target, 0.0)
                                taken = min(dv, available)
                                state.volumes_ul[target] = available - taken
                                state.channel_volume_ul[ch] += taken
                            else:
                                given = min(-dv, state.channel_volume_ul[ch])
                                state.channel_volume_ul[ch] -= given
                                state.volumes_ul[target] = (
                                    state.volumes_ul.get(target, 0.0) + given
                                )
                elif eject < 0.002 and pz <= state.max_tip_top_z:
                    # Weld tip when nozzle meets an unpicked tip collar.
                    for cand_id, (_, tip_length) in state.tips.items():
                        if cand_id in state.attached_ids or cand_id in state.ejected:
                            continue
                        tpos = xpos[cand_id]
                        if (
                            math.hypot(px - float(tpos[0]), py - float(tpos[1])) <= 0.003
                            and abs(pz - (float(tpos[2]) + tip_length)) <= 0.004
                        ):
                            model.eq_obj2id[weld_id] = cand_id
                            model.eq_data[weld_id, 3:6] = (0.0, 0.0, -tip_length)
                            data.eq_active[weld_id] = 1
                            state.attached[ch] = cand_id
                            state.attached_ids.add(cand_id)
                            break
