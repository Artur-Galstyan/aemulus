# /// script
# requires-python = "==3.12.*"
# dependencies = ["build123d"]
# ///
"""Make the BioShake 3000 elm meshes. Run: uv run scripts/bioshake_meshes.py

Shapes are simple and follow the dimensions of the QInstruments datasheet, not vendor CAD.
Units are mm. Each part is drawn in the frame of its MuJoCo body.
"""

from pathlib import Path

from build123d import Align, Axis, Box, Circle, Cone, Cylinder, Pos, RectangleRounded, export_stl, extrude, fillet, make_hull

OUT = Path(__file__).parent.parent / "python/aemulus/assets/bioshake/meshes"
BOTTOM = (Align.CENTER, Align.CENTER, Align.MIN)

FEET = 1.4
HOUSING = 34.7  # height of the housing without the feet
PLATE = 13.5  # height of the shaker plate


def housing():
    part = Pos(0, 0, FEET) * Box(142, 99, HOUSING, align=BOTTOM)
    part = fillet(part.edges().filter_by(Axis.Z), 4)
    part = fillet(part.edges().group_by(Axis.Z)[-1], 1)
    for x in (-63, 63):
        for y in (-42, 42):
            part += Pos(x, y, 0) * Cylinder(5, FEET, align=BOTTOM)
    return part


def plate():
    neck = extrude(RectangleRounded(124, 83, 6), 1.5)
    top = Pos(0, 0, 1.5) * extrude(RectangleRounded(140, 95, 6), PLATE - 1.5)
    return neck + fillet(top.edges().group_by(Axis.Z)[-1], 0.8)


def clamp():
    # One ELM pin on its lever. Origin: the pin axis, on the top of the plate. The lever points to +x.
    lever = extrude(Pos(0, 0, -0.5) * make_hull((Circle(2) + Pos(14, 0) * Circle(3)).edges()), 1)
    pivot = Pos(14, 0, 0.5) * Cylinder(1.8, 0.8, align=BOTTOM)
    return Cone(1.0, 1.5, 7.15, align=BOTTOM) + lever + pivot


if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    for make in (housing, plate, clamp):
        export_stl(make(), OUT / f"{make.__name__}.stl", tolerance=0.1, angular_tolerance=0.25)
        print(make.__name__)
