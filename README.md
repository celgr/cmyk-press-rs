# CMYK Press

CMYK Press is a Rust native After Effects effect for CMYK-style color separation,
fixed registration offset, circular halftone dots, paper color, and source blending.

This implementation is based on `PrintMisregister` and keeps its CPU renderer,
Metal GPU path, deterministic random registration, and dot processing. The default
state follows `docs/requirements.md`: `Default CMYK Dots`, `Composite`, halftone
enabled, circular dots, fixed random registration off, and full quality.

## Effect

- Display name: `CMYK Press`
- Match name: `CMYK Press`
- Category: `Stylize`
- macOS output: `rust/target/release/CMYK Press.plugin`

## Build

On macOS:

```bash
bash ./scripts/build_macos_release.sh
```

The built plug-in is:

```text
rust/target/release/CMYK Press.plugin
```

## Tests

The minimum app-free verification is:

```bash
cd rust
AESDK_ROOT= cargo test
```
