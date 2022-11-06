use crate::dns_rtype;

pub struct DnsQuestion {
    qname: u32,
    qtype: dns_rtype::DnsRType,
    qclass: u32,

}

impl DnsQuestion {
    //constructeur new
    pub(crate) fn new(a: u32, b: u32, c: dns_rtype::DnsRType) -> DnsQuestion {
        if b != 0x0001 {
            panic!("Error: qclass not 0x0001");
        }

        DnsQuestion {
            qname: a,
            qclass: b,
            qtype: c,
        }
    }

    //get set
    pub fn qname(&self) -> u32 {
        self.qname
    }
    pub fn qtype(&self) -> &dns_rtype::DnsRType {
        &self.qtype
    }
    pub fn qclass(&self) -> u32 {
        self.qclass
    }
    pub fn set_qname(&mut self, qname: u32) {
        self.qname = qname;
    }
    pub fn set_qtype(&mut self, qtype: dns_rtype::DnsRType) {
        self.qtype = qtype;
    }
    pub fn set_qclass(&mut self, qclass: u32) {
        self.qclass = qclass;
    }
}