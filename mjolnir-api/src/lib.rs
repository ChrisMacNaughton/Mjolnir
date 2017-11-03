extern crate protobuf;

pub use protobuf::core::{Message, parse_from_bytes};
pub use protobuf::repeated::RepeatedField;
pub mod agent;
pub mod plugin;
mod mjolnir;

pub use plugin::RemediationResult_ResultType as RemediationResultType;
pub use mjolnir::{Operation, OperationType};

pub use agent::Register;

impl plugin::Discover {
    pub fn try_from(input: &[u8]) -> Result<plugin::Discover, protobuf::ProtobufError> {
        parse_from_bytes::<plugin::Discover>(input)
    }
}