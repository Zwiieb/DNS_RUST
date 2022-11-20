

use crate::dns_header::DnsHeader;

    use super::*;
    #[test]
    pub fn test_serialize() {

        let mut h = dns_header::DnsHeader::new(false, true, false, false, 0, 1, 0, 0, 0);
        h.set_id(5);
        let v = h.serialize();
        let r = vec![0,5,5,0,0,1,0,0,0,0,0,0 as u8];
        assert_eq!(r,v);
    }

    #[test]
    pub fn test_get_set_id() {
        let mut h = dns_header::DnsHeader::new(false, true, false, false, 0, 1, 0, 0, 0);
        h.set_id(8);
        assert_eq!(h.id(),8);

    }

    #[test]
    pub fn test_get_set_qr( ) {
        let mut h = dns_header::DnsHeader::new(false, true, false, false, 0, 1, 0, 0, 0);
        h.set_qr(true);
        assert_eq!(h.qr(),true);
    }

    #[test]
    pub fn test_get_set_opcode( ) {
        let mut  h = dns_header::DnsHeader::new(false, true, false, false, 0, 1, 0, 0, 0);
        h.set_opcode(true);
        assert_eq!(h.opcode(), true);

    }

    #[test]
    pub fn test_get_set_aa( ) {
        let mut h = dns_header::DnsHeader::new(false, true, false, false, 0, 1, 0, 0, 0);
        h.set_aa(false);
        assert_eq!(h.aa(), false);

    }

    #[test]
    pub fn test_get_set_tc( ) {
        let mut h = dns_header::DnsHeader::new(false, true, false, false, 0, 1, 0, 0, 0);
        h.set_tc(true);
        assert_eq!(h.tc(), true);

    }

    #[test]
    pub fn test_get_set_rd( ) {
        let mut h = dns_header::DnsHeader::new(false, true, false, false, 0, 1, 0, 0, 0);
        h.set_rd(false);
        assert_eq!(h.rd(), false);

    }

    #[test]
    pub fn test_get_set_ra( ) {
        let mut h = dns_header::DnsHeader::new(false, true, false, false, 0, 1, 0, 0, 0);
        h.set_ra(true);
        assert_eq!(h.ra(), true);

    }

    #[test]
    pub fn test_get_set_z( ) {
        let mut h = dns_header::DnsHeader::new(false, true, false, false, 0, 1, 0, 0, 0);
        h.set_z(true);
        assert_eq!(h.z(), true);
    }

    #[test]
    pub fn test_get_set_rcode( ) {
        let mut h = dns_header::DnsHeader::new(false, true, false, false, 0, 1, 0, 0, 0);
        h.set_rcode(1);
        assert_eq!(h.rcode(), 1);

    }

    #[test]
    pub fn test_get_set_qdcount( ) {
        let mut h = dns_header::DnsHeader::new(false, true, false, false, 0, 1, 0, 0, 0);
        h.set_qdcount(0);
        assert_eq!(h.qdcount(),0);

    }

    #[test]
    pub fn test_get_set_ancount()  {
        let mut h = dns_header::DnsHeader::new(false, true, false, false, 0, 1, 0, 0, 0);
        h.set_ancount(1);
        assert_eq!(h.ancount(),1);

    }

    #[test]
    pub fn test_get_set_nscount( ) {
        let mut h = dns_header::DnsHeader::new(false, true, false, false, 0, 1, 0, 0, 0);
        h.set_nscount(1);
        assert_eq!(h.nscount(),1);

    }

    #[test]
    pub fn set_get_arcount( ) {
        let mut h = dns_header::DnsHeader::new(false, true, false, false, 0, 1, 0, 0, 0);
        h.set_arcount(1);
        assert_eq!(h.arcount(),1);

    }


