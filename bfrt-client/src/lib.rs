//! bfrt-client: A BFRuntime client wrapper

pub mod bfrt_info;
pub mod client;

pub use bfrt;

use error_set::error_set;

error_set! {
    DepsError = {
        TonicStatus(tonic::Status),
        SerdeJson(serde_json::Error),
    };
    GetBFRTInfoError = {
        ConfigNotFound
    } || DepsError;
}
