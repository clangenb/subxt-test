//! Runtime api created by the macro.
//!
//! Output can be checked with: `subxt codegen -f metadata/encointer-rococo.scale | rustfmt --edition=2021 --emit=stdout`

#[subxt::subxt(runtime_metadata_path = "metadata/encointer-rococo.scale")]
pub mod encointer_rococo_runtime { }