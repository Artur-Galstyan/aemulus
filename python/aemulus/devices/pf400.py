import typing
from importlib.resources import files

from aemulus._rust._native import ModelIdentifier
from aemulus.device import Device


@typing.final
class PF400(Device):
    mjcf = str(files("aemulus") / "assets/pf400/pf400.xml")
    device_id = ModelIdentifier.PrecisePf400
