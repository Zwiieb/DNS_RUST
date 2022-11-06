
use crate::{dns_header, dns_question, dns_rr, dns_rtype};
use crate::dns_rr::DnsRR;

pub struct DnsPacket {
    header: dns_header::DnsHeader,
    question: dns_question::DnsQuestion,
    reponse: Vec<DnsRR>,
}

impl DnsPacket {
    pub fn generate_rr(mut self){
        /*
        let answer = self.header.ancount();
        let authority = self.header.nscount();
        let additional = self.header.arcount();
        */
        let rr = dns_rr::DnsRR::new(dns_question::DnsQuestion::new(3,0x0001,dns_rtype::DnsRType::AAAA),2,2,2);
        self.reponse.push(rr);
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