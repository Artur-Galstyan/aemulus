from aemulus.devices.bioshake import BioShake
from aemulus.devices.pf400 import PF400
from aemulus.lab import create_lab


def main():
    lab = create_lab(
        devices=[
            PF400(pos=(0, 0, 0), device_name="pf400", port=50052),
            BioShake(pos=(0.25, 0.25, 0), device_name="bioshake", port=50053),
        ]
    )
    # lab.keep_alive()
    # lab.view()
    lab.keep_alive()


if __name__ == "__main__":
    main()
