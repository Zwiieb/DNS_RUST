use crate::{dns_header, dns_question};
use crate::dns_header::DnsHeader;
use crate::dns_question::DnsQuestion;
use crate::dns_rr::DnsRR;
use crate::dns_rtype::DnsRType;

pub struct DnsPacket {
    header: dns_header::DnsHeader,
    question: dns_question::DnsQuestion,
    reponse: Vec<DnsRR>,
}

impl DnsPacket {
    pub fn new(qr: bool, aa: bool, tc: bool, ra: bool, rcode: u16, qdcount: u16, ancount: u16, nscount: u16,arcount : u16, qname:String,offset:u16, qtype : DnsRType, qclass : u16,ttl:u32,rdlength:u16,rdata:String) -> DnsPacket{

        let mut vec = vec![];
        for z in 0..(ancount+nscount+arcount){
            vec.push(DnsRR::new(offset, qtype, qclass,ttl,rdlength,rdata.clone()));
        }
        DnsPacket {
            header: DnsHeader::new(qr,aa,tc,ra,rcode,qdcount,ancount,nscount,arcount),
            question: DnsQuestion::new(qname,qtype,qclass),
            reponse: vec,
        }
    }

    pub fn serialize(&self)-> Vec<u8>{
        let mut vec = Vec::new() as Vec<u8>;

        vec.append(&mut self.header.serialize());
        vec.append(&mut self.question.serialize());
        for RR in self.reponse.iter(){
            vec.append(&mut RR.serialize());
        }
        vec
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
    pub fn to_dname(vec: Vec<u8>,offset : u8 )->String{
        let mut temp = Vec::new() as Vec<u8>;
        for i in offset..(vec.len() as u8){
            if vec[i as usize]==0x00 as u8{
                break;
            }else{
                temp.push(vec[i as usize]);
            }
        }
        let s = match String::from_utf8(temp) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        s
    }
    pub fn header(&mut self) -> &mut dns_header::DnsHeader {
        &mut self.header
    }
    pub fn question(&mut self) -> &mut dns_question::DnsQuestion {
        &mut self.question
    }
    pub fn reponse(&mut self) -> &mut Vec<DnsRR> {
        &mut self.reponse
    }
    pub fn set_header(&mut self, header: dns_header::DnsHeader) {
        self.header = header;
    }
    pub fn set_question(&mut self, question: dns_question::DnsQuestion) {self.question = question;}
    pub fn set_reponse(&mut self, reponse: Vec<DnsRR>) {
        self.reponse = reponse;
    }
}