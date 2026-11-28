from dataclasses import dataclass

import numpy as np
from jaxtyping import Float


@dataclass(frozen=True)
class Labware:
    name: str
    mjcf: str
    pos: tuple[float, float, float]
    free: bool = False


from typing import NamedTuple


class Sbs96Offset(NamedTuple):
    dx: float
    dy: float


class WellXYPosition(NamedTuple):
    x: float
    y: float


def sbs_96_offset(well: str) -> Sbs96Offset:
    if len(well) < 2:
        raise ValueError(f"Invalid 96-well name: {well!r}")
    row = ord(well[0].upper()) - ord("A")
    try:
        col = int(well[1:]) - 1
    except ValueError as err:
        raise ValueError(f"Invalid 96-well name: {well!r}") from err
    if not (0 <= row < 8 and 0 <= col < 12):
        raise ValueError(f"Invalid 96-well name: {well!r}")
    return Sbs96Offset(-0.0495 + col * 0.009, 0.0315 - row * 0.009)


def well_xy(labware_pos: Float[np.ndarray, "3"], well: str) -> WellXYPosition:
    offset = sbs_96_offset(well)
    return WellXYPosition(
        float(labware_pos[0] + offset.dx), float(labware_pos[1] + offset.dy)
    )
