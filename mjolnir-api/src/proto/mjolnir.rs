// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Operation {
    // message fields
    operation_type: ::std::option::Option<OperationType>,
    ping_id: ::std::option::Option<u64>,
    alert: ::protobuf::SingularPtrField<super::plugin::Alert>,
    remediate: ::protobuf::SingularPtrField<super::plugin::RemediationRequest>,
    result: ::protobuf::SingularPtrField<super::plugin::RemediationResult>,
    register: ::protobuf::SingularPtrField<super::agent::Register>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Operation {}

impl Operation {
    pub fn new() -> Operation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Operation {
        static mut instance: ::protobuf::lazy::Lazy<Operation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Operation,
        };
        unsafe {
            instance.get(Operation::new)
        }
    }

    // required .Mjolnir.OperationType operation_type = 1;

    pub fn clear_operation_type(&mut self) {
        self.operation_type = ::std::option::Option::None;
    }

    pub fn has_operation_type(&self) -> bool {
        self.operation_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_operation_type(&mut self, v: OperationType) {
        self.operation_type = ::std::option::Option::Some(v);
    }

    pub fn get_operation_type(&self) -> OperationType {
        self.operation_type.unwrap_or(OperationType::REGISTER)
    }

    fn get_operation_type_for_reflect(&self) -> &::std::option::Option<OperationType> {
        &self.operation_type
    }

    fn mut_operation_type_for_reflect(&mut self) -> &mut ::std::option::Option<OperationType> {
        &mut self.operation_type
    }

    // optional uint64 ping_id = 2;

    pub fn clear_ping_id(&mut self) {
        self.ping_id = ::std::option::Option::None;
    }

    pub fn has_ping_id(&self) -> bool {
        self.ping_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_id(&mut self, v: u64) {
        self.ping_id = ::std::option::Option::Some(v);
    }

    pub fn get_ping_id(&self) -> u64 {
        self.ping_id.unwrap_or(0)
    }

    fn get_ping_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.ping_id
    }

    fn mut_ping_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.ping_id
    }

    // optional .Mjolnir.Alert alert = 3;

    pub fn clear_alert(&mut self) {
        self.alert.clear();
    }

    pub fn has_alert(&self) -> bool {
        self.alert.is_some()
    }

    // Param is passed by value, moved
    pub fn set_alert(&mut self, v: super::plugin::Alert) {
        self.alert = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_alert(&mut self) -> &mut super::plugin::Alert {
        if self.alert.is_none() {
            self.alert.set_default();
        }
        self.alert.as_mut().unwrap()
    }

    // Take field
    pub fn take_alert(&mut self) -> super::plugin::Alert {
        self.alert.take().unwrap_or_else(|| super::plugin::Alert::new())
    }

    pub fn get_alert(&self) -> &super::plugin::Alert {
        self.alert.as_ref().unwrap_or_else(|| super::plugin::Alert::default_instance())
    }

    fn get_alert_for_reflect(&self) -> &::protobuf::SingularPtrField<super::plugin::Alert> {
        &self.alert
    }

    fn mut_alert_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::plugin::Alert> {
        &mut self.alert
    }

    // optional .Mjolnir.RemediationRequest remediate = 4;

    pub fn clear_remediate(&mut self) {
        self.remediate.clear();
    }

    pub fn has_remediate(&self) -> bool {
        self.remediate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remediate(&mut self, v: super::plugin::RemediationRequest) {
        self.remediate = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_remediate(&mut self) -> &mut super::plugin::RemediationRequest {
        if self.remediate.is_none() {
            self.remediate.set_default();
        }
        self.remediate.as_mut().unwrap()
    }

    // Take field
    pub fn take_remediate(&mut self) -> super::plugin::RemediationRequest {
        self.remediate.take().unwrap_or_else(|| super::plugin::RemediationRequest::new())
    }

    pub fn get_remediate(&self) -> &super::plugin::RemediationRequest {
        self.remediate.as_ref().unwrap_or_else(|| super::plugin::RemediationRequest::default_instance())
    }

    fn get_remediate_for_reflect(&self) -> &::protobuf::SingularPtrField<super::plugin::RemediationRequest> {
        &self.remediate
    }

    fn mut_remediate_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::plugin::RemediationRequest> {
        &mut self.remediate
    }

    // optional .Mjolnir.RemediationResult result = 5;

    pub fn clear_result(&mut self) {
        self.result.clear();
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: super::plugin::RemediationResult) {
        self.result = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_result(&mut self) -> &mut super::plugin::RemediationResult {
        if self.result.is_none() {
            self.result.set_default();
        }
        self.result.as_mut().unwrap()
    }

    // Take field
    pub fn take_result(&mut self) -> super::plugin::RemediationResult {
        self.result.take().unwrap_or_else(|| super::plugin::RemediationResult::new())
    }

    pub fn get_result(&self) -> &super::plugin::RemediationResult {
        self.result.as_ref().unwrap_or_else(|| super::plugin::RemediationResult::default_instance())
    }

    fn get_result_for_reflect(&self) -> &::protobuf::SingularPtrField<super::plugin::RemediationResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::plugin::RemediationResult> {
        &mut self.result
    }

    // optional .Mjolnir.Register register = 6;

    pub fn clear_register(&mut self) {
        self.register.clear();
    }

    pub fn has_register(&self) -> bool {
        self.register.is_some()
    }

    // Param is passed by value, moved
    pub fn set_register(&mut self, v: super::agent::Register) {
        self.register = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_register(&mut self) -> &mut super::agent::Register {
        if self.register.is_none() {
            self.register.set_default();
        }
        self.register.as_mut().unwrap()
    }

    // Take field
    pub fn take_register(&mut self) -> super::agent::Register {
        self.register.take().unwrap_or_else(|| super::agent::Register::new())
    }

    pub fn get_register(&self) -> &super::agent::Register {
        self.register.as_ref().unwrap_or_else(|| super::agent::Register::default_instance())
    }

    fn get_register_for_reflect(&self) -> &::protobuf::SingularPtrField<super::agent::Register> {
        &self.register
    }

    fn mut_register_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::agent::Register> {
        &mut self.register
    }
}

impl ::protobuf::Message for Operation {
    fn is_initialized(&self) -> bool {
        if self.operation_type.is_none() {
            return false;
        }
        for v in &self.alert {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.remediate {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.result {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.register {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.operation_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.ping_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.alert)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.remediate)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.result)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.register)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.operation_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.ping_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.alert.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.remediate.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.result.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.register.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.operation_type {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.ping_id {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.alert.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.remediate.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.result.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.register.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Operation {
    fn new() -> Operation {
        Operation::new()
    }

    fn descriptor_static(_: ::std::option::Option<Operation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OperationType>>(
                    "operation_type",
                    Operation::get_operation_type_for_reflect,
                    Operation::mut_operation_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ping_id",
                    Operation::get_ping_id_for_reflect,
                    Operation::mut_ping_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::plugin::Alert>>(
                    "alert",
                    Operation::get_alert_for_reflect,
                    Operation::mut_alert_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::plugin::RemediationRequest>>(
                    "remediate",
                    Operation::get_remediate_for_reflect,
                    Operation::mut_remediate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::plugin::RemediationResult>>(
                    "result",
                    Operation::get_result_for_reflect,
                    Operation::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::agent::Register>>(
                    "register",
                    Operation::get_register_for_reflect,
                    Operation::mut_register_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Operation>(
                    "Operation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Operation {
    fn clear(&mut self) {
        self.clear_operation_type();
        self.clear_ping_id();
        self.clear_alert();
        self.clear_remediate();
        self.clear_result();
        self.clear_register();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Operation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OperationType {
    REGISTER = 0,
    PING = 1,
    PONG = 2,
    ALERT = 3,
    REMEDIATE = 4,
    REMEDIATION_RESULT = 5,
    ACK = 6,
}

impl ::protobuf::ProtobufEnum for OperationType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OperationType> {
        match value {
            0 => ::std::option::Option::Some(OperationType::REGISTER),
            1 => ::std::option::Option::Some(OperationType::PING),
            2 => ::std::option::Option::Some(OperationType::PONG),
            3 => ::std::option::Option::Some(OperationType::ALERT),
            4 => ::std::option::Option::Some(OperationType::REMEDIATE),
            5 => ::std::option::Option::Some(OperationType::REMEDIATION_RESULT),
            6 => ::std::option::Option::Some(OperationType::ACK),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OperationType] = &[
            OperationType::REGISTER,
            OperationType::PING,
            OperationType::PONG,
            OperationType::ALERT,
            OperationType::REMEDIATE,
            OperationType::REMEDIATION_RESULT,
            OperationType::ACK,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<OperationType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("OperationType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for OperationType {
}

impl ::protobuf::reflect::ProtobufValue for OperationType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14protos/mjolnir.proto\x12\x07Mjolnir\x1a\x12protos/agent.proto\x1a\
    \x13protos/plugin.proto\"\xa7\x02\n\tOperation\x12=\n\x0eoperation_type\
    \x18\x01\x20\x02(\x0e2\x16.Mjolnir.OperationTypeR\roperationType\x12\x17\
    \n\x07ping_id\x18\x02\x20\x01(\x04R\x06pingId\x12$\n\x05alert\x18\x03\
    \x20\x01(\x0b2\x0e.Mjolnir.AlertR\x05alert\x129\n\tremediate\x18\x04\x20\
    \x01(\x0b2\x1b.Mjolnir.RemediationRequestR\tremediate\x122\n\x06result\
    \x18\x05\x20\x01(\x0b2\x1a.Mjolnir.RemediationResultR\x06result\x12-\n\
    \x08register\x18\x06\x20\x01(\x0b2\x11.Mjolnir.RegisterR\x08register*l\n\
    \rOperationType\x12\x0c\n\x08REGISTER\x10\0\x12\x08\n\x04PING\x10\x01\
    \x12\x08\n\x04PONG\x10\x02\x12\t\n\x05ALERT\x10\x03\x12\r\n\tREMEDIATE\
    \x10\x04\x12\x16\n\x12REMEDIATION_RESULT\x10\x05\x12\x07\n\x03ACK\x10\
    \x06B\x02H\x01\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
