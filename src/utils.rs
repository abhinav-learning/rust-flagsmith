use crate::feature_flags::fetch_feature_flag;

pub fn get_configured_rate_limit() -> i64 {
    let rate_limit = fetch_feature_flag("rate_limit").value_as_i64();
    info!("Current Rate Limit is {:?}", rate_limit);
    return rate_limit.unwrap();
}

pub fn get_beta() -> bool {
    let beta = fetch_feature_flag("beta").value_as_bool();
    info!("Current Value for beta feature flag is {:?}", beta);
    return beta.unwrap();
}
