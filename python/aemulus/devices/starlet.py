import math
import threading
import typing
from dataclasses import dataclass, field
from importlib.resources import files

from mujoco import MjData, MjModel

from aemulus.device import Device

TIP_50UL_LENGTH = 0.0504
PLUNGER_UL_PER_M = 1000.0

_CHANNELS = 8
_PICKUP_XY_TOL = 0.003
_PICKUP_Z_TOL = 0.004
_EJECT_THRESHOLD = 0.005
_EJECT_RELEASED = 0.002


@dataclass(frozen=True)
class _ChannelInfo:
    pipette_id: int
    plunger_qposadr: int
    eject_qposadr: int
    weld_eq_id: int


@dataclass(frozen=True)
class _ContainerInfo:
    labware_name: str
    body_id: int
    kind: str
    half_x: float
    half_y: float
    max_z: float


@dataclass
class _StarletState:
    lock: threading.Lock = field(default_factory=threading.Lock)
    initialized: bool = False
    channels: list[_ChannelInfo] = field(default_factory=list)
    tips: dict[int, tuple[str, float]] = field(default_factory=dict)
    max_tip_top_z: float = 0.0
    containers: list[_ContainerInfo] = field(default_factory=list)
    attached: dict[int, int] = field(default_factory=dict)
    attached_ids: set[int] = field(default_factory=set)
    ejected: set[int] = field(default_factory=set)
    prev_plunger: list[float] = field(default_factory=lambda: [0.0] * _CHANNELS)
    channel_volume_ul: list[float] = field(default_factory=lambda: [0.0] * _CHANNELS)
    volumes_ul: dict[tuple[str, str], float] = field(default_factory=dict)


@typing.final
@dataclass(frozen=True)
class STARlet(Device):
    mjcf = str(files("aemulus") / "assets/hamilton_starlet/starlet.xml")
    sim_server_name = "starlet"
    _state: _StarletState = field(default_factory=_StarletState, init=False, repr=False)

    def set_liquid(self, labware: str, volume_ul: float, well: str = "") -> None:
        with self._state.lock:
            self._state.volumes_ul[(labware, well.upper())] = float(volume_ul)

    def volume_ul(self, labware: str, well: str = "") -> float:
        with self._state.lock:
            return self._state.volumes_ul.get((labware, well.upper()), 0.0)

    def attached_tip(self, channel: int) -> str | None:
        with self._state.lock:
            tip_id = self._state.attached.get(channel)
            if tip_id is None:
                return None
            return self._state.tips[tip_id][0]

    def channel_volume_ul(self, channel: int) -> float:
        with self._state.lock:
            return self._state.channel_volume_ul[channel]

    def _init_indices(self, model: MjModel, data: MjData) -> None:
        state = self._state
        prefix = f"{self.device_name}/"
        state.channels = [
            _ChannelInfo(
                pipette_id=model.body(f"{prefix}pipette{ch}").id,
                plunger_qposadr=int(model.joint(f"{prefix}plunger{ch}").qposadr[0]),
                eject_qposadr=int(model.joint(f"{prefix}eject{ch}").qposadr[0]),
                weld_eq_id=model.equality(f"{prefix}tip_weld_{ch}").id,
            )
            for ch in range(_CHANNELS)
        ]
        state.prev_plunger = [
            float(data.qpos[info.plunger_qposadr]) for info in state.channels
        ]
        state.tips = {}
        state.max_tip_top_z = 0.0
        state.containers = []
        for body_id in range(model.nbody):
            name = model.body(body_id).name
            if "/tip_" in name and model.body_jntnum[body_id] == 1:
                geom_id = model.body_geomadr[body_id]
                tip_length = float(model.geom_size[geom_id][1] * 2.0)
                state.tips[body_id] = (name, tip_length)
                top_z = float(data.xpos[body_id, 2]) + tip_length + _PICKUP_Z_TOL
                if top_z > state.max_tip_top_z:
                    state.max_tip_top_z = top_z
            elif name.endswith("/nunc_96_plate"):
                state.containers.append(
                    _ContainerInfo(
                        labware_name=name.split("/", 1)[0],
                        body_id=body_id,
                        kind="plate",
                        half_x=0.06388,
                        half_y=0.042735,
                        max_z=0.020,
                    )
                )
            elif name.endswith("/reservoir"):
                state.containers.append(
                    _ContainerInfo(
                        labware_name=name.split("/", 1)[0],
                        body_id=body_id,
                        kind="reservoir",
                        half_x=0.06375,
                        half_y=0.043,
                        max_z=0.025,
                    )
                )
        state.initialized = True

    def _container_at(
        self, data: MjData, x: float, y: float, z: float
    ) -> tuple[str, str] | None:
        xpos = data.xpos
        for container in self._state.containers:
            pos = xpos[container.body_id]
            cx = float(pos[0])
            cy = float(pos[1])
            cz = float(pos[2])
            if (
                abs(x - cx) <= container.half_x
                and abs(y - cy) <= container.half_y
                and cz - 0.002 <= z <= cz + container.max_z
            ):
                if container.kind == "reservoir":
                    return (container.labware_name, "")
                col = round((x - cx + 0.0495) / 0.009)
                row = round((0.0315 - (y - cy)) / 0.009)
                if 0 <= row < 8 and 0 <= col < 12:
                    wx = cx - 0.0495 + col * 0.009
                    wy = cy + 0.0315 - row * 0.009
                    if math.hypot(x - wx, y - wy) <= 0.004:
                        return (
                            container.labware_name,
                            f"{chr(ord('A') + row)}{col + 1}",
                        )
        return None

    def on_step(self, model: MjModel, data: MjData) -> None:
        state = self._state
        with state.lock:
            if not state.initialized:
                self._init_indices(model, data)

            xpos = data.xpos
            qpos = data.qpos
            for ch, info in enumerate(state.channels):
                pos = xpos[info.pipette_id]
                px = float(pos[0])
                py = float(pos[1])
                pz = float(pos[2])
                eject = float(qpos[info.eject_qposadr])
                plunger = float(qpos[info.plunger_qposadr])
                dp = plunger - state.prev_plunger[ch]
                state.prev_plunger[ch] = plunger

                tip_id = state.attached.get(ch)
                if tip_id is not None:
                    if eject >= _EJECT_THRESHOLD:
                        data.eq_active[info.weld_eq_id] = 0
                        model.body_gravcomp[tip_id] = 0.0
                        del state.attached[ch]
                        state.attached_ids.discard(tip_id)
                        state.ejected.add(tip_id)
                        state.channel_volume_ul[ch] = 0.0
                        continue

                    if abs(dp) > 1e-7:
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
                elif eject < _EJECT_RELEASED and pz <= state.max_tip_top_z:
                    for cand_id, (_, tip_length) in state.tips.items():
                        if cand_id in state.attached_ids or cand_id in state.ejected:
                            continue
                        tpos = xpos[cand_id]
                        if (
                            math.hypot(px - float(tpos[0]), py - float(tpos[1]))
                            <= _PICKUP_XY_TOL
                            and abs(pz - (float(tpos[2]) + tip_length))
                            <= _PICKUP_Z_TOL
                        ):
                            model.eq_obj2id[info.weld_eq_id] = cand_id
                            model.eq_data[info.weld_eq_id, 3:6] = (
                                0.0,
                                0.0,
                                -tip_length,
                            )
                            data.eq_active[info.weld_eq_id] = 1
                            state.attached[ch] = cand_id
                            state.attached_ids.add(cand_id)
                            break
