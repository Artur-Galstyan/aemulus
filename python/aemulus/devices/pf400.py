import typing
from importlib.resources import files

from aemulus.device import Device


@typing.final
class PF400(Device):
    mjcf = str(files("aemulus") / "assets/pf400/pf400.xml")
    sim_server_name = "pf400"
