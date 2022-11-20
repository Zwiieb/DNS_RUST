use crate::dns_rtype;

pub struct DnsQuestion {
    qname: String,
    qtype: dns_rtype::DnsRType,
    qclass: u16,

}

impl DnsQuestion {
    //constructeur new
    pub(crate) fn new(qname: String, qtype: dns_rtype::DnsRType, qclass: u16) -> DnsQuestion {
        if qclass != 0x0001 {
            panic!("Error: qclass not 0x0001");
        }

        DnsQuestion {
            qname,
            qtype,
            qclass,

        }
    }
    pub fn serialize(&self) ->Vec<u8>{

        let mut res = Vec::new() as Vec<u8>;
        let mut split = self.qname.split(".");

        for s in split{
            println!("{}",s);
            let mut strnumber = s.chars().count().to_string();
            let mut z = strnumber.as_bytes();
            for j in z{
                res.push(*j);
            }

            let mut temp = s.as_bytes();
            for i in temp{
                res.push(*i);
            }
        }
        res.push(0x00);

        let Qtype = self.qtype.no().to_be_bytes();
        for i in Qtype{
            res.push(i);
        }
        let Qclass = self.qclass.to_be_bytes();
        for i in Qclass{
            res.push(i);
        }
        res
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
    pub fn qclass(&self) -> u16 {
        self.qclass
    }
    pub fn set_qname(&mut self, qname: String) {
        self.qname = qname;
    }
    pub fn set_qtype(&mut self, qtype: dns_rtype::DnsRType) {
        self.qtype = qtype;
    }
    pub fn set_qclass(&mut self, qclass: u16) {
        if qclass != 0x0001 {
            panic!("Error: qclass not 0x0001");
        }else{
            self.qclass = qclass;
        }
    }

}