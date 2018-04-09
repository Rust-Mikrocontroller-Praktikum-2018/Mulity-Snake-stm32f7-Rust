extern crate smoltcp;

use super::alloc::Vec;
use super::stm32f7;
use core;
use smoltcp::socket::{Socket, SocketSet, TcpSocket, TcpSocketBuffer};
use smoltcp::time::Instant;
use smoltcp::wire::{IpAddress, IpEndpoint};
use stm32f7::ethernet;
use stm32f7::ethernet::IP_ADDR;

pub struct Network {
    ethernet_interface:
        smoltcp::iface::EthernetInterface<'static, 'static, stm32f7::ethernet::EthernetDevice>,
    sockets: smoltcp::socket::SocketSet<'static, 'static, 'static>,
}

impl Network {
    /**
     * Creates a new Network instance.
     */
    pub fn new(
        ethernet_interface: smoltcp::iface::EthernetInterface<
            'static,
            'static,
            stm32f7::ethernet::EthernetDevice,
        >,
    ) -> Network {
        let mut sockets = SocketSet::new(Vec::new());
        // pub const IP_ADDR: Ipv4Address = Ipv4Address([141, 52, 46, 198]);
        let endpoint = smoltcp::wire::IpEndpoint::new(smoltcp::wire::IpAddress::Unspecified, 12000_u16);
        // let endpoint = IpEndpoint::new(IpAddress::Ipv4(IP_ADDR), 15);

        let tcp_rx_buffer = TcpSocketBuffer::new(vec![0; ethernet::MTU]);
        let tcp_tx_buffer = TcpSocketBuffer::new(vec![0; ethernet::MTU]);
        let mut example_tcp_socket = TcpSocket::new(tcp_rx_buffer, tcp_tx_buffer);
        example_tcp_socket.listen(endpoint).unwrap();
        sockets.add(example_tcp_socket);
        Network {
            ethernet_interface: ethernet_interface,
            sockets: sockets,
        }
    }
}
