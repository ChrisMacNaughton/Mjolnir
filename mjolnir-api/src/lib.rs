extern crate protobuf;

pub use protobuf::core::Message;
pub use protobuf::repeated::RepeatedField;
pub mod agent;
pub mod plugin;

pub use plugin::RemediationResult_ResultType as RemediationResultType;