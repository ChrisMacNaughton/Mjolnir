use std::net::IpAddr;

use uuid::Uuid;

use proto;

#[cfg(test)]
mod tests {
    use super::*;

    pub use protobuf::core::{parse_from_bytes, Message};

    #[test]
    fn it_serializes_and_deserializes() {
        let register = Register::new(
            "10.0.0.1".parse().unwrap(),
            12011,
            "awesome.local",
            "supersecret",
            "pub_key",
            Uuid::new_v4(),
        );

        let request: proto::agent::Register = register.clone().into();

        let bytes = request.write_to_bytes().unwrap();
        let register2 = parse_from_bytes::<proto::agent::Register>(&bytes)
            .unwrap()
            .into();
        assert_eq!(register, register2);
    }

    #[test]
    fn it_serializes_and_deserializes_with_ipv6() {
        let register = Register::new(
            "::".parse().unwrap(),
            12011,
            "awesome.local",
            "supersecret",
            "pub_key",
            Uuid::new_v4(),
        );

        let request: proto::agent::Register = register.clone().into();

        let bytes = request.write_to_bytes().unwrap();
        let register2 = parse_from_bytes::<proto::agent::Register>(&bytes)
            .unwrap()
            .into();
        assert_eq!(register, register2);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Register {
    pub ip: IpAddr,
    pub port: u16,
    pub hostname: String,
    pub secret: String,
    pub public_key: String,
    pub uuid: Uuid,
}

impl Register {
    pub fn new<T1, T2, T3, T4>(
        ip: IpAddr,
        port: u16,
        hostname: T1,
        secret: T2,
        public_key: T3,
        uuid: T4,
    ) -> Register
    where
        T1: Into<String>,
        T2: Into<String>,
        T3: Into<String>,
        T4: Into<Uuid>,
    {
        Register {
            ip: ip,
            port: port,
            hostname: hostname.into(),
            secret: secret.into(),
            public_key: public_key.into(),
            uuid: uuid.into(),
        }
    }
}

impl Into<proto::agent::Register> for Register {
    fn into(self) -> proto::agent::Register {
        let mut register = proto::agent::Register::new();
        register.set_hostname(self.hostname);
        register.set_ip(self.ip.into());
        register.set_port(self.port as i32);
        register.set_secret(self.secret);
        register.set_public_key(self.public_key);
        register.set_uuid(self.uuid.into());
        register
    }
}

impl From<proto::agent::Register> for Register {
    fn from(register: proto::agent::Register) -> Register {
        (&register).into()
    }
}

impl<'a> From<&'a proto::agent::Register> for Register {
    fn from(register: &proto::agent::Register) -> Register {
        Register {
            hostname: register.get_hostname().into(),
            ip: register.get_ip().clone().into(),
            port: register.get_port() as u16,
            secret: register.get_secret().into(),
            public_key: register.get_public_key().into(),
            uuid: register.get_uuid().into(),
        }
    }
}
