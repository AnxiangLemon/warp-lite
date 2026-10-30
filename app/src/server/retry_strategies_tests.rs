use std::cell::Cell;

use super::*;

#[test]
fn lite_disabled_service_is_permanent_through_context() {
    for error in [
        anyhow::Error::new(crate::server::CloudServicesUnavailable),
        anyhow::Error::new(GraphQLError::Unavailable),
    ] {
        let error = error.context("request failed");
        assert!(!is_transient_http_error(&error));
        assert!(!is_transient_graphql_or_http_error(&error));
    }
}

#[test]
fn lite_disabled_service_does_not_schedule_retries() {
    let attempts = Cell::new(0);
    let result = futures::executor::block_on(with_bounded_retry("disabled service", || {
        attempts.set(attempts.get() + 1);
        futures::future::ready(Err::<(), _>(crate::server::CloudServicesUnavailable.into()))
    }));
    assert!(result.is_err());
    assert_eq!(attempts.get(), 1);
}
