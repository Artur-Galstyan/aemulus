import typing
from importlib.resources import files

from aemulus._rust._native import ModelIdentifier
from aemulus.device import Device


@typing.final
class BioShake(Device):
    mjcf = str(files("aemulus") / "assets/bioshake/bioshake.xml")
    device_id = ModelIdentifier.QInstrumentsBioshake3000Elm
