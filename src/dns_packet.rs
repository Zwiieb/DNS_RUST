use crate::{dns_header, dns_question, dns_rr, dns_rtype};
use crate::dns_header::DnsHeader;
use crate::dns_rr::DnsRR;

pub struct DnsPacket {
    header: dns_header::DnsHeader,
    question: dns_question::DnsQuestion,
    reponse: Vec<DnsRR>,
}

impl DnsPacket {
    pub fn new(qr: bool, aa: bool, tc: bool, ra: bool, rcode: u16, qdcount: u16, ancount: u16, nscount: u16, arcount: u16, qname: String, qclass: u32, qtype: dns_rtype::DnsRType) -> DnsPacket {
        let mut r = vec![];
        for i in 0..qdcount+ancount+nscount{
            r.push(DnsRR::new(dns_question::DnsQuestion::new(qname.clone(),qtype,qclass),2,2,String::from("3")));
        }
        DnsPacket{
            header:DnsHeader::new(qr, aa, tc, ra, rcode, qdcount, ancount, nscount, arcount),
            question:dns_question::DnsQuestion::new(qname, qtype,qclass),
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
    pub fn byte_size(&self)-> usize{
        let mut size_in_octet = 0;
        //rr
        for rr in self.reponse.iter(){
            size_in_octet += rr.get_size();
        }
        size_in_octet += self.question.get_size();
        //header
        size_in_octet+=12;
        return size_in_octet
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