use actix_governor::{
    governor::middleware::StateInformationMiddleware, GovernorConfig, GovernorConfigBuilder,
    PeerIpKeyExtractor,
};

/// 100 RPS - 100 requests per second
#[allow(clippy::missing_panics_doc)]
pub fn create_default_rate_limit() -> GovernorConfig<PeerIpKeyExtractor, StateInformationMiddleware>
{
    GovernorConfigBuilder::default()
        .per_second(1)
        .burst_size(100)
        .use_headers()
        .finish()
        .expect("Failed to create default rate limit config")
}

/// 10 RPH - 10 requests per hour
#[allow(clippy::missing_panics_doc)]
pub fn create_strict_rate_limit() -> GovernorConfig<PeerIpKeyExtractor, StateInformationMiddleware>
{
    GovernorConfigBuilder::default()
        .per_second(360)
        .burst_size(10)
        .use_headers()
        .finish()
        .expect("Failed to create strict rate limit config")
}
