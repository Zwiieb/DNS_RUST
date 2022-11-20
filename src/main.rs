extern crate core;

use crate::dns_rtype::DnsRType;

mod dns_rr;
mod dns_header;
mod dns_rtype;
mod dns_question;
mod dns_packet;
mod tests_dns_question;
mod tests_dns_rtype;
mod tests_dns_packet;
mod test_dns_header;
mod test_dns_rr;

fn main(){


    /*
    let header = dns_header::DnsHeader::new(false, false, false, false, 1, 2, 3, 4, 5);
    println!("{}", header.id());
    */
    let q = dns_question::DnsQuestion::new("".to_string(), DnsRType::A, 0x0001);
    let v = q.serialize();
    for i in v{
        println!("{}", i );
    }
    //let pac = dns_packet::DnsPacket::generate_rr();
    /*
    let mut x:i32 = 600;
    let x_b1 = ((x & 0x7F000000) >> 24) as u8;
    let x_b2 = ((x & 0x00FF0000) >> 16) as u8;
    let x_b3 = ((x & 0x0000FF00) >> 8) as u8;
    let x_b4 = ((x & 0x000000FF) >> 0) as u8;
    println!("{},{},{},{}",x_b1,x_b2,x_b3,x_b4);
    println!("{},{},{},{}",x.to_be_bytes()[0],x.to_be_bytes()[1],x.to_be_bytes()[2],x.to_be_bytes()[3]);
    */
}
