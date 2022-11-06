use crate::{dns_header, dns_question, dns_rr, dns_rtype};
use crate::dns_header::DnsHeader;
use crate::dns_rr::DnsRR;

pub struct DnsPacket {
    header: dns_header::DnsHeader,
    question: dns_question::DnsQuestion,
    reponse: Vec<DnsRR>,
}

impl DnsPacket {
    pub fn new(qr: bool, aa: bool, tc: bool, ra: bool, rcode: u16, qdcount: u16, ancount: u16, nscount: u16, arcount: u16, qname: u32, qclass: u32, qtype: dns_rtype::DnsRType) -> DnsPacket {
        let mut r = vec![];
        for i in 0..qdcount{
            r.push(DnsRR::new(dns_question::DnsQuestion::new(3,0x0001,dns_rtype::DnsRType::AAAA),2,2,2));
        }
        for i in 0..ancount{
            r.push(DnsRR::new(dns_question::DnsQuestion::new(3,0x0001,dns_rtype::DnsRType::AAAA),2,2,2));
        }
        for i in 0..nscount{
            r.push(DnsRR::new(dns_question::DnsQuestion::new(3,0x0001,dns_rtype::DnsRType::AAAA),2,2,2));
        }

        DnsPacket{
            header:DnsHeader::new(qr, aa, tc, ra, rcode, qdcount, ancount, nscount, arcount),
            question:dns_question::DnsQuestion::new(qname, qclass, qtype),
            reponse:r,
        }
    }
    pub fn generate_rr(mut self){
        /*
        let answer = self.header.ancount();
        let authority = self.header.nscount();
        let additional = self.header.arcount();
        */
    }
    pub fn byte_size(&self) -> i32 {
        return (12 + 6 + 12 * self.reponse.len()) as i32;
    }
    pub fn header(&self) -> &dns_header::DnsHeader {
        &self.header
    }
    pub fn question(&self) -> &dns_question::DnsQuestion {
        &self.question
    }
    pub fn reponse(&self) -> &Vec<DnsRR> {
        &self.reponse
    }
    pub fn set_header(&mut self, header: dns_header::DnsHeader) {
        self.header = header;
    }
    pub fn set_question(&mut self, question: dns_question::DnsQuestion) {
        self.question = question;
    }
    pub fn set_reponse(&mut self, reponse: Vec<DnsRR>) {
        self.reponse = reponse;
    }
}