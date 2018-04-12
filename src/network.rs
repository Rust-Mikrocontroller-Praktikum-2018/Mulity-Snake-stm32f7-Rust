extern crate smoltcp;

use alloc::borrow::ToOwned;
use alloc::Vec;
use smoltcp::iface::EthernetInterface;
// use smoltcp::phy::wait as phy_wait;
use smoltcp::socket::{Socket, SocketSet, UdpPacketMetadata, UdpSocket, UdpSocketBuffer};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpEndpoint, Ipv4Address};
use stm32f7::ethernet::EthernetDevice;
use stm32f7::system_clock;

/**
 * How to use:
  
    let mut network;
    // Todo: random EthernetAddress: FRAGE How to use random_gen here?
    let eth_addr = network::Network::get_ethernet_addr(network::NetworkMode::Client);
    let ethernet_device = ethernet::EthernetDevice::new(
        Default::default(),
        Default::default(),
        rcc,
        syscfg,
        &mut gpio,
        ethernet_mac,
        ethernet_dma,
        eth_addr,
    );
    match ethernet_device {
        Ok(ether_device) => network = network::Network::new(ether_device, network::NetworkMode::Client),
        Err(e) => panic!("error parsing ethernet_device: {:?}", e),
    }
  
 * then use network inside a loop()
 */
pub struct Network {
    ethernet_interface: EthernetInterface<'static, 'static, EthernetDevice>,
    network_mode: NetworkMode,
    sockets: SocketSet<'static, 'static, 'static>,
    target_endpoint: IpEndpoint,
}

/**
 * Operate as either client or server.
 */
#[derive(Copy, Clone)]
pub enum NetworkMode {
    Client,
    Server,
}

/**
 *
 */
impl Network {
    /**
     * Creates a new Network instance.
     */
    pub fn new(ethernet_device: EthernetDevice, network_mode: NetworkMode) -> Network {
        // Todo: Automatically choose ip/eth (maybe check if already there, or random?)
        let local_ip_addr: Ipv4Address;
        let local_port: u16;
        let target_ip_addr: Ipv4Address;
        let target_port: u16;
        match network_mode {
            NetworkMode::Server => {
                local_ip_addr = Ipv4Address([192, 168, 0, 24]);
                local_port = 2424;
                // target_ip_addr = Ipv4Address([192, 168, 0, 42]);
                // target_port = 4242;
                target_ip_addr = Ipv4Address([192, 168, 0, 50]);
                target_port = 5050;
            }
            NetworkMode::Client => {
                local_ip_addr = Ipv4Address([192, 168, 0, 42]);
                local_port = 4242;
                target_ip_addr = Ipv4Address([192, 168, 0, 24]);
                target_port = 2424;
            }
        }
        let local_endpoint = IpEndpoint::new(IpAddress::Ipv4(local_ip_addr), local_port);
        let target_endpoint = IpEndpoint::new(IpAddress::Ipv4(target_ip_addr), target_port);

        let ethernet_interface = ethernet_device.into_interface(local_ip_addr);

        let udp_rx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY; 3], vec![0u8; 256]);
        let udp_tx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY; 1], vec![0u8; 256]);
        let mut udp_socket = UdpSocket::new(udp_rx_buffer, udp_tx_buffer);
        udp_socket.bind(local_endpoint).unwrap();

        // tcp_socket.listen(endpoint).unwrap();
        let mut sockets = SocketSet::new(Vec::new());
        /* let tcp_handle =  */
        sockets.add(udp_socket);

        Network {
            ethernet_interface: ethernet_interface,
            network_mode: network_mode,
            sockets: sockets,
            target_endpoint: target_endpoint,
        }
    }

    /**
     *
     */
    pub fn get_ethernet_addr(network_mode: NetworkMode) -> EthernetAddress {
        match network_mode {
            NetworkMode::Client => EthernetAddress([0x00, 0x08, 0xdc, 0xab, 0xcd, 0xef]),
            NetworkMode::Server => EthernetAddress([0x00, 0x08, 0xdc, 0xab, 0xcd, 0xf0]),
        }
    }

    /**
     * Run this inside a loop()
     */
    pub fn operate(&mut self) {
        match self.network_mode {
            NetworkMode::Client => self.operate_client(),
            NetworkMode::Server => self.operate_server(),
        }
    }

    /**
     *
     */
    fn hprintln_data_as_char(data: &mut Vec<u8>) {
        if data.len() > 0 {
            // data = data.split(|&b| b == b'\n').collect::<Vec<_>>().concat();
            hprint!("Data: ");
            for x in data {
                hprint!("{}", *x as char);
            }
            hprintln!("");
        }
    }

    fn operate_server(&mut self) {
        let data = "Hello".as_bytes();
        self.send(data);
    }

    fn operate_client(&mut self) {
        match self.recv() {
            None => {}
            Some(x) => {
                let mut x_copy = x.to_owned();
                Network::hprintln_data_as_char(&mut x_copy);
            }
        }
    }

    /**
     * Receive bytes from udp socket.
     */
    pub fn recv(&mut self) -> Option<Vec<u8>> {
        let timestamp = Instant::from_millis(system_clock::ticks() as i64);
        match self.ethernet_interface.poll(&mut self.sockets, timestamp) {
            Err(::smoltcp::Error::Exhausted) => None,
            Err(::smoltcp::Error::Unrecognized) => None,
            Err(e) => {
                println!("Network error: {:?}", e);
                None
            }
            Ok(socket_changed) => {
                if socket_changed {
                    for mut socket in self.sockets.iter_mut() {
                        return Network::poll_socket(&mut socket).expect("socket poll failed");
                    }
                } else {
                    return None;
                }
                None
            }
            _ => None,
        }
    }

    fn poll_socket(socket: &mut Socket) -> Result<Option<Vec<u8>>, smoltcp::Error> {
        match socket {
            &mut Socket::Udp(ref mut socket) => match socket.endpoint().port {
                // Client Port
                4242_u16 => match socket.recv() {
                    Ok((data, _remote_endpoint)) => Ok(Some(Vec::from(data))),
                    Err(smoltcp::Error::Exhausted) => Ok(None),
                    Err(err) => Err(err),
                },
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }

    /**
     * Send bytes to udp socket.
     */
    pub fn send(&mut self, data: &[u8]) {
        for mut socket in self.sockets.iter_mut() {
            Network::send_data(&mut socket, self.target_endpoint, data);
        }
    }

    fn send_data(socket: &mut Socket, target_endpoint: IpEndpoint, data: &[u8]) {
        match socket {
            &mut Socket::Udp(ref mut socket) => {
                let mut data_copy = data.to_owned();
                Network::hprintln_data_as_char(&mut data_copy);
                match socket.send_slice(data, target_endpoint) {
                    Err(e) => hprint!("Err: {:?}", e),
                    Ok(_) => (),
                }
                // Did not work -> Network was alway exhausted.
                /*
                let mut could_send = false;
                let mut counter: usize = 0;
                let retrys = 10; // try 10 times before giving up.
                while (!could_send) && (counter < retrys) {
                    match socket.send_slice(data, target_endpoint) {
                        Err(e) => {
                            counter += 1;
                            if counter >= retrys {
                                println!("Error: {:?}!", e);
                            }
                        },
                        Ok(_) => could_send = true,
                    }
                }
                */
            }
            _ => {}
        }
    }
}
