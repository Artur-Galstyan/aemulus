"""Move a plate on the STARlet deck, headless by default (pass --view for the viewer)."""

import argparse
import asyncio
from importlib.resources import files

import numpy as np
from connectrpc.protocol import ProtocolType
from mujoco import MjData, mj_forward, mj_setState, mjtState
from pyqwest import Client, HTTPTransport, HTTPVersion

from aemulus.devices.starlet import STARlet
from aemulus.lab import Labware, create_lab, wait_for_port
from aemulus.sila2.SiLAFramework_pb import ExecutionInfo, Integer, Real
from aemulus.sila2.Starlet_connect import StarletClient
from aemulus.sila2.Starlet_pb import MoveX_Parameters, MoveY_Parameters, MoveZ_Parameters


async def wait(confirmation, info, result):
    command = confirmation.commandExecutionUUID
    if command is None:
        raise RuntimeError("Missing command execution UUID")
    async with asyncio.timeout(20):
        async for status in info(command):
            if status.commandStatus == ExecutionInfo.CommandStatus.finishedWithError:
                raise RuntimeError("Move failed")
            if status.commandStatus == ExecutionInfo.CommandStatus.finishedSuccessfully:
                await result(command)
                return
        raise RuntimeError("Move ended without a result")


async def move_x(client, position):
    confirmation = await client.move_x(MoveX_Parameters(Position=Real(value=position)))
    await wait(confirmation, client.move_x_info, client.move_x_result)


async def move_y(client, position, channel):
    confirmation = await client.move_y(
        MoveY_Parameters(Channel=Integer(value=channel), Position=Real(value=position))
    )
    await wait(confirmation, client.move_y_info, client.move_y_result)


async def move_z(client, position, channel):
    confirmation = await client.move_z(
        MoveZ_Parameters(Channel=Integer(value=channel), Position=Real(value=position))
    )
    await wait(confirmation, client.move_z_info, client.move_z_result)


async def main(view=False):
    assets = files("aemulus") / "assets"
    starlet = STARlet(device_name="starlet", pos=(0, 0, 0))
    lab = create_lab(
        [starlet],
        labware=[
            Labware(
                "plate",
                str(assets / "nunc_96_plate/nunc_96_plate.xml"),
                (-0.20, -0.20, 0.20),
                free=True,
            ),
            Labware(
                "tips_50",
                str(assets / "hamilton_tip_rack/tip_rack_50ul.xml"),
                (-0.20, 0.04, 0.20),
            ),
            Labware(
                "tips_300",
                str(assets / "hamilton_tip_rack/tip_rack_300ul.xml"),
                (0.0, 0.04, 0.20),
            ),
            Labware(
                "reservoir",
                str(assets / "reservoir/reservoir.xml"),
                (0.20, 0.04, 0.20),
            ),
        ],
    )
    if view:
        lab.view()
    wait_for_port(50053)
    async with (
        HTTPTransport(http_version=HTTPVersion.HTTP2) as transport,
        StarletClient(
            "http://127.0.0.1:50053",
            protocol=ProtocolType.GRPC,
            timeout_ms=20000,
            http_client=Client(transport),
        ) as client,
    ):
        # Channels 6 and 7 carry the CO-RE paddles. Their Y positions are
        # the inner gripping faces, and their Z positions are the paddle bottoms.
        await asyncio.gather(move_y(client, -0.145, 6), move_y(client, -0.255, 7))
        await move_x(client, -0.20)
        await asyncio.gather(move_z(client, 0.202, 6), move_z(client, 0.202, 7))
        await asyncio.gather(move_y(client, -0.158, 6), move_y(client, -0.242, 7))
        await asyncio.gather(move_z(client, 0.30, 6), move_z(client, 0.30, 7))
        await move_x(client, 0.05)
        await asyncio.gather(move_z(client, 0.212, 6), move_z(client, 0.212, 7))
        await asyncio.gather(move_y(client, -0.145, 6), move_y(client, -0.255, 7))
        await asyncio.gather(move_z(client, 0.35, 6), move_z(client, 0.35, 7))
        await asyncio.sleep(0.5)

    data = MjData(lab.model)
    mj_setState(lab.model, data, lab.sim.state, mjtState.mjSTATE_FULLPHYSICS)
    mj_forward(lab.model, data)
    position = data.body("plate/nunc_96_plate").xpos.copy()
    if not np.allclose(position, (0.05, -0.20, 0.20), atol=0.005):
        raise RuntimeError(f"Plate did not reach the destination: {position}")
    print(f"Plate moved successfully to {position.round(4).tolist()} m")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--view", action="store_true", help="Show the MuJoCo viewer")
    asyncio.run(main(parser.parse_args().view))
