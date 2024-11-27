use flagsmith::{Flag, Flagsmith, FlagsmithOptions};
use std::env;

pub fn fetch_feature_flag(flag_name: &str) -> Flag {
    let options = FlagsmithOptions {
        ..Default::default()
    };
    let flagsmith = Flagsmith::new(
        env::var("FLAGSMITH_SERVER_SIDE_ENVIRONMENT_KEY")
            .expect("FLAGSMITH_SERVER_SIDE_ENVIRONMENT_KEY not found in environment"),
        options,
    );
    let flags = flagsmith.get_environment_flags().unwrap();
    flags.get_flag(flag_name).unwrap_or_default()
}
