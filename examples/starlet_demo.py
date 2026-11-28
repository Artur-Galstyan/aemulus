"""Run a PF400 and STARlet workcell: load, pipette, move and unload a plate (pass --view for the viewer)."""

import argparse
import asyncio
import sys
from importlib.resources import files

import mujoco.viewer
import numpy as np
from connectrpc.protocol import ProtocolType
from pyqwest import Client, HTTPTransport, HTTPVersion

from aemulus.devices.pf400 import PF400
from aemulus.devices.starlet import STARlet, TIP_50UL_LENGTH
from aemulus.lab import Labware, create_lab, wait_for_port
from aemulus.sila2.Pf400_connect import Pf400Client
from aemulus.sila2.Pf400_pb import (
    MoveElbow_Parameters,
    MoveGripper_Parameters,
    MoveShoulder_Parameters,
    MoveWrist_Parameters,
)
from aemulus.sila2.Pf400_pb import MoveZ_Parameters as Pf400MoveZ_Parameters
from aemulus.sila2.SiLAFramework_pb import ExecutionInfo, Integer, Real
from aemulus.sila2.Starlet_connect import StarletClient
from aemulus.sila2.Starlet_pb import (
    MoveEject_Parameters,
    MovePlunger_Parameters,
    MoveX_Parameters,
    MoveY_Parameters,
    MoveZ_Parameters,
)

# Plate carrier site 0 (front) and tip carrier site 0 in the world frame, from the carrier MJCF.
SITE_0_Y = -0.283
PLATE_SITE_Z = 0.28615
TIP_SITE_Z = 0.31495
# The off-deck nest sits in front of the STARlet, reached by the PF400.
NEST = (0.10, -0.70, 0.0)
NEST_Z = 0.280
# PF400 joint poses (shoulder, elbow, wrist) in rad. The grip poses add the Z height that puts
# the TCP 16 mm above the plate bottom, so the 30 mm fingers clear the carrier site.
# The wrist angles keep the gripper along +x, so the fingers hold the long sides of the plate.
PF400_HOME = (0.0, np.pi, np.pi)
PF400_SAFE_Z = 0.3332
NEST_POSE = (0.2420, -1.4879, 1.9532, 5.8179)
SITE_A_POSE = (0.24815, 0.8629, 1.6931, 3.7272)
SITE_B_POSE = (0.24815, 0.3600, 1.0640, 4.8592)


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


async def move_height(client, position):
    confirmation = await client.move_z(Pf400MoveZ_Parameters(Height=Real(value=position)))
    await wait(confirmation, client.move_z_info, client.move_z_result)


async def move_shoulder(client, angle):
    confirmation = await client.move_shoulder(
        MoveShoulder_Parameters(Angle=Real(value=angle))
    )
    await wait(confirmation, client.move_shoulder_info, client.move_shoulder_result)


async def move_elbow(client, angle):
    confirmation = await client.move_elbow(MoveElbow_Parameters(Angle=Real(value=angle)))
    await wait(confirmation, client.move_elbow_info, client.move_elbow_result)


async def move_wrist(client, angle):
    confirmation = await client.move_wrist(MoveWrist_Parameters(Angle=Real(value=angle)))
    await wait(confirmation, client.move_wrist_info, client.move_wrist_result)


async def move_gripper(client, position):
    confirmation = await client.move_gripper(
        MoveGripper_Parameters(Position=Real(value=position))
    )
    await wait(confirmation, client.move_gripper_info, client.move_gripper_result)


async def pf400_move(client, start, end):
    # Move the three rotary joints together to keep the plate landscape.
    # Small angle increments limit acceleration while the fingers grip the plate.
    for fraction in np.linspace(0, 1, 21)[1:]:
        shoulder, elbow, wrist = np.array(start) + fraction * (np.array(end) - start)
        await asyncio.gather(
            move_shoulder(client, shoulder),
            move_elbow(client, elbow),
            move_wrist(client, wrist),
        )


async def transfer(client, start, end):
    # Poses contain (Z, shoulder, elbow, wrist). The arm starts above start.
    # The fingers hold the plate by friction.
    await move_height(client, start[0])
    await move_gripper(client, 0.0019)
    await move_height(client, PF400_SAFE_Z)
    await pf400_move(client, start[1:], end[1:])
    await move_height(client, end[0])
    await asyncio.sleep(0.5)
    await move_gripper(client, 0.030)
    await move_height(client, PF400_SAFE_Z)


def check_plate(lab, expected, place):
    position = lab.labware_pos("plate")
    if not np.allclose(position, expected, atol=0.005):
        raise RuntimeError(f"Plate did not reach {place}: {position}")
    print(f"Plate reached {place} at {position.round(4).tolist()} m")


async def main(view=False):
    if view and sys.platform == "darwin" and mujoco.viewer._MJPYTHON is None:
        raise SystemExit(
            "On macOS use: uv run mjpython examples/starlet_demo.py --view"
        )
    assets = files("aemulus") / "assets"
    starlet = STARlet(device_name="starlet", pos=(0, 0, 0))
    pf400 = PF400(device_name="pf400", pos=(-0.22, -0.57, 0))
    # Carriers stand on the deck tracks, front edge on the deck front (y = -0.3345 m).
    # Track n has its left edge at x = -0.3375 + (n - 1) * 0.0225 m; each carrier is 6 tracks wide.
    # The PF400 palm reaches 108 mm to the left of a plate it grips, so the tip carrier
    # stands right of both plate carriers, and the reservoir stays below the palm.
    reservoir_carrier = (-0.270, -0.086, 0.20)  # tracks 1 to 6
    carrier_a = (-0.135, -0.086, 0.20)  # tracks 7 to 12
    carrier_b = (0.135, -0.086, 0.20)  # tracks 19 to 24
    tip_carrier = (0.270, -0.086, 0.20)  # tracks 25 to 30
    lab = create_lab(
        [starlet, pf400],
        labware=[
            Labware("tip_carrier", str(assets / "hamilton_carriers/tip_car_480.xml"), tip_carrier),
            Labware("carrier_a", str(assets / "hamilton_carriers/plt_car_l5ac.xml"), carrier_a),
            Labware(
                "reservoir_carrier",
                str(assets / "hamilton_carriers/plt_car_l5ac.xml"),
                reservoir_carrier,
            ),
            Labware("carrier_b", str(assets / "hamilton_carriers/plt_car_l5ac.xml"), carrier_b),
            Labware("nest", str(assets / "plate_nest/plate_nest.xml"), NEST),
            Labware(
                "plate",
                str(assets / "nunc_96_plate/nunc_96_plate.xml"),
                (NEST[0], NEST[1], NEST_Z),
                free=True,
            ),
            Labware(
                "tips_50",
                str(assets / "hamilton_tip_rack/tip_rack_50ul.xml"),
                (tip_carrier[0], SITE_0_Y, TIP_SITE_Z),
            ),
            Labware(
                "reservoir",
                str(assets / "reservoir/reservoir.xml"),
                (reservoir_carrier[0], SITE_0_Y, PLATE_SITE_Z),
            ),
        ],
    )
    starlet.set_liquid("reservoir", 1000.0)
    if view:
        lab.view()
    wait_for_port(50052)
    wait_for_port(50053)
    async with (
        HTTPTransport(http_version=HTTPVersion.HTTP2) as transport,
        StarletClient(
            "http://127.0.0.1:50053",
            protocol=ProtocolType.GRPC,
            timeout_ms=20000,
            http_client=Client(transport),
        ) as client,
        Pf400Client(
            "http://127.0.0.1:50052",
            protocol=ProtocolType.GRPC,
            timeout_ms=20000,
            http_client=Client(transport),
        ) as arm,
    ):
        # Step 1: the PF400 loads the plate from the nest onto carrier A, site 0.
        await move_gripper(arm, 0.030)
        await move_height(arm, PF400_SAFE_Z)
        await pf400_move(arm, PF400_HOME, NEST_POSE[1:])
        await transfer(arm, NEST_POSE, SITE_A_POSE)
        # Park above the nest, clear of the deck.
        await pf400_move(arm, SITE_A_POSE[1:], NEST_POSE[1:])
        check_plate(lab, (carrier_a[0], SITE_0_Y, PLATE_SITE_Z), "carrier A")

        # Step 2: channel 5 transfers 20 uL from the reservoir to plate A1.
        # Channels 6 and 7 wait in front of it, with their paddles above the labware.
        await asyncio.gather(
            *(move_y(client, 0.10 - channel * 0.02, channel) for channel in range(5)),
            move_y(client, -0.305, 6),
            move_y(client, -0.325, 7),
        )
        # XYZ describes the nozzle. The tip collar tops are 28 mm above the rack bottom.
        x, y = lab.well_xy("tips_50", "A1")
        await move_x(client, x)
        await move_y(client, y, 5)
        await move_z(client, lab.labware_pos("tips_50")[2] + 0.028, 5)
        await move_z(client, 0.45, 5)
        if starlet.attached_tip(5) is None:
            raise RuntimeError("Channel 5 did not pick up the A1 tip")

        # The 50.4 mm tip ends 5 mm above the reservoir floor.
        x, y = lab.well_xy("reservoir")
        await move_x(client, x)
        await move_y(client, y, 5)
        await move_z(client, lab.labware_pos("reservoir")[2] + 0.005 + TIP_50UL_LENGTH, 5)
        await move_plunger(client, 0.020, 5)
        await asyncio.sleep(0.5)
        if not np.isclose(starlet.channel_volume_ul(5), 20.0, atol=1.0):
            raise RuntimeError("Channel 5 did not aspirate 20 uL")
        await move_z(client, 0.45, 5)

        x, y = lab.well_xy("plate", "A1")
        await move_x(client, x)
        await move_y(client, y, 5)
        await move_z(client, lab.labware_pos("plate")[2] + 0.010 + TIP_50UL_LENGTH, 5)
        await move_plunger(client, 0.0, 5)
        await asyncio.sleep(0.5)
        await move_z(client, 0.45, 5)

        # Check conservation before ejection discards any residual in the tip.
        total = (
            starlet.volume_ul("reservoir")
            + starlet.volume_ul("plate", "A1")
            + starlet.channel_volume_ul(5)
        )
        if not np.isclose(total, 1000.0, rtol=0.0, atol=1e-6):
            raise RuntimeError(f"Liquid was lost during transfer: {total} uL")

        # Eject onto the free tracks 13 to 18.
        await move_x(client, 0.0)
        await move_y(client, -0.10, 5)
        await move_eject(client, 0.010, 5)
        await move_eject(client, 0.0, 5)
        await asyncio.sleep(0.5)
        if starlet.attached_tip(5) is not None:
            raise RuntimeError("Channel 5 did not eject its tip")
        if not np.isclose(starlet.channel_volume_ul(5), 0.0, atol=0.1):
            raise RuntimeError("Liquid remains in channel 5")
        volume = starlet.volume_ul("plate", "A1")
        # The 1 mm plunger arrival tolerance corresponds to 1 uL.
        if not np.isclose(volume, 20.0, atol=1.0):
            raise RuntimeError(f"Liquid did not reach plate A1: {volume} uL")
        print(f"Transferred 20 uL from the reservoir to plate A1: {volume:.2f} uL")
        check_plate(lab, (carrier_a[0], SITE_0_Y, PLATE_SITE_Z), "carrier A")

        # Step 3: the CO-RE paddles move the plate from carrier A to carrier B.
        # Channels 6 and 7 carry the paddles. Their Y positions are
        # the inner gripping faces, and their Z positions are the paddle bottoms.
        await asyncio.gather(move_y(client, SITE_0_Y + 0.055, 6), move_y(client, -0.333, 7))
        await move_x(client, carrier_a[0])
        await asyncio.gather(
            move_z(client, PLATE_SITE_Z + 0.002, 6), move_z(client, PLATE_SITE_Z + 0.002, 7)
        )
        await asyncio.gather(
            move_y(client, SITE_0_Y + 0.042, 6), move_y(client, SITE_0_Y - 0.042, 7)
        )
        await asyncio.gather(move_z(client, 0.40, 6), move_z(client, 0.40, 7))
        await move_x(client, carrier_b[0])
        await asyncio.gather(
            move_z(client, PLATE_SITE_Z + 0.012, 6), move_z(client, PLATE_SITE_Z + 0.012, 7)
        )
        await asyncio.gather(move_y(client, SITE_0_Y + 0.055, 6), move_y(client, -0.333, 7))
        await asyncio.gather(move_z(client, 0.45, 6), move_z(client, 0.45, 7))
        # Clear carrier B for the PF400.
        await move_x(client, reservoir_carrier[0])
        await asyncio.sleep(0.5)
        check_plate(lab, (carrier_b[0], SITE_0_Y, PLATE_SITE_Z), "carrier B")

        # Step 4: the PF400 returns the plate from carrier B, site 0, to the nest.
        await pf400_move(arm, NEST_POSE[1:], SITE_B_POSE[1:])
        await transfer(arm, SITE_B_POSE, NEST_POSE)
        check_plate(lab, (NEST[0], NEST[1], NEST_Z), "the nest")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--view", action="store_true", help="Show the MuJoCo viewer")
    asyncio.run(main(parser.parse_args().view))
