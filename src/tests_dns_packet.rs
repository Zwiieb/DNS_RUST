use crate::{dns_header, dns_packet, dns_question, dns_rtype};
use crate::dns_rr::DnsRR;
use crate::dns_rtype::DnsRType;

#[test]
fn test_new(){
    dns_packet::DnsPacket::new(false, false, false, false, 0, 0, 0, 0, 0, "".to_string(), 0, DnsRType::A, 0x0001, 0, 0, "".to_string());
}
#[test]
fn test_get_set_header(){
    let h = dns_header::DnsHeader::new(false, false, false, false, 20, 0, 0, 0, 0);
    let mut p = dns_packet::DnsPacket::new(false, false, false, false, 0, 0, 0, 0, 0, "".to_string(), 0, DnsRType::A, 0x0001, 0, 0, "".to_string());
    p.set_header(h);
    assert!(matches!(p.header(),h));
}
#[test]
fn test_get_set_question(){
    let q = dns_question::DnsQuestion::new("".to_string(), DnsRType::AAAA, 0x0001);
    let mut p = dns_packet::DnsPacket::new(false, false, false, false, 0, 0, 0, 0, 0, "".to_string(), 0, DnsRType::A, 0x0001, 0, 0, "".to_string());
    p.set_question(q);
    assert!(matches!(p.question(),q));
}
#[test]
fn test_get_set_reponse(){
    let r = vec![DnsRR::new(0, DnsRType::A, 0, 25, 0, "".to_string())];
    let mut p = dns_packet::DnsPacket::new(false, false, false, false, 0, 0, 0, 0, 0, "".to_string(), 0, DnsRType::A, 0x0001, 0, 0, "".to_string());
    p.set_reponse(r);
    assert!(matches!(p.reponse(),r));
}
#[test]
fn test_byte_size(){
    let pac = dns_packet::DnsPacket::new(false, false, false, false, 1, 2, 3, 4, 5, String::from("3"), 0x0001, dns_rtype::DnsRType::AAAA, 0x0001, 0, 0, "".to_string());
    assert_eq!(161,pac.byte_size())
}
#[test]
fn test_serialize(){
    let mut pac = dns_packet::DnsPacket::new(false, false, false, false, 1, 2, 3, 4, 5, String::from("3"), 0x0001, dns_rtype::DnsRType::AAAA, 0x0001, 0, 0, "".to_string());
    pac.header().set_id(3);
    let r = pac.serialize();
    let v = vec![0 as u8,3,1,1,0,2,0,3,0,4,0,5,51,0,0,0,0,28,0,1,0,1,0,0,0,28,0,1,0,0,0,0,0,0,0,1,0,0,0,28,0,1,0,0,0,0,0,0,0,1,0,0,0,28,0,1,0,0,0,0,0,0,0,1,0,0,0,28,0,1,0,0,0,0,0,0,0,1,0,0,0,28,0,1,0,0,0,0,0,0,0,1,0,0,0,28,0,1,0,0,0,0,0,0,0,1,0,0,0,28,0,1,0,0,0,0,0,0,0,1,0,0,0,28,0,1,0,0,0,0,0,0,0,1,0,0,0,28,0,1,0,0,0,0,0,0,0,1,0,0,0,28,0,1,0,0,0,0,0,0,0,1,0,0,0,28,0,1,0,0,0,0,0,0,0,1,0,0,0,28,0,1,0,0,0,0,0,0];
    assert_eq!(r,v)
}