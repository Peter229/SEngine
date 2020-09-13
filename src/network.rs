use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv4Addr};
use std::io::{self, Read, Error, stdin, stdout, Write, ErrorKind};

pub struct Network_Packet {

    pub recieve: bool,
    pub buffer: [u8; 16],
    pub from_socket: SocketAddr,
}

pub struct Network {

    socket: UdpSocket,
}

impl Network {

    pub fn quick_new() -> Network {

        let socket = UdpSocket::bind("127.0.0.1:3400").expect("couldn't bind to address");
        socket.set_nonblocking(true).expect("Failed to enter non-blocking mode");

        Network { socket }
    }

    pub fn new() -> Network {

        let mut s=String::new();
        print!("Please enter your ip: ");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");

        let socket = UdpSocket::bind(s.trim_right()).expect("couldn't bind to address");
        socket.set_nonblocking(true).expect("Failed to enter non-blocking mode");

        Network { socket }
    }

    pub fn send_inputs(&self, keys: &[u8; 16], socket: SocketAddr) {
        
        self.socket.send_to(keys, socket).expect("couldn't send message");
    }

    pub fn recieve(&self) -> Network_Packet {

        let mut buff = [0; 16];

        let mut network_packet = Network_Packet { recieve: false, buffer: [0; 16], from_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3400 ) };
    
        let result = self.socket.recv_from(&mut buff);
        match result {
            Ok((num_bytes, from)) => {
                //println!("{}", from);
                network_packet.recieve = true;
                network_packet.buffer = buff;
                network_packet.from_socket = from;
            },
            Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
                println!("Something went wrong: {}", err)
            }
            _ => {}
        }
    
        network_packet
    }
}