import asyncio
import socket
import threading
import time
from dataclasses import dataclass
from importlib.resources import files

import mujoco.viewer
import numpy as np
from hypercorn import Config
from hypercorn.asyncio import serve
from jaxtyping import Float
from mujoco import (
    MjData,
    MjModel,
    MjSpec,
    mj_forward,
    mj_getState,
    mj_resetData,
    mj_setState,
    mj_stateSize,
    mj_step,
    mjtState,
)

from aemulus.device import Device
from aemulus.sim.v1.sim_connect import SimService, SimServiceASGIApplication
from aemulus.sim.v1.sim_pb import (
    GetJointRequest,
    GetJointResponse,
    SetActuatorRequest,
    SetActuatorResponse,
)


@dataclass(frozen=True)
class Labware:
    name: str
    mjcf: str
    pos: tuple[float, float, float]
    free: bool = False


def sbs_96_offset(well: str) -> tuple[float, float]:
    """Return (dx, dy) in meters from labware center for a 96-position well/tip ('A1'..'H12')."""
    if len(well) < 2:
        raise ValueError(f"Invalid 96-well name: {well!r}")
    row = ord(well[0].upper()) - ord("A")
    try:
        col = int(well[1:]) - 1
    except ValueError as err:
        raise ValueError(f"Invalid 96-well name: {well!r}") from err
    if not (0 <= row < 8 and 0 <= col < 12):
        raise ValueError(f"Invalid 96-well name: {well!r}")
    return (-0.0495 + col * 0.009, 0.0315 - row * 0.009)


def create_lab(devices: list[Device], labware: list[Labware] | None = None) -> Lab:
    lab = Lab(devices, labware)
    if labware or any(device.address is None for device in devices):
        lab.sim = SimServer(lab)
        run_sim(lab.sim)
    return lab


class Lab:
    devices: list[Device]
    model: MjModel
    sim: SimServer | None

    def __init__(self, devices: list[Device], labware: list[Labware] | None = None):
        self.devices = devices
        self.sim = None
        self._labware_bodies: dict[str, str] = {}
        self._labware_pos: dict[str, tuple[float, float, float]] = {}
        spec = MjSpec.from_file(str(files("aemulus") / "assets/lab.xml"))
        for device in self.devices:
            if device.address is None:
                if device.pos is None:
                    raise ValueError(f"Device {device.device_name} has no position")
                child = MjSpec.from_file(device.mjcf)
                frame = spec.worldbody.add_frame(pos=device.pos)
                spec.attach(child, prefix=f"{device.device_name}/", frame=frame)
                device.start_sim_server()

        for item in labware or []:
            child = MjSpec.from_file(item.mjcf)
            root_body = child.worldbody.bodies[0]
            root_name = root_body.name
            if item.free:
                root_body.add_freejoint()
            frame = spec.worldbody.add_frame(pos=item.pos)
            spec.attach(child, prefix=f"{item.name}/", frame=frame)
            self._labware_bodies[item.name] = f"{item.name}/{root_name}"
            self._labware_pos[item.name] = item.pos

        self.model = spec.compile()

    def labware_pos(self, name: str) -> np.ndarray:
        """Return current world (x, y, z) position of a labware root body."""
        if name not in self._labware_bodies:
            raise KeyError(f"Unknown labware: {name!r}")
        if self.sim is None:
            return np.array(self._labware_pos[name], dtype=float)
        data = MjData(self.model)
        mj_setState(self.model, data, self.sim.state, mjtState.mjSTATE_FULLPHYSICS)
        mj_forward(self.model, data)
        return data.body(self._labware_bodies[name]).xpos.copy()

    def well_xy(
        self, labware_name: str, well: str | None = None
    ) -> tuple[float, float]:
        """Return current world (x, y) of a well/tip ('A1'..'H12') or labware center."""
        pos = self.labware_pos(labware_name)
        if well is None:
            return (float(pos[0]), float(pos[1]))
        dx, dy = sbs_96_offset(well)
        return (float(pos[0] + dx), float(pos[1] + dy))

    def keep_alive(self):
        try:
            while True:
                time.sleep(0.1)
        except KeyboardInterrupt:
            pass

    def view(self) -> None:
        if self.sim is None:
            raise ValueError("no simulation to view")
        sim = self.sim

        def _loop():
            data = MjData(self.model)
            spec = mjtState.mjSTATE_FULLPHYSICS
            with mujoco.viewer.launch_passive(self.model, data) as viewer:
                while viewer.is_running():
                    with viewer.lock():
                        mj_setState(self.model, data, sim.state, spec)
                        mj_forward(self.model, data)
                    viewer.sync()
                    time.sleep(1 / 60)

        threading.Thread(target=_loop, daemon=True).start()


class SimServer(SimService):
    model: MjModel
    data: MjData
    state: Float[np.ndarray, "nstate"]
    ctrl: Float[np.ndarray, "nu"]

    def __init__(self, lab: Lab):
        self.model = lab.model
        self.devices = [device for device in lab.devices if device.address is None]
        self.data = MjData(self.model)
        self.state = self.reset()
        joints = self.model.actuator_trnid[:, 0]
        self.ctrl = self.state[1 + self.model.jnt_qposadr[joints]]

    def reset(self) -> Float[np.ndarray, "nstate"]:
        spec = mjtState.mjSTATE_FULLPHYSICS
        mj_resetData(self.model, self.data)
        state = np.empty(mj_stateSize(self.model, spec))
        mj_getState(self.model, self.data, state, spec)
        return state

    def step(self) -> None:
        mj_setState(self.model, self.data, self.state, mjtState.mjSTATE_FULLPHYSICS)
        self.data.ctrl[:] = self.ctrl
        mj_step(self.model, self.data)
        for device in self.devices:
            device.on_step(self.model, self.data)
        next_state = np.empty_like(self.state)
        mj_getState(self.model, self.data, next_state, mjtState.mjSTATE_FULLPHYSICS)
        self.state = next_state

    async def get_joint(self, request: GetJointRequest, ctx, /) -> GetJointResponse:
        index = 1 + self.model.joint(request.joint).qposadr[0]
        return GetJointResponse(position=self.state[index])

    async def set_actuator(
        self, request: SetActuatorRequest, ctx, /
    ) -> SetActuatorResponse:
        self.ctrl[self.model.actuator(request.actuator).id] = request.value
        return SetActuatorResponse()


def wait_for_port(port: int, timeout: float = 10.0) -> None:
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        try:
            socket.create_connection(("127.0.0.1", port), timeout=0.2).close()
            return
        except OSError:
            time.sleep(0.05)
    raise TimeoutError(f"port {port} did not open")


def run_sim(sim_server: SimServer, port: int = 50051) -> None:
    """
    Starts the SiLA server for the simulation as well as the
    MuJoCo simulation itself in a thread.
    """

    def _run_server():
        config = Config()
        config.bind = [f"127.0.0.1:{port}"]

        async def main():
            app = SimServiceASGIApplication(sim_server)
            await serve(app, config, shutdown_trigger=asyncio.Event().wait)

        asyncio.run(main())

    def _loop():
        next_step = time.monotonic()
        while True:
            sim_server.step()
            next_step += sim_server.model.opt.timestep
            remaining = next_step - time.monotonic()
            if remaining > 0:
                time.sleep(remaining)

    threading.Thread(target=_run_server, daemon=True).start()
    wait_for_port(port)
    threading.Thread(target=_loop, daemon=True).start()
