use super::addr::Ipv4Addr;
use libc::in_addr;
use std::fmt;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}

impl fmt::Debug for ip_mreq {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "ip_mreq {{ imr_multiaddr: {{ s_addr: 0x{:x} }}, imr_interface: {{ s_addr: 0x{:x} }} }}",
                    self.imr_multiaddr.s_addr, self.imr_interface.s_addr)
    }
}

impl ip_mreq {
    pub fn new(group: Ipv4Addr, interface: Option<Ipv4Addr>) -> ip_mreq {
        ip_mreq {
            imr_multiaddr: group.0,
            imr_interface: interface.unwrap_or(Ipv4Addr::any()).0
        }
    }
}
