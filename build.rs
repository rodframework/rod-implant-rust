use std::path::PathBuf;

fn main() {
    // The proto is the single source of truth for the wire contract and is
    // referenced, not copied (the same relative reference the .NET implant's
    // csproj carries). rod.proto has no imports, so only its directory is on
    // the include path. Three probe sites keep the one reference working in
    // every layout the crate builds from: the co-located platform layout
    // (../../teamserver/... -- the staging shape the platform's build unit
    // copies into and an extracted release tarball carries), a protos/
    // directory beside the crate (a standalone checkout of this repository,
    // filled by scripts/fetch-rod-proto.sh), and the ROD_PROTO override for
    // anything else.
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:rerun-if-env-changed=ROD_PROTO");
    let proto = match std::env::var_os("ROD_PROTO") {
        Some(path) => PathBuf::from(path),
        None => {
            let co_located = manifest.join("../../teamserver/Rod.Protocol/protos/rod.proto");
            let local = manifest.join("protos/rod.proto");
            if co_located.exists() {
                co_located
            } else if local.exists() {
                local
            } else {
                panic!(
                    "rod.proto not found: put it in protos/ (scripts/fetch-rod-proto.sh), \
                     build from the platform's co-located layout, or set ROD_PROTO"
                );
            }
        }
    };
    println!("cargo:rerun-if-changed={}", proto.display());
    prost_build::Config::new()
        .compile_protos(
            std::slice::from_ref(&proto),
            &[proto.parent().unwrap().to_path_buf()],
        )
        .expect("rod.proto compiles");
}
