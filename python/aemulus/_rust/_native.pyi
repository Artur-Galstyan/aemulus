from typing import ClassVar, final

@final
class ModelIdentifier:
    PrecisePf400: ClassVar[ModelIdentifier]
    QInstrumentsBioshake3000Elm: ClassVar[ModelIdentifier]

def start_sim_server(device_name: str, model: ModelIdentifier, port: int) -> None: ...
