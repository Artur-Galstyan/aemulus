from dataclasses import dataclass
from typing import ClassVar

from aemulus._rust._native import start_sim_server


@dataclass(frozen=True)
class Device:
    device_name: str
    address: str | None = None
    pos: tuple[float, float, float] | None = None

    mjcf: ClassVar[str]
    sim_server_name: ClassVar[str]

    def start_sim_server(self) -> None:
        if self.address is not None:
            raise ValueError("address is not None, sim mode is disabled")

        start_sim_server(self.sim_server_name, self.device_name)
