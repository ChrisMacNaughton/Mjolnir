use protobuf;

#[macro_use]
extern crate serde_derive;


use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub use protobuf::{Message, RepeatedField, parse_from_bytes};

use uuid::Uuid;

pub mod proto;
// pub mod agent;
// pub mod plugin;
// mod mjolnir;

pub use crate::proto::plugin::RemediationResult_ResultType as RemediationResultType;
pub use crate::proto::mjolnir::{Operation, OperationType};

pub use crate::proto::plugin;

mod discover;
mod register;
mod alert;
mod plugin_entry;
mod remediation;
mod remediation_result;

pub use crate::discover::Discover;
pub use crate::register::Register;
pub use crate::alert::Alert;
pub use crate::plugin_entry::PluginEntry;
pub use crate::remediation::Remediation;
pub use crate::remediation_result::RemediationResult;

#[macro_export]
macro_rules! plugin_list {
    ( $( $name:expr => $call:ident ),* ) => {
        {
            let mut plugins: HashMap<String, _> = HashMap::new();
            $(
                plugins.insert(
                    $name.into(), $call as fn(HashMap<String, String>) -> RemediationResult);
            )*
            plugins
        }
    };
}


impl proto::plugin::Discover {
    pub fn try_from(input: &[u8]) -> Result<proto::plugin::Discover, protobuf::ProtobufError> {
        parse_from_bytes::<proto::plugin::Discover>(input)
    }
}

impl Into<proto::agent::IpAddr> for IpAddr {
    fn into(self) -> proto::agent::IpAddr {
        let mut addr = proto::agent::IpAddr::new();
        match self {
            IpAddr::V4(ref a) => {
                addr.set_version(proto::agent::Version::V4);
                let mut address = proto::agent::Ipv4Addr::new();
                let octets = a.octets();
                address.set_a(octets[0] as u32);
                address.set_b(octets[1] as u32);
                address.set_c(octets[2] as u32);
                address.set_d(octets[3] as u32);
                addr.set_v4(address);
            }
            IpAddr::V6(ref a) => {
                addr.set_version(proto::agent::Version::V6);
                let mut address = proto::agent::Ipv6Addr::new();
                let octets = a.octets();
                address.set_a(octets[0] as u32);
                address.set_b(octets[1] as u32);
                address.set_c(octets[2] as u32);
                address.set_d(octets[3] as u32);
                address.set_e(octets[4] as u32);
                address.set_f(octets[5] as u32);
                address.set_g(octets[6] as u32);
                address.set_h(octets[7] as u32);

                addr.set_v6(address);
            }
        }
        addr
    }
}

impl From<proto::agent::IpAddr> for IpAddr {
    fn from(addr: proto::agent::IpAddr) -> IpAddr {
        match addr.get_version() {
            proto::agent::Version::V4 => {
                let ip = addr.get_v4();
                IpAddr::V4(Ipv4Addr::new(
                    ip.get_a() as u8,
                    ip.get_b() as u8,
                    ip.get_c() as u8,
                    ip.get_d() as u8,
                ))
            }
            proto::agent::Version::V6 => {
                let ip = addr.get_v6();
                IpAddr::V6(Ipv6Addr::new(
                    ip.get_a() as u16,
                    ip.get_b() as u16,
                    ip.get_c() as u16,
                    ip.get_d() as u16,
                    ip.get_e() as u16,
                    ip.get_f() as u16,
                    ip.get_g() as u16,
                    ip.get_h() as u16,
                ))
            }
        }
    }
}

impl From<proto::agent::UUID> for Uuid {
    fn from(uuid: proto::agent::UUID) -> Uuid {
        (&uuid).into()
    }
}

impl<'a> From<&'a proto::agent::UUID> for Uuid {
    fn from(uuid: &'a proto::agent::UUID) -> Uuid {
        Uuid::parse_str(uuid.get_value()).unwrap()
    }
}

impl Into<proto::agent::UUID> for Uuid {
    fn into(self) -> proto::agent::UUID {
        let mut uuid = proto::agent::UUID::new();
        uuid.set_value(format!("{}", self));
        uuid
    }
}
