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
    pub fn deserialize(vec : Vec<u8>)-> Vec<DnsRR>{
        let mut res = Vec::new() as Vec<DnsRR>;
        let mut temp = Vec::new() as Vec<u8>;
        for i in 0..vec.len() as usize{
            if vec[i] == 0xc0{
                if !(temp.is_empty()){
                    println!("creation de rr");
                    //create rr and push to res
                    let name = ((temp[0] as u16) << 8) | temp[1] as u16;
                    let mut rrtype = ((temp[2] as u16) << 8) | temp[3] as u16;
                    let mut rtype = DnsRType::A;
                    match rrtype{
                        1 => rtype = DnsRType::A,
                        28 => rtype = DnsRType::AAAA,
                        2 => rtype = DnsRType::NS,
                        5 => rtype = DnsRType::CNAME,
                        12 => rtype = DnsRType::PTR,
                        15 => rtype = DnsRType::MX,
                        _=> rrtype=1
                    }
                    let class = ((temp[4] as u16) << 8) | temp[5] as u16;
                    let ttl = ((temp[6] as u32) <<24) | ((temp[7] as u32) <<16) | ((temp[8] as u32) <<8) | temp[9] as u32;
                    let length = ((temp[10] as u16) << 8) | temp[11] as u16;
                    let mut strvec = Vec::new() as Vec<u8>;
                    for i in 12..temp.len() as usize{
                        strvec.push(temp[i]);
                    }
                    let rdata = match String::from_utf8(strvec) {
                        Ok(v) => v,
                        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    };
                    res.push(DnsRR { rname: name, Dnsrtype: rtype, rclass: class, ttl: ttl, rdlength: length, rdata: rdata });
                    temp.clear();
                }
            }
            temp.push(vec[i]);
        }
        let name = ((temp[0] as u16) << 8) | temp[1] as u16;
        let mut rrtype = ((temp[2] as u16) << 8) | temp[3] as u16;
        let mut rtype = DnsRType::A;
        match rrtype{
            1 => rtype = DnsRType::A,
            28 => rtype = DnsRType::AAAA,
            2 => rtype = DnsRType::NS,
            5 => rtype = DnsRType::CNAME,
            12 => rtype = DnsRType::PTR,
            15 => rtype = DnsRType::MX,
            _=> rrtype=1
        }
        let class = ((temp[4] as u16) << 8) | temp[5] as u16;
        let ttl = ((temp[6] as u32) <<24) | ((temp[7] as u32) <<16) | ((temp[8] as u32) <<8) | temp[9] as u32;
        let length = ((temp[10] as u16) << 8) | temp[11] as u16;
        let mut strvec = Vec::new() as Vec<u8>;
        for i in 12..temp.len() as usize{
            strvec.push(temp[i]);
        }
        let rdata = match String::from_utf8(strvec) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        res.push(DnsRR { rname: name, Dnsrtype: rtype, rclass: class, ttl: ttl, rdlength: length, rdata: rdata });
        temp.clear();
        //create rr and push tu res
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
    pub fn rname(&self) -> u16 {
        self.rname
    }
    pub fn Dnsrtype(&self) -> DnsRType {
        self.Dnsrtype
    }
    pub fn rclass(&self) -> u16 {
        self.rclass
    }
    pub fn set_rname(&mut self, rname: u16) {
        self.rname = rname;
    }
    pub fn set_Dnsrtype(&mut self, Dnsrtype: DnsRType) {
        self.Dnsrtype = Dnsrtype;
    }
    pub fn set_rclass(&mut self, rclass: u16) {
        self.rclass = rclass;
    }
}