extern crate core;

mod dns_rr;
mod dns_header;
mod dns_rtype;
mod dns_question;
mod dns_packet;
mod tests_dns_question;
mod tests_dns_rtype;
mod tests_dns_packet;

fn main(){

    let header = dns_header::DnsHeader::new(false, false, false, false, 1, 2, 3, 4, 5);
    println!("{}", header.id());

    let pac = dns_packet::DnsPacket::new(false, false, false, false, 1, 2, 3, 4, 5, String::from("3"), 0x0001, dns_rtype::DnsRType::AAAA, 0x0001, 0, 0, "".to_string());
    println!("{}",pac.reponse()[0].ttl());
    println!("{}",pac.byte_size());
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
