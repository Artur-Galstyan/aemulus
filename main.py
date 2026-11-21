import socket
import threading
import time

import pyqwest
from aemulus.pf400.v1.pf400_connect import Pf400ServiceClientSync
from aemulus.pf400.v1.pf400_pb import MoveZRequest, ReadZRequest
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
    t = threading.Thread(target=run_simulation)
    t.start()

    wait_for_port(50051)

    http = pyqwest.SyncClient(
        pyqwest.SyncHTTPTransport(http_version=pyqwest.HTTPVersion.HTTP2)
    )
    pf400_client = Pf400ServiceClientSync(
        "http://127.0.0.1:50052",
        http_client=http,
        protocol=ProtocolType.GRPC,
    )
    pf400_client.move_z(MoveZRequest(height=0.0))

    time.sleep(2)

    z = pf400_client.read_z(ReadZRequest())
    print(z)

    t.join()


if __name__ == "__main__":
    main()
