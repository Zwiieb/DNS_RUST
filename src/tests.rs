use super::*;

#[test]
fn test_dns_rtype_aaaa() {
    assert_eq!(dns_rtype::DnsRType::AAAA.no(), 28);
}

#[test]
fn test_dns_question_class(){
    let question = dns_question::DnsQuestion::new(String::from("3"),dns_rtype::DnsRType::AAAA,0x0001);
}

#[test]
#[should_panic]
fn test_dns_question_with_bad_class(){
    dns_question::DnsQuestion::new(String::from("3"),dns_rtype::DnsRType::AAAA,0x0001);
}
#[test]
fn test_set(){
    let mut header = dns_header::DnsHeader::new(false, false, false, false, 1, 2, 3, 4, 5);
    if header.id() != 1{
        header.set_id(1);
        assert_eq!(header.id(),1);
    }else{
        header.set_id(2);
        assert_eq!(header.id(),2);
    }
}
#[test]
#[should_panic]
fn test_tc(){
    dns_packet::DnsPacket::new(false, false, true, false, 1, 2, 3, 4, 5,String::from("3"),0x0001,dns_rtype::DnsRType::AAAA);
}