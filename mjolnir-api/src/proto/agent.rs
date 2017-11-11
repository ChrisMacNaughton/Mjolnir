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
pub struct Register {
    // message fields
    ip: ::protobuf::SingularPtrField<IpAddr>,
    port: ::std::option::Option<i32>,
    hostname: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Register {}

impl Register {
    pub fn new() -> Register {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Register {
        static mut instance: ::protobuf::lazy::Lazy<Register> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Register,
        };
        unsafe {
            instance.get(Register::new)
        }
    }

    // required .Mjolnir.IpAddr ip = 1;

    pub fn clear_ip(&mut self) {
        self.ip.clear();
    }

    pub fn has_ip(&self) -> bool {
        self.ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: IpAddr) {
        self.ip = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ip(&mut self) -> &mut IpAddr {
        if self.ip.is_none() {
            self.ip.set_default();
        }
        self.ip.as_mut().unwrap()
    }

    // Take field
    pub fn take_ip(&mut self) -> IpAddr {
        self.ip.take().unwrap_or_else(|| IpAddr::new())
    }

    pub fn get_ip(&self) -> &IpAddr {
        self.ip.as_ref().unwrap_or_else(|| IpAddr::default_instance())
    }

    fn get_ip_for_reflect(&self) -> &::protobuf::SingularPtrField<IpAddr> {
        &self.ip
    }

    fn mut_ip_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<IpAddr> {
        &mut self.ip
    }

    // required int32 port = 2;

    pub fn clear_port(&mut self) {
        self.port = ::std::option::Option::None;
    }

    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: i32) {
        self.port = ::std::option::Option::Some(v);
    }

    pub fn get_port(&self) -> i32 {
        self.port.unwrap_or(0)
    }

    fn get_port_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.port
    }

    fn mut_port_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.port
    }

    // required string hostname = 3;

    pub fn clear_hostname(&mut self) {
        self.hostname.clear();
    }

    pub fn has_hostname(&self) -> bool {
        self.hostname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostname(&mut self, v: ::std::string::String) {
        self.hostname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostname(&mut self) -> &mut ::std::string::String {
        if self.hostname.is_none() {
            self.hostname.set_default();
        }
        self.hostname.as_mut().unwrap()
    }

    // Take field
    pub fn take_hostname(&mut self) -> ::std::string::String {
        self.hostname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hostname(&self) -> &str {
        match self.hostname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_hostname_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.hostname
    }

    fn mut_hostname_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.hostname
    }
}

impl ::protobuf::Message for Register {
    fn is_initialized(&self) -> bool {
        if self.ip.is_none() {
            return false;
        }
        if self.port.is_none() {
            return false;
        }
        if self.hostname.is_none() {
            return false;
        }
        for v in &self.ip {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ip)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.port = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hostname)?;
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
        if let Some(ref v) = self.ip.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.port {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.hostname.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ip.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.port {
            os.write_int32(2, v)?;
        }
        if let Some(ref v) = self.hostname.as_ref() {
            os.write_string(3, &v)?;
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

impl ::protobuf::MessageStatic for Register {
    fn new() -> Register {
        Register::new()
    }

    fn descriptor_static(_: ::std::option::Option<Register>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<IpAddr>>(
                    "ip",
                    Register::get_ip_for_reflect,
                    Register::mut_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "port",
                    Register::get_port_for_reflect,
                    Register::mut_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hostname",
                    Register::get_hostname_for_reflect,
                    Register::mut_hostname_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Register>(
                    "Register",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Register {
    fn clear(&mut self) {
        self.clear_ip();
        self.clear_port();
        self.clear_hostname();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Register {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Register {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IpAddr {
    // message fields
    version: ::std::option::Option<Version>,
    v4: ::protobuf::SingularPtrField<Ipv4Addr>,
    v6: ::protobuf::SingularPtrField<Ipv6Addr>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IpAddr {}

impl IpAddr {
    pub fn new() -> IpAddr {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IpAddr {
        static mut instance: ::protobuf::lazy::Lazy<IpAddr> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IpAddr,
        };
        unsafe {
            instance.get(IpAddr::new)
        }
    }

    // required .Mjolnir.Version version = 1;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: Version) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> Version {
        self.version.unwrap_or(Version::V4)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<Version> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<Version> {
        &mut self.version
    }

    // optional .Mjolnir.Ipv4Addr v4 = 2;

    pub fn clear_v4(&mut self) {
        self.v4.clear();
    }

    pub fn has_v4(&self) -> bool {
        self.v4.is_some()
    }

    // Param is passed by value, moved
    pub fn set_v4(&mut self, v: Ipv4Addr) {
        self.v4 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_v4(&mut self) -> &mut Ipv4Addr {
        if self.v4.is_none() {
            self.v4.set_default();
        }
        self.v4.as_mut().unwrap()
    }

    // Take field
    pub fn take_v4(&mut self) -> Ipv4Addr {
        self.v4.take().unwrap_or_else(|| Ipv4Addr::new())
    }

    pub fn get_v4(&self) -> &Ipv4Addr {
        self.v4.as_ref().unwrap_or_else(|| Ipv4Addr::default_instance())
    }

    fn get_v4_for_reflect(&self) -> &::protobuf::SingularPtrField<Ipv4Addr> {
        &self.v4
    }

    fn mut_v4_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Ipv4Addr> {
        &mut self.v4
    }

    // optional .Mjolnir.Ipv6Addr v6 = 3;

    pub fn clear_v6(&mut self) {
        self.v6.clear();
    }

    pub fn has_v6(&self) -> bool {
        self.v6.is_some()
    }

    // Param is passed by value, moved
    pub fn set_v6(&mut self, v: Ipv6Addr) {
        self.v6 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_v6(&mut self) -> &mut Ipv6Addr {
        if self.v6.is_none() {
            self.v6.set_default();
        }
        self.v6.as_mut().unwrap()
    }

    // Take field
    pub fn take_v6(&mut self) -> Ipv6Addr {
        self.v6.take().unwrap_or_else(|| Ipv6Addr::new())
    }

    pub fn get_v6(&self) -> &Ipv6Addr {
        self.v6.as_ref().unwrap_or_else(|| Ipv6Addr::default_instance())
    }

    fn get_v6_for_reflect(&self) -> &::protobuf::SingularPtrField<Ipv6Addr> {
        &self.v6
    }

    fn mut_v6_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Ipv6Addr> {
        &mut self.v6
    }
}

impl ::protobuf::Message for IpAddr {
    fn is_initialized(&self) -> bool {
        if self.version.is_none() {
            return false;
        }
        for v in &self.v4 {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.v6 {
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
                    self.version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.v4)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.v6)?;
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
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.v4.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.v6.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.v4.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.v6.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for IpAddr {
    fn new() -> IpAddr {
        IpAddr::new()
    }

    fn descriptor_static(_: ::std::option::Option<IpAddr>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Version>>(
                    "version",
                    IpAddr::get_version_for_reflect,
                    IpAddr::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Ipv4Addr>>(
                    "v4",
                    IpAddr::get_v4_for_reflect,
                    IpAddr::mut_v4_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Ipv6Addr>>(
                    "v6",
                    IpAddr::get_v6_for_reflect,
                    IpAddr::mut_v6_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IpAddr>(
                    "IpAddr",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IpAddr {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_v4();
        self.clear_v6();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IpAddr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IpAddr {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Ipv4Addr {
    // message fields
    a: ::std::option::Option<u32>,
    c: ::std::option::Option<u32>,
    b: ::std::option::Option<u32>,
    d: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Ipv4Addr {}

impl Ipv4Addr {
    pub fn new() -> Ipv4Addr {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Ipv4Addr {
        static mut instance: ::protobuf::lazy::Lazy<Ipv4Addr> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Ipv4Addr,
        };
        unsafe {
            instance.get(Ipv4Addr::new)
        }
    }

    // required uint32 a = 1;

    pub fn clear_a(&mut self) {
        self.a = ::std::option::Option::None;
    }

    pub fn has_a(&self) -> bool {
        self.a.is_some()
    }

    // Param is passed by value, moved
    pub fn set_a(&mut self, v: u32) {
        self.a = ::std::option::Option::Some(v);
    }

    pub fn get_a(&self) -> u32 {
        self.a.unwrap_or(0)
    }

    fn get_a_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.a
    }

    fn mut_a_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.a
    }

    // required uint32 c = 2;

    pub fn clear_c(&mut self) {
        self.c = ::std::option::Option::None;
    }

    pub fn has_c(&self) -> bool {
        self.c.is_some()
    }

    // Param is passed by value, moved
    pub fn set_c(&mut self, v: u32) {
        self.c = ::std::option::Option::Some(v);
    }

    pub fn get_c(&self) -> u32 {
        self.c.unwrap_or(0)
    }

    fn get_c_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.c
    }

    fn mut_c_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.c
    }

    // required uint32 b = 3;

    pub fn clear_b(&mut self) {
        self.b = ::std::option::Option::None;
    }

    pub fn has_b(&self) -> bool {
        self.b.is_some()
    }

    // Param is passed by value, moved
    pub fn set_b(&mut self, v: u32) {
        self.b = ::std::option::Option::Some(v);
    }

    pub fn get_b(&self) -> u32 {
        self.b.unwrap_or(0)
    }

    fn get_b_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.b
    }

    fn mut_b_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.b
    }

    // required uint32 d = 4;

    pub fn clear_d(&mut self) {
        self.d = ::std::option::Option::None;
    }

    pub fn has_d(&self) -> bool {
        self.d.is_some()
    }

    // Param is passed by value, moved
    pub fn set_d(&mut self, v: u32) {
        self.d = ::std::option::Option::Some(v);
    }

    pub fn get_d(&self) -> u32 {
        self.d.unwrap_or(0)
    }

    fn get_d_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.d
    }

    fn mut_d_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.d
    }
}

impl ::protobuf::Message for Ipv4Addr {
    fn is_initialized(&self) -> bool {
        if self.a.is_none() {
            return false;
        }
        if self.c.is_none() {
            return false;
        }
        if self.b.is_none() {
            return false;
        }
        if self.d.is_none() {
            return false;
        }
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
                    let tmp = is.read_uint32()?;
                    self.a = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.c = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.b = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.d = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.a {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.c {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.b {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.d {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.a {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.c {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.b {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.d {
            os.write_uint32(4, v)?;
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

impl ::protobuf::MessageStatic for Ipv4Addr {
    fn new() -> Ipv4Addr {
        Ipv4Addr::new()
    }

    fn descriptor_static(_: ::std::option::Option<Ipv4Addr>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "a",
                    Ipv4Addr::get_a_for_reflect,
                    Ipv4Addr::mut_a_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "c",
                    Ipv4Addr::get_c_for_reflect,
                    Ipv4Addr::mut_c_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "b",
                    Ipv4Addr::get_b_for_reflect,
                    Ipv4Addr::mut_b_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "d",
                    Ipv4Addr::get_d_for_reflect,
                    Ipv4Addr::mut_d_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Ipv4Addr>(
                    "Ipv4Addr",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Ipv4Addr {
    fn clear(&mut self) {
        self.clear_a();
        self.clear_c();
        self.clear_b();
        self.clear_d();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Ipv4Addr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Ipv4Addr {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Ipv6Addr {
    // message fields
    a: ::std::option::Option<u32>,
    c: ::std::option::Option<u32>,
    b: ::std::option::Option<u32>,
    d: ::std::option::Option<u32>,
    e: ::std::option::Option<u32>,
    f: ::std::option::Option<u32>,
    g: ::std::option::Option<u32>,
    h: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Ipv6Addr {}

impl Ipv6Addr {
    pub fn new() -> Ipv6Addr {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Ipv6Addr {
        static mut instance: ::protobuf::lazy::Lazy<Ipv6Addr> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Ipv6Addr,
        };
        unsafe {
            instance.get(Ipv6Addr::new)
        }
    }

    // required uint32 a = 1;

    pub fn clear_a(&mut self) {
        self.a = ::std::option::Option::None;
    }

    pub fn has_a(&self) -> bool {
        self.a.is_some()
    }

    // Param is passed by value, moved
    pub fn set_a(&mut self, v: u32) {
        self.a = ::std::option::Option::Some(v);
    }

    pub fn get_a(&self) -> u32 {
        self.a.unwrap_or(0)
    }

    fn get_a_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.a
    }

    fn mut_a_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.a
    }

    // required uint32 c = 2;

    pub fn clear_c(&mut self) {
        self.c = ::std::option::Option::None;
    }

    pub fn has_c(&self) -> bool {
        self.c.is_some()
    }

    // Param is passed by value, moved
    pub fn set_c(&mut self, v: u32) {
        self.c = ::std::option::Option::Some(v);
    }

    pub fn get_c(&self) -> u32 {
        self.c.unwrap_or(0)
    }

    fn get_c_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.c
    }

    fn mut_c_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.c
    }

    // required uint32 b = 3;

    pub fn clear_b(&mut self) {
        self.b = ::std::option::Option::None;
    }

    pub fn has_b(&self) -> bool {
        self.b.is_some()
    }

    // Param is passed by value, moved
    pub fn set_b(&mut self, v: u32) {
        self.b = ::std::option::Option::Some(v);
    }

    pub fn get_b(&self) -> u32 {
        self.b.unwrap_or(0)
    }

    fn get_b_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.b
    }

    fn mut_b_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.b
    }

    // required uint32 d = 4;

    pub fn clear_d(&mut self) {
        self.d = ::std::option::Option::None;
    }

    pub fn has_d(&self) -> bool {
        self.d.is_some()
    }

    // Param is passed by value, moved
    pub fn set_d(&mut self, v: u32) {
        self.d = ::std::option::Option::Some(v);
    }

    pub fn get_d(&self) -> u32 {
        self.d.unwrap_or(0)
    }

    fn get_d_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.d
    }

    fn mut_d_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.d
    }

    // required uint32 e = 5;

    pub fn clear_e(&mut self) {
        self.e = ::std::option::Option::None;
    }

    pub fn has_e(&self) -> bool {
        self.e.is_some()
    }

    // Param is passed by value, moved
    pub fn set_e(&mut self, v: u32) {
        self.e = ::std::option::Option::Some(v);
    }

    pub fn get_e(&self) -> u32 {
        self.e.unwrap_or(0)
    }

    fn get_e_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.e
    }

    fn mut_e_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.e
    }

    // required uint32 f = 6;

    pub fn clear_f(&mut self) {
        self.f = ::std::option::Option::None;
    }

    pub fn has_f(&self) -> bool {
        self.f.is_some()
    }

    // Param is passed by value, moved
    pub fn set_f(&mut self, v: u32) {
        self.f = ::std::option::Option::Some(v);
    }

    pub fn get_f(&self) -> u32 {
        self.f.unwrap_or(0)
    }

    fn get_f_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.f
    }

    fn mut_f_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.f
    }

    // required uint32 g = 7;

    pub fn clear_g(&mut self) {
        self.g = ::std::option::Option::None;
    }

    pub fn has_g(&self) -> bool {
        self.g.is_some()
    }

    // Param is passed by value, moved
    pub fn set_g(&mut self, v: u32) {
        self.g = ::std::option::Option::Some(v);
    }

    pub fn get_g(&self) -> u32 {
        self.g.unwrap_or(0)
    }

    fn get_g_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.g
    }

    fn mut_g_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.g
    }

    // required uint32 h = 8;

    pub fn clear_h(&mut self) {
        self.h = ::std::option::Option::None;
    }

    pub fn has_h(&self) -> bool {
        self.h.is_some()
    }

    // Param is passed by value, moved
    pub fn set_h(&mut self, v: u32) {
        self.h = ::std::option::Option::Some(v);
    }

    pub fn get_h(&self) -> u32 {
        self.h.unwrap_or(0)
    }

    fn get_h_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.h
    }

    fn mut_h_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.h
    }
}

impl ::protobuf::Message for Ipv6Addr {
    fn is_initialized(&self) -> bool {
        if self.a.is_none() {
            return false;
        }
        if self.c.is_none() {
            return false;
        }
        if self.b.is_none() {
            return false;
        }
        if self.d.is_none() {
            return false;
        }
        if self.e.is_none() {
            return false;
        }
        if self.f.is_none() {
            return false;
        }
        if self.g.is_none() {
            return false;
        }
        if self.h.is_none() {
            return false;
        }
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
                    let tmp = is.read_uint32()?;
                    self.a = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.c = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.b = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.d = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.e = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.f = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.g = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.h = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.a {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.c {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.b {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.d {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.e {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.f {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.g {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.h {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.a {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.c {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.b {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.d {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.e {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.f {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.g {
            os.write_uint32(7, v)?;
        }
        if let Some(v) = self.h {
            os.write_uint32(8, v)?;
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

impl ::protobuf::MessageStatic for Ipv6Addr {
    fn new() -> Ipv6Addr {
        Ipv6Addr::new()
    }

    fn descriptor_static(_: ::std::option::Option<Ipv6Addr>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "a",
                    Ipv6Addr::get_a_for_reflect,
                    Ipv6Addr::mut_a_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "c",
                    Ipv6Addr::get_c_for_reflect,
                    Ipv6Addr::mut_c_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "b",
                    Ipv6Addr::get_b_for_reflect,
                    Ipv6Addr::mut_b_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "d",
                    Ipv6Addr::get_d_for_reflect,
                    Ipv6Addr::mut_d_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "e",
                    Ipv6Addr::get_e_for_reflect,
                    Ipv6Addr::mut_e_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "f",
                    Ipv6Addr::get_f_for_reflect,
                    Ipv6Addr::mut_f_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "g",
                    Ipv6Addr::get_g_for_reflect,
                    Ipv6Addr::mut_g_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "h",
                    Ipv6Addr::get_h_for_reflect,
                    Ipv6Addr::mut_h_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Ipv6Addr>(
                    "Ipv6Addr",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Ipv6Addr {
    fn clear(&mut self) {
        self.clear_a();
        self.clear_c();
        self.clear_b();
        self.clear_d();
        self.clear_e();
        self.clear_f();
        self.clear_g();
        self.clear_h();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Ipv6Addr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Ipv6Addr {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Version {
    V4 = 0,
    V6 = 1,
}

impl ::protobuf::ProtobufEnum for Version {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Version> {
        match value {
            0 => ::std::option::Option::Some(Version::V4),
            1 => ::std::option::Option::Some(Version::V6),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Version] = &[
            Version::V4,
            Version::V6,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Version>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Version", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Version {
}

impl ::protobuf::reflect::ProtobufValue for Version {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12protos/agent.proto\x12\x07Mjolnir\"[\n\x08Register\x12\x1f\n\x02ip\
    \x18\x01\x20\x02(\x0b2\x0f.Mjolnir.IpAddrR\x02ip\x12\x12\n\x04port\x18\
    \x02\x20\x02(\x05R\x04port\x12\x1a\n\x08hostname\x18\x03\x20\x02(\tR\x08\
    hostname\"z\n\x06IpAddr\x12*\n\x07version\x18\x01\x20\x02(\x0e2\x10.Mjol\
    nir.VersionR\x07version\x12!\n\x02v4\x18\x02\x20\x01(\x0b2\x11.Mjolnir.I\
    pv4AddrR\x02v4\x12!\n\x02v6\x18\x03\x20\x01(\x0b2\x11.Mjolnir.Ipv6AddrR\
    \x02v6\"B\n\x08Ipv4Addr\x12\x0c\n\x01a\x18\x01\x20\x02(\rR\x01a\x12\x0c\
    \n\x01c\x18\x02\x20\x02(\rR\x01c\x12\x0c\n\x01b\x18\x03\x20\x02(\rR\x01b\
    \x12\x0c\n\x01d\x18\x04\x20\x02(\rR\x01d\"z\n\x08Ipv6Addr\x12\x0c\n\x01a\
    \x18\x01\x20\x02(\rR\x01a\x12\x0c\n\x01c\x18\x02\x20\x02(\rR\x01c\x12\
    \x0c\n\x01b\x18\x03\x20\x02(\rR\x01b\x12\x0c\n\x01d\x18\x04\x20\x02(\rR\
    \x01d\x12\x0c\n\x01e\x18\x05\x20\x02(\rR\x01e\x12\x0c\n\x01f\x18\x06\x20\
    \x02(\rR\x01f\x12\x0c\n\x01g\x18\x07\x20\x02(\rR\x01g\x12\x0c\n\x01h\x18\
    \x08\x20\x02(\rR\x01h*\x19\n\x07Version\x12\x06\n\x02V4\x10\0\x12\x06\n\
    \x02V6\x10\x01B\x02H\x01\
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
