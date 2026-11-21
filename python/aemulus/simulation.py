import asyncio
import threading
import time
from importlib.resources import files

import mujoco
import mujoco.viewer
from connectrpc.request import RequestContext
from hypercorn import Config
from hypercorn.asyncio import serve
from loguru import logger

from aemulus.sim.v1.sim_connect import SimService, SimServiceASGIApplication
from aemulus.sim.v1.sim_pb import (
    GetJointRequest,
    GetJointResponse,
    SetActuatorRequest,
    SetActuatorResponse,
)


def run_server(service: PF400):
    config = Config()
    config.bind = ["127.0.0.1:50051"]

    async def main():
        app = SimServiceASGIApplication(service)
        await serve(app, config, shutdown_trigger=asyncio.Event().wait)

    asyncio.run(main())


def run_simulation():
    path = files("aemulus") / "assets/pf400/scene.xml"
    model = mujoco.MjModel.from_xml_path(str(path))

    data = mujoco.MjData(model)
    mujoco.mj_resetDataKeyframe(model, data, 0)
    lock = threading.Lock()
    service = PF400(model, data, lock)

    thread = threading.Thread(target=run_server, args=(service,), daemon=True)
    thread.start()
    logger.info("sim server started")

    while True:
        step_start = time.time()
        with lock:
            mujoco.mj_step(model, data)
        remaining = model.opt.timestep - (time.time() - step_start)
        if remaining > 0:
            time.sleep(remaining)


class PF400(SimService):
    def __init__(
        self, model: mujoco.MjModel, data: mujoco.MjData, lock: threading.Lock
    ):
        self.model = model
        self.data = data
        self.lock = lock

    async def get_joint(
        self,
        request: GetJointRequest,
        ctx: RequestContext[GetJointRequest, GetJointResponse],
        /,
    ) -> GetJointResponse:
        with self.lock:
            return GetJointResponse(position=self.data.joint(request.joint).qpos[0])

    async def set_actuator(
        self,
        request: SetActuatorRequest,
        ctx: RequestContext[SetActuatorRequest, SetActuatorResponse],
        /,
    ) -> SetActuatorResponse:
        with self.lock:
            self.data.actuator(request.actuator).ctrl = request.value
        return SetActuatorResponse()
