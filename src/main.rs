
pub enum DnsRType {
    A,
    AAAA,
    NS,
    MX
}

mod dns_question {
    use crate::DnsRType;

    pub struct DnsQuestion {
        qname : u32 ,
        qtype : DnsRType ,
        qclass : u32 ,

    }
    impl DnsQuestion {
        fn new(a : u32 , b : u32,c : DnsRType) -> DnsQuestion {

            if b != 0x0001{
                eprintln!("Error: qclass not 0x0001");
                std::process::exit(1);
            }

            DnsQuestion {
                qname : a ,
                qclass: b ,
                qtype : c,
            }
        }
        pub fn qname(&self) -> u32 {
            self.qname
        }
        pub fn qtype(&self) -> &DnsRType {
            &self.qtype
        }
        pub fn qclass(&self) -> u32 {
            self.qclass
        }
        pub fn set_qname(&mut self, qname: u32) {
            self.qname = qname;
        }
        pub fn set_qtype(&mut self, qtype: DnsRType) {
            self.qtype = qtype;
        }
        pub fn set_qclass(&mut self, qclass: u32) {
            self.qclass = qclass;
        }
    }
}
mod dns_packet {
    use crate::{dns_header, dns_question, dns_rr};

    pub struct DnsPacket {
        header: dns_header::DnsHeader ,
        question: dns_question::DnsQuestion,
        reponse: dns_rr::DnsRR
    }

    impl DnsPacket {
        pub fn header(&self) -> &dns_header::DnsHeader {
            &self.header
        }
        pub fn question(&self) -> &dns_question::DnsQuestion {
            &self.question
        }
        pub fn reponse(&self) -> &dns_rr::DnsRR {
            &self.reponse
        }
        pub fn set_header(&mut self, header: dns_header::DnsHeader) {
            self.header = header;
        }
        pub fn set_question(&mut self, question: dns_question::DnsQuestion) {
            self.question = question;
        }
        pub fn set_reponse(&mut self, reponse: dns_rr::DnsRR) {
            self.reponse = reponse;
        }
    }
}
mod dns_rr {
    use crate::dns_question;

    pub struct DnsRR {
        question : dns_question::DnsQuestion,
        ttl : i32 ,
        rdlength : i16 ,
        rdata : u32
    }

    impl DnsRR {
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
}
mod dns_header {
    static mut LIST_ID: Vec<u16> = vec![];
    pub struct DnsHeader {
        id: u16,
        qr: bool,
        opcode: bool,
        aa: bool,
        tc: bool,
        rd: bool,
        ra: bool,
        z: bool,
        rcode: u16,
        qdcount: u16,
        ancount: u16,
        nscount: u16,
        arcount: u16,
    }
    impl DnsHeader {
        pub(crate) fn new(a: bool, b: bool, c: bool, e: bool, f: u16, g: u16, h: u16, i: u16, j: u16) -> Self {
            use rand::Rng;
            //attribution d'un id unique et aléatoire
            let mut temp: u16 = rand::thread_rng().gen();
            unsafe {
                while LIST_ID.contains(&temp) {
                    temp = rand::thread_rng().gen();
                }
            }

            DnsHeader {
                id: temp,
                qr: a,
                opcode: false,
                aa: b,
                tc: c,
                rd: true,
                ra: e,
                z: false,
                rcode: f,
                qdcount: g,
                ancount: h,
                nscount: i,
                arcount: j,
            }
        }
        //get set
        pub fn id(&self) -> u16 {
            self.id
        }
        pub fn qr(&self) -> bool {
            self.qr
        }
        pub fn opcode(&self) -> bool {
            self.opcode
        }
        pub fn aa(&self) -> bool {
            self.aa
        }
        pub fn tc(&self) -> bool {
            self.tc
        }
        pub fn rd(&self) -> bool {
            self.rd
        }
        pub fn ra(&self) -> bool {
            self.ra
        }
        pub fn z(&self) -> bool {
            self.z
        }
        pub fn rcode(&self) -> u16 {
            self.rcode
        }
        pub fn qdcount(&self) -> u16 {
            self.qdcount
        }
        pub fn ancount(&self) -> u16 {
            self.ancount
        }
        pub fn nscount(&self) -> u16 {
            self.nscount
        }
        pub fn arcount(&self) -> u16 {
            self.arcount
        }
        pub fn set_id(&mut self, id: u16) {
            self.id = id;
        }
        pub fn set_qr(&mut self, qr: bool) {
            self.qr = qr;
        }
        pub fn set_opcode(&mut self, opcode: bool) {
            self.opcode = opcode;
        }
        pub fn set_aa(&mut self, aa: bool) {
            self.aa = aa;
        }
        pub fn set_tc(&mut self, tc: bool) {
            self.tc = tc;
        }
        pub fn set_rd(&mut self, rd: bool) {
            self.rd = rd;
        }
        pub fn set_ra(&mut self, ra: bool) {
            self.ra = ra;
        }
        pub fn set_z(&mut self, z: bool) {
            self.z = z;
        }
        pub fn set_rcode(&mut self, rcode: u16) {
            self.rcode = rcode;
        }
        pub fn set_qdcount(&mut self, qdcount: u16) {
            self.qdcount = qdcount;
        }
        pub fn set_ancount(&mut self, ancount: u16) {
            self.ancount = ancount;
        }
        pub fn set_nscount(&mut self, nscount: u16) {
            self.nscount = nscount;
        }
        pub fn set_arcount(&mut self, arcount: u16) {
            self.arcount = arcount;
        }
    }
}

fn main() {
    let header = dns_header::DnsHeader::new(false,false,false,false,1,2,3,4,5);
    println!("{}",header.id());
}
