use schema_rust::build::ContractCrateBuild;

fn main() {
    ContractCrateBuild::from_environment(
        "signal-router",
        "0.4.1",
        "SIGNAL_ROUTER_UPDATE_SCHEMA_ARTIFACTS",
    )
    .expect_fresh();
}
