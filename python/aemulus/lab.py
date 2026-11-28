import asyncio
import socket
import sys
import threading
import time
from importlib.resources import files
from pathlib import Path
from typing import AsyncIterator

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
from aemulus.labware import Labware
from aemulus.sim.v1.sim_connect import SimService, SimServiceASGIApplication
from aemulus.sim.v1.sim_pb import (
    GetJointRequest,
    GetJointResponse,
    GetModelRequest,
    GetModelResponse,
    SetActuatorRequest,
    SetActuatorResponse,
    StreamStateRequest,
    StreamStateResponse,
)


def _collect_meshes(child: MjSpec, prefix: str, meshes: dict[str, bytes]) -> None:
    for mesh in child.meshes:
        file = f"{prefix}_{mesh.file}"
        meshes[file] = (
            Path(child.modelfiledir) / child.meshdir / mesh.file
        ).read_bytes()
        mesh.file = file


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
    meshes: dict[str, bytes]
    labware_bodies: dict[str, str]

    def __init__(self, devices: list[Device], labware: list[Labware] | None = None):
        self.devices = devices
        self.sim = None
        self.labware_bodies = {}
        self.meshes = {}
        spec = MjSpec.from_file(str(files("aemulus") / "assets/lab.xml"))
        for device in self.devices:
            if device.address is None:
                if device.pos is None:
                    raise ValueError(f"Device {device.device_name} has no position")
                child = MjSpec.from_file(device.mjcf)
                frame = spec.worldbody.add_frame(pos=device.pos)
                prefix = f"{device.device_name}/"
                _collect_meshes(child, device.device_name, self.meshes)
                spec.attach(child, prefix=prefix, frame=frame)
                device.start_sim_server()

        for item in labware or []:
            child = MjSpec.from_file(item.mjcf)
            root_body = child.worldbody.bodies[0]
            root_name = root_body.name
            if item.free:
                root_body.add_freejoint()
            frame = spec.worldbody.add_frame(pos=item.pos)
            prefix = f"{item.name}/"
            _collect_meshes(child, item.name, self.meshes)
            spec.attach(child, prefix=prefix, frame=frame)
            self.labware_bodies[item.name] = f"{item.name}/{root_name}"

        spec.assets = {**self.meshes}
        self.model = spec.compile()
        self.xml = spec.to_xml()

    def labware_pos(self, name: str) -> Float[np.ndarray, "3"]:
        if self.sim is None:
            raise ValueError("no simulation")
        data = MjData(self.model)
        mj_setState(self.model, data, self.sim.state, mjtState.mjSTATE_FULLPHYSICS)
        mj_forward(self.model, data)
        return data.body(self.labware_bodies[name]).xpos.copy()

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
    data: MjData
    state: Float[np.ndarray, "nstate"]
    ctrl: Float[np.ndarray, "nu"]
    lab: Lab

    def __init__(self, lab: Lab):
        self.lab = lab
        self.devices = [device for device in lab.devices if device.address is None]
        self.data = MjData(self.lab.model)
        self.state = self.reset()
        joints = self.lab.model.actuator_trnid[:, 0]
        self.ctrl = self.state[1 + self.lab.model.jnt_qposadr[joints]]

    def reset(self) -> Float[np.ndarray, "nstate"]:
        spec = mjtState.mjSTATE_FULLPHYSICS
        mj_resetData(self.lab.model, self.data)
        state = np.empty(mj_stateSize(self.lab.model, spec))
        mj_getState(self.lab.model, self.data, state, spec)
        return state

    def step(self) -> None:
        mj_setState(self.lab.model, self.data, self.state, mjtState.mjSTATE_FULLPHYSICS)
        self.data.ctrl[:] = self.ctrl
        mj_step(self.lab.model, self.data)
        for device in self.devices:
            device.on_step(self.lab.model, self.data)
        next_state = np.empty_like(self.state)
        mj_getState(self.lab.model, self.data, next_state, mjtState.mjSTATE_FULLPHYSICS)
        self.state = next_state

    async def get_joint(self, request: GetJointRequest, ctx, /) -> GetJointResponse:
        joint = self.lab.model.joint(request.joint)
        position = self.state[1 + joint.qposadr[0]]
        velocity = self.state[1 + self.lab.model.nq + joint.dofadr[0]]
        return GetJointResponse(position=position, velocity=velocity)

    async def set_actuator(
        self, request: SetActuatorRequest, ctx, /
    ) -> SetActuatorResponse:
        self.ctrl[self.lab.model.actuator(request.actuator).id] = request.value
        return SetActuatorResponse()

    async def get_model(self, _request: GetModelRequest, ctx, /) -> GetModelResponse:
        return GetModelResponse(xml=self.lab.xml, meshes=self.lab.meshes)

    async def stream_state(
        self, request: StreamStateRequest, ctx, /
    ) -> AsyncIterator[StreamStateResponse]:
        while True:
            qpos = self.state[1 : 1 + self.lab.model.nq]
            yield StreamStateResponse(qpos=qpos.tolist())
            await asyncio.sleep(1 / 60)


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
        # The device servers poll joints over one connection for the whole run.
        config.keep_alive_max_requests = sys.maxsize

        async def main():
            app = SimServiceASGIApplication(sim_server)
            await serve(app, config, shutdown_trigger=asyncio.Event().wait)

        asyncio.run(main())

    def _loop():
        next_step = time.monotonic()
        while True:
            sim_server.step()
            next_step += sim_server.lab.model.opt.timestep
            remaining = next_step - time.monotonic()
            if remaining > 0:
                time.sleep(remaining)

    threading.Thread(target=_run_server, daemon=True).start()
    wait_for_port(port)
    threading.Thread(target=_loop, daemon=True).start()
