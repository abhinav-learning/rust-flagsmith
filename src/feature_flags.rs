use crate::env_config::EnvConfig;
use flagsmith::{Flag, Flagsmith, FlagsmithOptions};

pub fn fetch_feature_flag(flag_name: &str) -> Flag {
    let env: EnvConfig = EnvConfig::from_env();
    let options = FlagsmithOptions {
        ..Default::default()
    };
    let flagsmith = Flagsmith::new(env.flagsmith_environment_key, options);
    let flags = flagsmith.get_environment_flags().unwrap();
    flags.get_flag(flag_name).unwrap_or_default()
}
