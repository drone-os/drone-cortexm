use drone_config::{validate_drone_crate_config_flag, Result};

fn main() -> Result<()> {
    validate_drone_crate_config_flag(None)
}
