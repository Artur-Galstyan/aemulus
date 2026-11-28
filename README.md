# aemulus

A digital twin for lab automation. Write a lab protocol in Python, run it against a
simulated lab in MuJoCo, then run the same protocol against the real devices.

**Status:** early work in progress. Version 0.0.1 holds the package names and is not usable yet.

## Layout

- `python/aemulus/`: the Python package (user API, MuJoCo simulation, device models)
- `crates/`: the Rust device servers
- `proto/`: the Protobuf contracts, shared by Python and Rust

## Development

```bash
task setup   # install the code generator plugins
task gen     # generate Python and Rust code from proto/
```

## STARlet demo

The demo runs a PF400 and STARlet workcell. The PF400 loads a plate from an
off-deck nest onto a STARlet plate carrier, the STARlet transfers liquid from a
reservoir into the plate with a disposable tip and moves the plate with the CO-RE
paddles to a second carrier, and the PF400 returns the plate to the nest. Run it headless:

```bash
uv run python examples/starlet_demo.py
```

On macOS, the MuJoCo viewer needs `mjpython`:

```bash
uv run mjpython examples/starlet_demo.py --view
```

On other platforms, use `uv run python examples/starlet_demo.py --view`.

## License

aemulus is source-available under the [PolyForm Noncommercial License 1.0.0](LICENSE).

- **Free:** noncommercial use, for example by universities, schools, public research organizations, and for personal projects.
- **Commercial use:** needs a separate commercial license. Open an issue on GitHub to ask for one.

Required Notice: Copyright Artur Galstyan (https://github.com/Artur-Galstyan/aemulus)
