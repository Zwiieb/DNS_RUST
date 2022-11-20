use crate::dns_rr;
use crate::dns_rtype::DnsRType;

#[test]
    pub fn test_get_size() {
        let h = dns_rr::DnsRR::new(0,DnsRType::PTR,1, 2,0,"test".to_string());
        assert_eq!(h.get_size(), 16) ;
    }

    #[test]
    pub fn test_serialize() {

        let h = dns_rr::DnsRR::new(1,DnsRType::A,1, 5,0,"test".to_string());
        let r = h.serialize();
        let v =vec![0 as u8, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 5, 0, 0, 116, 101, 115, 116];
        assert_eq!(r,v);
    }

    #[test]
    pub fn test_get_set_ttl() {
        let mut h = dns_rr::DnsRR::new(1,DnsRType::A,1, 5,0,"test".to_string());
        h.set_ttl(8);
        assert_eq!(h.ttl(),8);

    }

    #[test]
    pub fn test_get_set_rdlength() {
        let mut h = dns_rr::DnsRR::new(1,DnsRType::A,1, 5,0,"test".to_string());
        h.set_rdlength(1);
        assert_eq!(h.rdlength(),1);

    }

    #[test]
    pub fn test_get_set_rdata() {
        let mut h = dns_rr::DnsRR::new(1,DnsRType::A,1, 5,0,"test".to_string());
        h.set_rdata("changement".to_string());
        assert_eq!(h.rdata(), "changement".to_string());

    }

    #[test]
    pub fn test_get_set_rname() {
        let mut h = dns_rr::DnsRR::new(1,DnsRType::A,1, 5,0,"test".to_string());
        h.set_rname(0);
        assert_eq!(h.rname(), 0);

    }

    #[test]
    pub fn test_get_set_DnsRType() {
        let mut h = dns_rr::DnsRR::new(1,DnsRType::A,1, 5,0,"test".to_string());
        h.set_Dnsrtype(DnsRType::PTR);
        assert!(matches!(h.Dnsrtype(),DnsRType::PTR));

    }

    #[test]
    pub fn test_get_set_rclass() {
        let mut h = dns_rr::DnsRR::new(1,DnsRType::A,1, 5,0,"test".to_string());
        h.set_rclass(0);
        assert_eq!(h.rclass(),0);

    }
