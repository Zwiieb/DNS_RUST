use crate::dns_rtype;

pub struct DnsQuestion {
    qname: String,
    qtype: dns_rtype::DnsRType,
    qclass: u32,

}

impl DnsQuestion {
    //constructeur new
    pub(crate) fn new(qname: String,  qtype: dns_rtype::DnsRType,qclass: u32) -> DnsQuestion {
        if qclass != 0x0001 {
            panic!("Error: qclass not 0x0001");
        }

        DnsQuestion {
            qname,
            qtype,
            qclass,

        }
    }
    pub fn get_size(&self)-> usize{
        return self.qname.len()+4;
    }
    //get set
    pub fn qname(&self) -> &str {
        &self.qname
    }
    pub fn qtype(&self) -> &dns_rtype::DnsRType {
        &self.qtype
    }
    pub fn qclass(&self) -> u32 {
        self.qclass
    }
    pub fn set_qname(&mut self, qname: String) {
        self.qname = qname;
    }
    pub fn set_qtype(&mut self, qtype: dns_rtype::DnsRType) {
        self.qtype = qtype;
    }
    pub fn set_qclass(&mut self, qclass: u32) {
        self.qclass = qclass;
    }

}