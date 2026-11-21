import socket
import threading
import time

import pyqwest
from aemulus.sila2.Pf400_connect import Pf400ClientSync
from aemulus.sila2.Pf400_pb import MoveZ_Parameters
from aemulus.SiLAFramework_pb import Real
from aemulus.simulation import run_simulation
from connectrpc.protocol import ProtocolType


def wait_for_port(port: int, timeout: float = 10.0) -> None:
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        try:
            socket.create_connection(("127.0.0.1", port), timeout=0.2).close()
            return
        except OSError:
            time.sleep(0.05)
    raise TimeoutError(f"port {port} did not open")


def main():
    t = threading.Thread(target=run_simulation, daemon=True)
    t.start()

    wait_for_port(50051)

    http = pyqwest.SyncClient(
        pyqwest.SyncHTTPTransport(http_version=pyqwest.HTTPVersion.HTTP2)
    )
    pf400_client = Pf400ClientSync(
        "http://127.0.0.1:50052",
        http_client=http,
        protocol=ProtocolType.GRPC,
    )
    confirmation = pf400_client.move_z(MoveZ_Parameters(Height=Real(value=0.0)))
    confirmation_uuid = confirmation.commandExecutionUUID
    assert confirmation_uuid is not None

    for info in pf400_client.move_z_info(confirmation_uuid):
        assert info is not None
        assert info.progressInfo is not None
        print(info.commandStatus, info.progressInfo.value)

    result = pf400_client.move_z_result(confirmation_uuid)
    print(result)

    t.join()


if __name__ == "__main__":
    main()
