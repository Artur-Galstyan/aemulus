import typing
from importlib.resources import files

from aemulus.device import Device
from aemulus.sila2.Bioshake_connect import Bioshake


@typing.final
class BioShake(Device):
    mjcf = str(files("aemulus") / "assets/bioshake/bioshake.xml")
    device_id = Bioshake.desc().type_name
