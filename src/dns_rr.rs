use crate::dns_question;

pub struct DnsRR {
    question: dns_question::DnsQuestion,
    ttl: i32,
    rdlength: i16,
    rdata: u32,
}

impl DnsRR {
    pub fn new(question: dns_question::DnsQuestion, ttl: i32, rdlength: i16, rdata: u32) -> DnsRR {
        DnsRR{
            question,
            ttl,
            rdlength,
            rdata,
        }
    }
    pub fn question(&self) -> &dns_question::DnsQuestion {
        &self.question
    }
    pub fn ttl(&self) -> i32 {
        self.ttl
    }
    pub fn rdlength(&self) -> i16 {
        self.rdlength
    }
    pub fn rdata(&self) -> u32 {
        self.rdata
    }
    pub fn set_question(&mut self, question: dns_question::DnsQuestion) {
        self.question = question;
    }
    pub fn set_ttl(&mut self, ttl: i32) {
        self.ttl = ttl;
    }
    pub fn set_rdlength(&mut self, rdlength: i16) {
        self.rdlength = rdlength;
    }
    pub fn set_rdata(&mut self, rdata: u32) {
        self.rdata = rdata;
    }
}