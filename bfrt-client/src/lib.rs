//! bfrt-client: A BFRuntime client wrapper

pub mod bfrt_info;
pub mod client;
// pub mod digest;
pub mod table;
pub mod utils;

use core::str;

pub use bfrt;

use error_set::error_set;

error_set! {
    DepsError = {
        #[display("GRPC error: [{}] message: {} details: {} metadata: {:?}",
            source.code(), source.message(),
            unsafe{ str::from_utf8_unchecked(source.details()) },
            source.metadata()
        )]
        TonicStatus(tonic::Status),
        SerdeJson(serde_json::Error),
    };
    ClientBasicError = {
        MissingBfrtClient,
        MissingStreamMessageSender,
        Timeout,
        #[display("Wrapped error: {msg}")]
        WrappedError {
            msg: String
        }
    } || DepsError;
    GetBFRTInfoError = {
        ConfigNotFound
    } || ClientBasicError;
    MakeTableKeyError = {
        UnexistedField {
            field_name: String
        },
        MissingSecondValue,
        ExpectedBytes,
        ExpectedI32,
        ExpectedBool,
        UnsupportedMatchType
    };
    MakeTableDataError = {
        UnexistedAction {
            action_name: String
        },
        UnexistedField {
            field_name: String
        }
    };
    DeserializeError = {
        #[display("Custom error: {msg}")]
        Custom {
            msg: String
        },
        ExpectedBool,
        ExpectedI8,
        ExpectedI16,
        ExpectedI32,
        ExpectedI64,
        ExpectedU8,
        ExpectedU16,
        ExpectedU32,
        ExpectedU64,
        ExpectedF32,
        ExpectedBytes,
        ExpectedString,
        MissingFieldValue,
        IndexOutOfBounds,
        UnsupportedDataType,
    };
}
