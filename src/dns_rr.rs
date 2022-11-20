use crate::dns_rtype::DnsRType;

pub struct DnsRR {
    rname: u16,
    Dnsrtype: DnsRType,
    rclass: u16,
    ttl: u32,
    rdlength: u16,
    rdata: String
}

impl DnsRR {
    pub fn new(rname: u16, rtype: DnsRType,rclass:u16,ttl:u32,rdlength: u16,rdata: String)->DnsRR{

        DnsRR{
            rname,
            Dnsrtype: rtype,
            rclass,
            ttl,
            rdlength,
            rdata,
        }
    }
    pub fn serialize(&self)->Vec<u8>{
        let mut res = Vec::new();

        let RRName = self.rname.to_be_bytes();
        for i in RRName{
            res.push(i);
        }

        let RRtype = self.Dnsrtype.no().to_be_bytes();
        for i in RRtype{
            res.push(i);
        }
        let RRclass = self.rclass.to_be_bytes();
        for i in RRclass{
            res.push(i);
        }

        let TTL = self.ttl.to_be_bytes();
        for i in TTL{
            res.push(i);
        }
        let RDL = self.rdlength.to_be_bytes();
        for i in RDL{
            res.push(i);
        }

        let rdata = self.rdata.as_bytes();
        for i in rdata{
            res.push(*i);
        }

        res
    }
    pub fn get_size(&self)-> usize{
        let mut size = 0;
        size += 12;
        size += self.rdata.len();
        size
    }

    pub fn ttl(&self) -> u32 {
        self.ttl
    }
    pub fn rdlength(&self) -> u16 {
        self.rdlength
    }
    pub fn rdata(&self) -> String {
        self.rdata.clone()
    }
    pub fn set_ttl(&mut self, ttl: u32) {
        self.ttl = ttl;
    }
    pub fn set_rdlength(&mut self, rdlength: u16) {
        self.rdlength = rdlength;
    }
    pub fn set_rdata(&mut self, rdata: String) {
        self.rdata = rdata;
    }
}