use crate::new_key_cache::key_cache::api_call::APICall;

/// Returns throttling rates for a given call.
///
/// Returns a tuple with (requests per second, burst)
pub fn throttle_info(call: APICall) -> (f64, usize) {
    match call {
        APICall::ListCatalogCategories => (1.0, 2),
        APICall::GetCompetitivePricing => (0.5, 1),
        APICall::GetMyFeesEstimates => (0.5, 1),
    }
}
