use schema_rust::build::ContractCrateBuild;

fn main() {
    ContractCrateBuild::from_environment(
        "signal-router",
        "0.3.0",
        "SIGNAL_ROUTER_UPDATE_SCHEMA_ARTIFACTS",
    )
    .expect_fresh();
}
