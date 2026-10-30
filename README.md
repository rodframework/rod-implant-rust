# rod-implant-rust

The reference Rust implant for [Rod](https://github.com/rodframework/Rod):
the implant of record for every target a managed runtime cannot serve, built
against the platform's wire contract (`rod.proto`).

This repository is one rung of the platform's repository ladder (the main
repository's architecture.md Sec 15.4). The implant split from the platform
tree, and the pinned source tarball is the whole coupling contract: the
platform's `rod-implants.lock.json` names the release (version and sha256)
its Rust build unit compiles, and its release assembly repacks this tree
beside the wire protos. A checkout of this repository overrides the pin
through `ROD_IMPLANT_RUST_DIR`.

## Layout

The tree is a loose set of crates, not a workspace -- every manifest owns
its own `Cargo.lock` and target dir:

- `Cargo.toml` -- the implant crate (`rod-implant` binary; the `rod_implant`
  library builds `rlib` and `staticlib`, the linkable core an embedding host
  links against)
- `loader/` -- the no_stage loader tier, a small `no_std` fetch stub the
  platform's build unit bakes per request
- `carrier/` -- the Android cdylib the APK packaging tier wraps
- `plugin-sdk/` -- the crate plugin modules build against (`rod-plugin-sdk`)
- `modules/` -- the reference plugin modules (`rod-module-<name>`), the
  worked examples of the plugin seam
- `include/rod_core.h`, `shell/` -- the C-ABI header and the worked C shell
  over the static library

## Building

The crate compiles its wire bindings from the platform's `rod.proto` at
build time (prost in `build.rs`) and needs a system `protoc`. The proto is
referenced, never copied: `build.rs` probes, in order, the `ROD_PROTO`
environment variable, the co-located platform layout
(`../../teamserver/Rod.Protocol/protos` -- the staging shape the platform's
build unit copies into and an extracted release tarball carries), and a
`protos/` directory beside the crate. For a standalone checkout:

```sh
scripts/fetch-rod-proto.sh        # fills protos/ from the platform repo
cargo build --release
```

The platform repository remains the wire contract's home; drift is gated
there -- its conformance harness runs against this repository's pinned
releases.

## Releases

A `v*` tag triggers the release workflow: it smoke-builds the static musl
shape, packs the tag's tree under the `src/implant/rust/` prefix with
deterministic tar flags (sorted names, zeroed mtime and ownership, `gzip
-n`), and attaches the tarball plus its sha256 to the release. The
determinism is the contract's other half: the sha256 the platform's lock
pins is reproducible from the tag on any host.
