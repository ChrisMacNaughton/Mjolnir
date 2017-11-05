use std::net::IpAddr;

use proto;

pub struct Register {
    pub ip: IpAddr,
    pub port: u16,
    pub hostname: String,
}

impl Register {
    pub fn new<T: Into<String>>(ip: IpAddr, port: u16, hostname: T) -> Register {
        Register {
            ip: ip,
            port: port,
            hostname: hostname.into(),
        }
    }
}

impl Into<proto::agent::Register> for Register {
    fn into(self) -> proto::agent::Register {
        let mut register = proto::agent::Register::new();
        register.set_hostname(self.hostname);
        register.set_ip(self.ip.into());
        register.set_port(self.port as i32);
        register
    }
}

impl From<proto::agent::Register> for Register {
    fn from(register: proto::agent::Register) ->Register {
        Register {
            hostname: register.get_hostname().into(),
            ip: register.get_ip().clone().into(),
            port: register.get_port() as u16,
        }
    }
}