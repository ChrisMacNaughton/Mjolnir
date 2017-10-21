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
pub struct RemediationRequest {
    // message fields
    plugin: ::protobuf::SingularField<::std::string::String>,
    target: ::protobuf::SingularField<::std::string::String>,
    args: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemediationRequest {}

impl RemediationRequest {
    pub fn new() -> RemediationRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemediationRequest {
        static mut instance: ::protobuf::lazy::Lazy<RemediationRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemediationRequest,
        };
        unsafe {
            instance.get(RemediationRequest::new)
        }
    }

    // required string plugin = 1;

    pub fn clear_plugin(&mut self) {
        self.plugin.clear();
    }

    pub fn has_plugin(&self) -> bool {
        self.plugin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_plugin(&mut self, v: ::std::string::String) {
        self.plugin = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_plugin(&mut self) -> &mut ::std::string::String {
        if self.plugin.is_none() {
            self.plugin.set_default();
        }
        self.plugin.as_mut().unwrap()
    }

    // Take field
    pub fn take_plugin(&mut self) -> ::std::string::String {
        self.plugin.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_plugin(&self) -> &str {
        match self.plugin.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_plugin_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.plugin
    }

    fn mut_plugin_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.plugin
    }

    // optional string target = 2;

    pub fn clear_target(&mut self) {
        self.target.clear();
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: ::std::string::String) {
        self.target = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target(&mut self) -> &mut ::std::string::String {
        if self.target.is_none() {
            self.target.set_default();
        }
        self.target.as_mut().unwrap()
    }

    // Take field
    pub fn take_target(&mut self) -> ::std::string::String {
        self.target.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_target(&self) -> &str {
        match self.target.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_target_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.target
    }

    fn mut_target_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.target
    }

    // repeated string args = 3;

    pub fn clear_args(&mut self) {
        self.args.clear();
    }

    // Param is passed by value, moved
    pub fn set_args(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.args = v;
    }

    // Mutable pointer to the field.
    pub fn mut_args(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.args
    }

    // Take field
    pub fn take_args(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.args, ::protobuf::RepeatedField::new())
    }

    pub fn get_args(&self) -> &[::std::string::String] {
        &self.args
    }

    fn get_args_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.args
    }

    fn mut_args_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.args
    }
}

impl ::protobuf::Message for RemediationRequest {
    fn is_initialized(&self) -> bool {
        if self.plugin.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.plugin)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.target)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.args)?;
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
        if let Some(ref v) = self.plugin.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.target.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        for value in &self.args {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.plugin.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.target.as_ref() {
            os.write_string(2, &v)?;
        }
        for v in &self.args {
            os.write_string(3, &v)?;
        };
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

impl ::protobuf::MessageStatic for RemediationRequest {
    fn new() -> RemediationRequest {
        RemediationRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemediationRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "plugin",
                    RemediationRequest::get_plugin_for_reflect,
                    RemediationRequest::mut_plugin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "target",
                    RemediationRequest::get_target_for_reflect,
                    RemediationRequest::mut_target_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "args",
                    RemediationRequest::get_args_for_reflect,
                    RemediationRequest::mut_args_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RemediationRequest>(
                    "RemediationRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemediationRequest {
    fn clear(&mut self) {
        self.clear_plugin();
        self.clear_target();
        self.clear_args();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RemediationRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemediationRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RemediationResult {
    // message fields
    result: ::std::option::Option<RemediationResult_ResultType>,
    error_msg: ::protobuf::SingularField<::std::string::String>,
    alerts: ::protobuf::RepeatedField<Alert>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemediationResult {}

impl RemediationResult {
    pub fn new() -> RemediationResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemediationResult {
        static mut instance: ::protobuf::lazy::Lazy<RemediationResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemediationResult,
        };
        unsafe {
            instance.get(RemediationResult::new)
        }
    }

    // required .Mjolnir.RemediationResult.ResultType result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: RemediationResult_ResultType) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> RemediationResult_ResultType {
        self.result.unwrap_or(RemediationResult_ResultType::OK)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<RemediationResult_ResultType> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<RemediationResult_ResultType> {
        &mut self.result
    }

    // optional string error_msg = 2;

    pub fn clear_error_msg(&mut self) {
        self.error_msg.clear();
    }

    pub fn has_error_msg(&self) -> bool {
        self.error_msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_msg(&mut self, v: ::std::string::String) {
        self.error_msg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_msg(&mut self) -> &mut ::std::string::String {
        if self.error_msg.is_none() {
            self.error_msg.set_default();
        }
        self.error_msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_error_msg(&mut self) -> ::std::string::String {
        self.error_msg.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error_msg(&self) -> &str {
        match self.error_msg.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_msg_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error_msg
    }

    fn mut_error_msg_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error_msg
    }

    // repeated .Mjolnir.Alert alerts = 3;

    pub fn clear_alerts(&mut self) {
        self.alerts.clear();
    }

    // Param is passed by value, moved
    pub fn set_alerts(&mut self, v: ::protobuf::RepeatedField<Alert>) {
        self.alerts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_alerts(&mut self) -> &mut ::protobuf::RepeatedField<Alert> {
        &mut self.alerts
    }

    // Take field
    pub fn take_alerts(&mut self) -> ::protobuf::RepeatedField<Alert> {
        ::std::mem::replace(&mut self.alerts, ::protobuf::RepeatedField::new())
    }

    pub fn get_alerts(&self) -> &[Alert] {
        &self.alerts
    }

    fn get_alerts_for_reflect(&self) -> &::protobuf::RepeatedField<Alert> {
        &self.alerts
    }

    fn mut_alerts_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Alert> {
        &mut self.alerts
    }
}

impl ::protobuf::Message for RemediationResult {
    fn is_initialized(&self) -> bool {
        if self.result.is_none() {
            return false;
        }
        for v in &self.alerts {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_msg)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.alerts)?;
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
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.error_msg.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        for value in &self.alerts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.error_msg.as_ref() {
            os.write_string(2, &v)?;
        }
        for v in &self.alerts {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for RemediationResult {
    fn new() -> RemediationResult {
        RemediationResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemediationResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RemediationResult_ResultType>>(
                    "result",
                    RemediationResult::get_result_for_reflect,
                    RemediationResult::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error_msg",
                    RemediationResult::get_error_msg_for_reflect,
                    RemediationResult::mut_error_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Alert>>(
                    "alerts",
                    RemediationResult::get_alerts_for_reflect,
                    RemediationResult::mut_alerts_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RemediationResult>(
                    "RemediationResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemediationResult {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_error_msg();
        self.clear_alerts();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RemediationResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemediationResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RemediationResult_ResultType {
    OK = 0,
    ERR = 1,
}

impl ::protobuf::ProtobufEnum for RemediationResult_ResultType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RemediationResult_ResultType> {
        match value {
            0 => ::std::option::Option::Some(RemediationResult_ResultType::OK),
            1 => ::std::option::Option::Some(RemediationResult_ResultType::ERR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RemediationResult_ResultType] = &[
            RemediationResult_ResultType::OK,
            RemediationResult_ResultType::ERR,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RemediationResult_ResultType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RemediationResult_ResultType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RemediationResult_ResultType {
}

impl ::protobuf::reflect::ProtobufValue for RemediationResult_ResultType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Alert {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    source: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Alert {}

impl Alert {
    pub fn new() -> Alert {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Alert {
        static mut instance: ::protobuf::lazy::Lazy<Alert> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Alert,
        };
        unsafe {
            instance.get(Alert::new)
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // required string source = 2;

    pub fn clear_source(&mut self) {
        self.source.clear();
    }

    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source(&mut self, v: ::std::string::String) {
        self.source = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source(&mut self) -> &mut ::std::string::String {
        if self.source.is_none() {
            self.source.set_default();
        }
        self.source.as_mut().unwrap()
    }

    // Take field
    pub fn take_source(&mut self) -> ::std::string::String {
        self.source.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_source(&self) -> &str {
        match self.source.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_source_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.source
    }

    fn mut_source_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.source
    }
}

impl ::protobuf::Message for Alert {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        if self.source.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.source)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.source.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.source.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for Alert {
    fn new() -> Alert {
        Alert::new()
    }

    fn descriptor_static(_: ::std::option::Option<Alert>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Alert::get_name_for_reflect,
                    Alert::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "source",
                    Alert::get_source_for_reflect,
                    Alert::mut_source_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Alert>(
                    "Alert",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Alert {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_source();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Alert {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Alert {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Discover {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    author: ::protobuf::SingularField<::std::string::String>,
    version: ::protobuf::SingularField<::std::string::String>,
    alerts: ::protobuf::RepeatedField<Alert>,
    actions: ::protobuf::RepeatedField<RemediationRequest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Discover {}

impl Discover {
    pub fn new() -> Discover {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Discover {
        static mut instance: ::protobuf::lazy::Lazy<Discover> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Discover,
        };
        unsafe {
            instance.get(Discover::new)
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string author = 2;

    pub fn clear_author(&mut self) {
        self.author.clear();
    }

    pub fn has_author(&self) -> bool {
        self.author.is_some()
    }

    // Param is passed by value, moved
    pub fn set_author(&mut self, v: ::std::string::String) {
        self.author = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_author(&mut self) -> &mut ::std::string::String {
        if self.author.is_none() {
            self.author.set_default();
        }
        self.author.as_mut().unwrap()
    }

    // Take field
    pub fn take_author(&mut self) -> ::std::string::String {
        self.author.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_author(&self) -> &str {
        match self.author.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_author_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.author
    }

    fn mut_author_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.author
    }

    // optional string version = 3;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        if self.version.is_none() {
            self.version.set_default();
        }
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        self.version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_version(&self) -> &str {
        match self.version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.version
    }

    // repeated .Mjolnir.Alert alerts = 4;

    pub fn clear_alerts(&mut self) {
        self.alerts.clear();
    }

    // Param is passed by value, moved
    pub fn set_alerts(&mut self, v: ::protobuf::RepeatedField<Alert>) {
        self.alerts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_alerts(&mut self) -> &mut ::protobuf::RepeatedField<Alert> {
        &mut self.alerts
    }

    // Take field
    pub fn take_alerts(&mut self) -> ::protobuf::RepeatedField<Alert> {
        ::std::mem::replace(&mut self.alerts, ::protobuf::RepeatedField::new())
    }

    pub fn get_alerts(&self) -> &[Alert] {
        &self.alerts
    }

    fn get_alerts_for_reflect(&self) -> &::protobuf::RepeatedField<Alert> {
        &self.alerts
    }

    fn mut_alerts_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Alert> {
        &mut self.alerts
    }

    // repeated .Mjolnir.RemediationRequest actions = 5;

    pub fn clear_actions(&mut self) {
        self.actions.clear();
    }

    // Param is passed by value, moved
    pub fn set_actions(&mut self, v: ::protobuf::RepeatedField<RemediationRequest>) {
        self.actions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_actions(&mut self) -> &mut ::protobuf::RepeatedField<RemediationRequest> {
        &mut self.actions
    }

    // Take field
    pub fn take_actions(&mut self) -> ::protobuf::RepeatedField<RemediationRequest> {
        ::std::mem::replace(&mut self.actions, ::protobuf::RepeatedField::new())
    }

    pub fn get_actions(&self) -> &[RemediationRequest] {
        &self.actions
    }

    fn get_actions_for_reflect(&self) -> &::protobuf::RepeatedField<RemediationRequest> {
        &self.actions
    }

    fn mut_actions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RemediationRequest> {
        &mut self.actions
    }
}

impl ::protobuf::Message for Discover {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        for v in &self.alerts {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.actions {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.author)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.version)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.alerts)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.actions)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.author.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.version.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        for value in &self.alerts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.actions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.author.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.version.as_ref() {
            os.write_string(3, &v)?;
        }
        for v in &self.alerts {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.actions {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for Discover {
    fn new() -> Discover {
        Discover::new()
    }

    fn descriptor_static(_: ::std::option::Option<Discover>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Discover::get_name_for_reflect,
                    Discover::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "author",
                    Discover::get_author_for_reflect,
                    Discover::mut_author_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    Discover::get_version_for_reflect,
                    Discover::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Alert>>(
                    "alerts",
                    Discover::get_alerts_for_reflect,
                    Discover::mut_alerts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RemediationRequest>>(
                    "actions",
                    Discover::get_actions_for_reflect,
                    Discover::mut_actions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Discover>(
                    "Discover",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Discover {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_author();
        self.clear_version();
        self.clear_alerts();
        self.clear_actions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Discover {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Discover {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13protos/plugin.proto\x12\x07Mjolnir\x1a\x12protos/agent.proto\"X\n\
    \x12RemediationRequest\x12\x16\n\x06plugin\x18\x01\x20\x02(\tR\x06plugin\
    \x12\x16\n\x06target\x18\x02\x20\x01(\tR\x06target\x12\x12\n\x04args\x18\
    \x03\x20\x03(\tR\x04args\"\xb6\x01\n\x11RemediationResult\x12=\n\x06resu\
    lt\x18\x01\x20\x02(\x0e2%.Mjolnir.RemediationResult.ResultTypeR\x06resul\
    t\x12\x1b\n\terror_msg\x18\x02\x20\x01(\tR\x08errorMsg\x12&\n\x06alerts\
    \x18\x03\x20\x03(\x0b2\x0e.Mjolnir.AlertR\x06alerts\"\x1d\n\nResultType\
    \x12\x06\n\x02OK\x10\0\x12\x07\n\x03ERR\x10\x01\"3\n\x05Alert\x12\x12\n\
    \x04name\x18\x01\x20\x02(\tR\x04name\x12\x16\n\x06source\x18\x02\x20\x02\
    (\tR\x06source\"\xaf\x01\n\x08Discover\x12\x12\n\x04name\x18\x01\x20\x02\
    (\tR\x04name\x12\x16\n\x06author\x18\x02\x20\x01(\tR\x06author\x12\x18\n\
    \x07version\x18\x03\x20\x01(\tR\x07version\x12&\n\x06alerts\x18\x04\x20\
    \x03(\x0b2\x0e.Mjolnir.AlertR\x06alerts\x125\n\x07actions\x18\x05\x20\
    \x03(\x0b2\x1b.Mjolnir.RemediationRequestR\x07actionsB\x02H\x01\
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
