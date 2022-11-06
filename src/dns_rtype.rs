pub enum DnsRType {
    A,
    AAAA,
    NS,
    CNAME,
    PTR,
    MX,
}
impl DnsRType{
    pub fn no(&self)-> i32{
        match self{
            DnsRType::A => return 1,
            DnsRType::AAAA => return 28,
            DnsRType::CNAME => return 12,
            DnsRType::MX => return 5,
            DnsRType::NS => return 2,
            DnsRType::PTR => return 15

        }
    }
}