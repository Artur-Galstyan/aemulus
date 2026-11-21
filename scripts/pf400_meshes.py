# /// script
# requires-python = "==3.12.*"
# dependencies = ["build123d"]
# ///
"""Make the PF400 meshes. Run: uv run scripts/pf400_meshes.py

Shapes follow the Brooks datasheet 661423 Rev A drawings and product photos, not vendor CAD.
Units are mm. Each part is drawn in the frame of its MuJoCo body.
"""

from pathlib import Path

from build123d import (
    Align,
    Axis,
    Box,
    Circle,
    Cylinder,
    Location,
    Plane,
    Pos,
    Rectangle,
    RectangleRounded,
    export_stl,
    extrude,
    fillet,
    make_hull,
)

OUT = Path(__file__).parent.parent / "python/aemulus/assets/pf400/meshes"
BOTTOM = (Align.CENTER, Align.CENTER, Align.MIN)


def slab(sketch, z_low, height, edge_radius):
    part = extrude(Plane.XY.offset(z_low) * sketch, height)
    return fillet(part.edges().filter_by(Axis.Z, reverse=True), edge_radius)


def link(length, r_start, r_end, z_low, height):
    outline = make_hull((Circle(r_start) + Pos(length, 0) * Circle(r_end)).edges())
    return slab(outline, z_low, height, 6)


def base_plate():
    return slab(RectangleRounded(201, 235, 12), 0, 12, 2)


def column():
    part = Pos(-50.5, 0, 12) * Box(90, 130, 667, align=BOTTOM)
    part = fillet(part.edges().filter_by(Axis.Z), 22)
    return fillet(part.edges().group_by(Axis.Z)[-1], 8)


def carriage():
    part = Pos(-30, 0, -35) * Box(50, 70, 70, align=BOTTOM)
    return fillet(part.edges().filter_by(Axis.Z), 8)


def inner_link():
    hub = Pos(0, 0, -30) * Cylinder(50, 90, align=BOTTOM)
    hub = fillet(hub.edges().group_by(Axis.Z)[-1], 10)
    return hub + link(225, 50, 42, -30, 50)


def outer_link():
    return link(210, 42, 36, -40, 40)


def gripper():
    # Datasheet: 134 wide, 115.5 long, 54 deep front block, round rear. J4 is the rear circle center.
    front = Pos(48.5, 0) * Rectangle(54, 134)
    outline = make_hull((Circle(40) + front).edges())
    return slab(outline, -30, 30, 4)


def finger():
    part = Box(20, 6, 30)
    return fillet(part.edges().filter_by(Axis.Y), 3)


if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    for make in (base_plate, column, carriage, inner_link, outer_link, gripper, finger):
        export_stl(make(), OUT / f"{make.__name__}.stl", tolerance=0.1, angular_tolerance=0.25)
        print(make.__name__)
