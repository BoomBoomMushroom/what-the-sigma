use std::io::Read;
use std::net::{self, IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};

// 10.59.209.150:1234


fn main() {
    let ipv4_address: Vec<u8> = vec![10, 59, 209, 150];
    let ip_address: Ipv4Addr = Ipv4Addr::new(ipv4_address[0], ipv4_address[1], ipv4_address[2], ipv4_address[3]);
    let socket_ip_address: SocketAddr = SocketAddr::V4(SocketAddrV4::new(ip_address, 1234));
    
}
