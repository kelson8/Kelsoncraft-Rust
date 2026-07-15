use std::net::{IpAddr, Ipv4Addr};


// Testing with IP addresses
// I may try to generate random IPv6 addresses in a range with this.

// https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html

/// Validate an IP address
/// TODO Set this up to take an input.
fn validate_ip_address(ip_address: IpAddr) {
    let main_ip: IpAddr = "127.0.0.1"
        .parse().expect("IP address is not a valid IP address");


    // let is_ipv4 = ip_address.is_ipv4();
    // let is_ipv6 = ip_address.is_ipv6();
}


/// Testing with IP Addresses and vectors.
pub fn ip_address_test() {

    // https://doc.rust-lang.org/book/ch08-01-vectors.html
    // TODO Add error handling to this, I am trying to not make this panic when the IP is invalid.
    let ip_addresses: Vec<IpAddr> = vec![
        "127.0.0.1".parse().expect("IP address is not a valid IP address"),
        "192.168.1.14".parse().expect("IP address is not a valid IP address"),
    ];

    let test_ip = ip_addresses[1];
    if test_ip.is_ipv4() {
        println!("IP: {}", test_ip);
    } else {
        println!("IP is not valid");
    }


}