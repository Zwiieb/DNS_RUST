use crate::dns_packet::DnsPacket;
use crate::dns_rtype;
use crate::dns_rtype::DnsRType;

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

        let str = self.qname.as_bytes();

        for i in str{
            res.push(*i);
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
    pub fn deserialize(vecQuestion : Vec<u8>)->DnsQuestion{
        let length = vecQuestion.len();
        let mut temp = Vec::new() as Vec<u8>;
        for i in 0..((length-4) as u8){
            if vecQuestion[i as usize]==0x00 as u8{
                break;
            }else{
                temp.push(vecQuestion[i as usize]);
            }
        }
        let mut qtype = ((vecQuestion[length-4] as u16) << 8) | vecQuestion[length-3] as u16;
        let mut rtype = DnsRType::A;
        match qtype{
            1 => rtype = DnsRType::A,
            28 => rtype = DnsRType::AAAA,
            2 => rtype = DnsRType::NS,
            5 => rtype = DnsRType::CNAME,
            12 => rtype = DnsRType::PTR,
            15 => rtype = DnsRType::MX,
            _=> qtype = 1
        }
        println!("qtype = {}",qtype);
        let class = ((vecQuestion[length-2] as u16) << 8) | vecQuestion[length-1] as u16;

        DnsQuestion { qname: DnsPacket::to_dname(temp, 0), qclass : class, qtype: DnsRType::A }
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