use crate::dns_rtype;

#[test]
fn test_no() {
    assert_eq!(dns_rtype::DnsRType::AAAA.no(), 28);
}
