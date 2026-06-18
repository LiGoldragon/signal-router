use std::{env, path::PathBuf};

use schema_rust_next::build::{
    CargoSchemaMetadata, DependencySchema, GenerationDriver, GenerationPlan,
};

/// The signal-router build lowers `schema/lib.schema` to `src/schema/lib.rs`
/// through the wire-contract emission target. The schema imports the
/// cross-component authorized-object vocabulary from `signal-standard` (the
/// shared standards library, Spirit eeeo); the `ImportResolver` finds that
/// dependency schema through `signal-standard`'s links-crate metadata
/// (`DEP_SIGNAL_STANDARD_SCHEMA_DIR`), so the generated module emits a
/// `use signal_standard::schema::lib::Type as LocalName;` for each import.
struct SchemaBuild {
    crate_root: PathBuf,
}

impl SchemaBuild {
    fn from_environment() -> Self {
        Self {
            crate_root: PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir set")),
        }
    }

    fn run(&self) {
        println!("cargo:rerun-if-changed=schema/lib.schema");
        println!("cargo:rerun-if-changed=src/schema/lib.rs");
        println!("cargo:rerun-if-env-changed=DEP_SIGNAL_STANDARD_SCHEMA_DIR");
        CargoSchemaMetadata::new("signal-router").emit_schema_directory(&self.crate_root);

        let standard =
            DependencySchema::from_cargo_metadata("signal-standard", "signal-standard", "0.1.0")
                .expect("read signal-standard schema metadata")
                .expect(
                    "signal-standard schema directory exposed via DEP_SIGNAL_STANDARD_SCHEMA_DIR",
                );

        GenerationDriver::new(
            GenerationPlan::wire_contract(&self.crate_root, "signal-router", "0.2.0")
                .with_dependency_schema(standard),
        )
        .generate()
        .expect("generate signal-router schema artifacts")
        .write_or_check("SIGNAL_ROUTER_UPDATE_SCHEMA_ARTIFACTS")
        .expect("checked-in signal-router schema artifacts are fresh");
    }
}

fn main() {
    SchemaBuild::from_environment().run();
}
