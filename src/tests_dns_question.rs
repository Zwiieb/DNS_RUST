use crate::dns_question::DnsQuestion;
use crate::{dns_question, dns_rtype};
use crate::dns_rtype::DnsRType;

#[test]
fn test_new(){
    let question = dns_question::DnsQuestion::new(String::from("3"),dns_rtype::DnsRType::AAAA,0x0001);
}
#[test]
#[should_panic]
fn test_with_bad_class(){
    DnsQuestion::new(String::from("3"),dns_rtype::DnsRType::AAAA,0x0000);
}
#[test]
fn test_get_size(){
    let q = DnsQuestion::new(String::from("test"),DnsRType::A,0x0001);
    assert_eq!(q.get_size(),String::from("test").len()+4);
}
#[test]
fn test_get_set_qname(){
    let mut q = DnsQuestion::new(String::from("test"), DnsRType::A, 0x0001);
    q.set_qname(String::from("what"));
    assert_eq!(q.qname(),String::from("what"));
}
#[test]
fn test_no_qtype(){
    let mut q = DnsQuestion::new(String::from("test"), DnsRType::A, 0x0001);
    q.set_qtype(dns_rtype::DnsRType::AAAA);
    assert_eq!(q.qtype().no(),28);
}
fn test_get_set_qtype(){
    let mut q = DnsQuestion::new(String::from("test"), DnsRType::A, 0x0001);
    q.set_qtype(dns_rtype::DnsRType::AAAA);
    assert!(matches!(q.qtype(),DnsRType::AAAA));
}
#[test]
fn test_get_qclass(){
    let q =DnsQuestion::new(String::from("3"),dns_rtype::DnsRType::AAAA,0x0001);
    assert_eq!(0x0001,q.qclass());
}
#[test]
#[should_panic]
fn test_set_qclass(){
    let mut q =DnsQuestion::new(String::from("3"),dns_rtype::DnsRType::AAAA,0x0001);
    q.set_qclass(0x0000);
}

#[test]
fn test_serialise(){
    let q = dns_question::DnsQuestion::new("".to_string(), DnsRType::A, 0x0001);
}
