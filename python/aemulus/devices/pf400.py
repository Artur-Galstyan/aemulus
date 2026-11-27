import typing
from importlib.resources import files

from aemulus.device import Device
from aemulus.sila2.Pf400_connect import Pf400


@typing.final
class PF400(Device):
    mjcf = str(files("aemulus") / "assets/pf400/pf400.xml")
    device_id = Pf400.desc().type_name
