import typing
from importlib.resources import files

from aemulus.device import Device


@typing.final
class STARlet(Device):
    mjcf = str(files("aemulus") / "assets/hamilton_starlet/starlet.xml")
    sim_server_name = "starlet"
