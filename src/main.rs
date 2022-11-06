extern crate core;

mod dns_rr;
mod tests;
mod dns_header;
mod dns_rtype;
mod dns_question;
mod dns_packet;

fn main(){

    let header = dns_header::DnsHeader::new(false, false, false, false, 1, 2, 3, 4, 5);
    println!("{}", header.id());

    let pac = dns_packet::DnsPacket::new(false, false, false, false, 1, 2, 3, 4, 5,String::from("3"),0x0001,dns_rtype::DnsRType::AAAA);
    println!("{}",pac.reponse()[0].ttl());
    println!("{}",pac.byte_size());
    //let pac = dns_packet::DnsPacket::generate_rr();
}
