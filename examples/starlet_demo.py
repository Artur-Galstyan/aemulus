"""Move a plate and transfer liquid on the STARlet deck (pass --view for the viewer)."""

import argparse
import asyncio
import sys
from importlib.resources import files

import mujoco.viewer
import numpy as np
from connectrpc.protocol import ProtocolType
from mujoco import MjData, mj_forward, mj_setState, mjtState
from pyqwest import Client, HTTPTransport, HTTPVersion

from aemulus.devices.starlet import STARlet, TIP_50UL_LENGTH
from aemulus.lab import Labware, create_lab, wait_for_port
from aemulus.sila2.SiLAFramework_pb import ExecutionInfo, Integer, Real
from aemulus.sila2.Starlet_connect import StarletClient
from aemulus.sila2.Starlet_pb import (
    MoveEject_Parameters,
    MovePlunger_Parameters,
    MoveX_Parameters,
    MoveY_Parameters,
    MoveZ_Parameters,
)


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


async def move_plunger(client, position, channel):
    confirmation = await client.move_plunger(
        MovePlunger_Parameters(Channel=Integer(value=channel), Position=Real(value=position))
    )
    await wait(confirmation, client.move_plunger_info, client.move_plunger_result)


async def move_eject(client, position, channel):
    confirmation = await client.move_eject(
        MoveEject_Parameters(Channel=Integer(value=channel), Position=Real(value=position))
    )
    await wait(confirmation, client.move_eject_info, client.move_eject_result)


async def main(view=False):
    if view and sys.platform == "darwin" and mujoco.viewer._MJPYTHON is None:
        raise SystemExit(
            "On macOS use: uv run mjpython examples/starlet_demo.py --view"
        )
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
                "destination",
                str(assets / "nunc_96_plate/nunc_96_plate.xml"),
                (0.22, -0.20, 0.20),
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
    starlet.set_liquid("plate", 50.0, "A1")
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

        # Preserve channel order while channel 5 travels between the racks and plates.
        await asyncio.gather(
            *(move_y(client, 0.18 - channel * 0.02, channel) for channel in range(5)),
            move_y(client, -0.25, 6),
            move_y(client, -0.27, 7),
        )
        # Channel 5 picks up a 50 uL tip at A1. XYZ describes the nozzle,
        # so the tip collar meets the nozzle at the rack top.
        x, y = lab.well_xy("tips_50", "A1")
        await move_x(client, x)
        await move_y(client, y, 5)
        await move_z(client, lab.labware_pos("tips_50")[2] + TIP_50UL_LENGTH, 5)
        await move_z(client, 0.35, 5)
        if starlet.attached_tip(5) is None:
            raise RuntimeError("Channel 5 did not pick up the A1 tip")

        # Aspirate 20 uL from the moved plate. The 50.4 mm tip ends
        # 10 mm above the plate bottom, inside the A1 well.
        x, y = lab.well_xy("plate", "A1")
        await move_x(client, x)
        await move_y(client, y, 5)
        await move_z(client, lab.labware_pos("plate")[2] + 0.010 + TIP_50UL_LENGTH, 5)
        await move_plunger(client, 0.020, 5)
        await asyncio.sleep(0.5)
        if not np.isclose(starlet.channel_volume_ul(5), 20.0, atol=1.0):
            raise RuntimeError("Channel 5 did not aspirate 20 uL")
        await move_z(client, 0.35, 5)

        x, y = lab.well_xy("destination", "A1")
        await move_x(client, x)
        await move_y(client, y, 5)
        await move_z(
            client, lab.labware_pos("destination")[2] + 0.010 + TIP_50UL_LENGTH, 5
        )
        await move_plunger(client, 0.0, 5)
        await asyncio.sleep(0.5)
        await move_z(client, 0.35, 5)

        # Check conservation before ejection discards any residual in the tip.
        total = (
            starlet.volume_ul("plate", "A1")
            + starlet.volume_ul("destination", "A1")
            + starlet.channel_volume_ul(5)
        )
        if not np.isclose(total, 50.0, rtol=0.0, atol=1e-6):
            raise RuntimeError(f"Liquid was lost during transfer: {total} uL")

        # Eject onto an empty part of the deck, clear of both plates.
        await move_x(client, -0.30)
        await move_y(client, -0.10, 5)
        await move_eject(client, 0.010, 5)
        await move_eject(client, 0.0, 5)
        await asyncio.sleep(0.5)

    data = MjData(lab.model)
    mj_setState(lab.model, data, lab.sim.state, mjtState.mjSTATE_FULLPHYSICS)
    mj_forward(lab.model, data)
    position = data.body("plate/nunc_96_plate").xpos.copy()
    if not np.allclose(position, (0.05, -0.20, 0.20), atol=0.005):
        raise RuntimeError(f"Plate did not reach the destination: {position}")
    print(f"Plate moved successfully to {position.round(4).tolist()} m")
    volumes = (starlet.volume_ul("plate", "A1"), starlet.volume_ul("destination", "A1"))
    # The 1 mm plunger arrival tolerance corresponds to 1 uL.
    if not np.allclose(volumes, (30.0, 20.0), atol=1.0):
        raise RuntimeError(f"Liquid did not reach the destination: {volumes} uL")
    if starlet.attached_tip(5) is not None:
        raise RuntimeError("Channel 5 did not eject its tip")
    if not np.isclose(starlet.channel_volume_ul(5), 0.0, atol=0.1):
        raise RuntimeError("Liquid remains in channel 5")
    print(f"Transferred 20 uL from plate A1 to destination A1: {np.round(volumes, 2)} uL")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--view", action="store_true", help="Show the MuJoCo viewer")
    asyncio.run(main(parser.parse_args().view))
