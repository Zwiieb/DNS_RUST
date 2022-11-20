use crate::{dns_header, dns_packet, dns_question};
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
    assert_eq!(p.header().rcode(),20);
}
#[test]
fn test_get_set_question(){
    let q = dns_question::DnsQuestion::new("".to_string(), DnsRType::AAAA, 0x0001);
    let mut p = dns_packet::DnsPacket::new(false, false, false, false, 0, 0, 0, 0, 0, "".to_string(), 0, DnsRType::A, 0x0001, 0, 0, "".to_string());
    p.set_question(q);
    assert_eq!(p.question().qtype().no(),28);
}
#[test]
fn test_get_set_reponse(){
    let r = vec![DnsRR::new(0, DnsRType::A, 0, 25, 0, "".to_string())];
    let mut p = dns_packet::DnsPacket::new(false, false, false, false, 0, 0, 0, 0, 0, "".to_string(), 0, DnsRType::A, 0x0001, 0, 0, "".to_string());
    p.set_reponse(r);
    assert_eq!(p.reponse()[0].ttl(),25);
}
