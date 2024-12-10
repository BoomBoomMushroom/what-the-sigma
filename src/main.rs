use std::{io::Read, net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream}};

// 10.59.209.150:1234

fn man_handle_the_client(stream: TcpStream) {
    //stream.read(read_buf);
}

fn main() {
    let ipv4_address: Vec<u8> = vec![10, 59, 209, 150];
    let ip_address: Ipv4Addr = Ipv4Addr::new(ipv4_address[0], ipv4_address[1], ipv4_address[2], ipv4_address[3]);
    let socket_ip_address: SocketAddr = SocketAddr::V4(SocketAddrV4::new(ip_address, 1234));
    
    let listener = TcpListener::bind(socket_ip_address).unwrap();

    for stream in listener.incoming() {
        man_handle_the_client(stream.unwrap());
    }
}
