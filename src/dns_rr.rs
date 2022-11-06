use crate::dns_question;
use crate::dns_rtype::DnsRType;

pub struct DnsRR {
    question: dns_question::DnsQuestion,
    ttl: i32,
    rdlength: i16,
    rdata: String,
}

impl DnsRR {
    pub fn new(question: dns_question::DnsQuestion, ttl: i32, rdlength: i16, rdata: String) -> DnsRR {
        DnsRR{
            question,
            ttl,
            rdlength,
            rdata,
        }
    }
    pub fn get_size(&self)-> usize{
        return self.question.qname().len()+ 10+ self.rdata.len();
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
    pub fn rdata(&self) -> String {
        self.rdata.clone()
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
    pub fn set_rdata(&mut self, rdata: String) {
        self.rdata = rdata;
    }
}