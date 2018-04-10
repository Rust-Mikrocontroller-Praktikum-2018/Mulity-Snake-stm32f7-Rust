extern crate smoltcp;

use super::alloc::Vec;
use super::stm32f7;
use smoltcp::iface::EthernetInterface;
use smoltcp::socket::{Socket, SocketSet, TcpSocket, TcpSocketBuffer};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address, Ipv4Cidr};
use stm32f7::ethernet;
use stm32f7::ethernet::EthernetDevice;

/**
 *
 */
pub struct Network {
    ethernet_interface: EthernetInterface<'static, 'static, EthernetDevice>,
    // sockets: SocketSet<'static, 'static, 'static>,
    network_mode: NetworkMode,
}

/**
 * Operate as either client or server.
 */
pub enum NetworkMode {
    client,
    server,
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
        let local_eth_addr = EthernetAddress([0x00, 0x08, 0xdc, 0xab, 0xcd, 0xef]);
        let local_ip_addr = Ipv4Address([192, 168, 0, 42]);
        let local_port = 49500_u16;

        let remote_ip_addr = IpAddress::v4(192, 168, 0, 50);
        let remote_port = 50000_u16;

        let tcp_rx_buffer = TcpSocketBuffer::new(vec![0; ethernet::MTU]);
        let tcp_tx_buffer = TcpSocketBuffer::new(vec![0; ethernet::MTU]);
        let mut tcp_socket = TcpSocket::new(tcp_rx_buffer, tcp_tx_buffer);

        let mut ethernet_interface = Network::into_interface(ethernet_device, local_ip_addr, local_eth_addr);

        // tcp_socket.listen(endpoint).unwrap();
        let mut sockets = SocketSet::new(Vec::new());
        let tcp_handle = sockets.add(tcp_socket);

        {
            let mut socket = sockets.get::<TcpSocket>(tcp_handle);
            socket.connect((remote_ip_addr, remote_port), local_port).unwrap();
        }

        sockets.add(tcp_socket);
        Network {
            ethernet_interface: ethernet_interface,
            network_mode: network_mode,
        }
    }

    /**
     * private: creating Interface
     * (Copying)
     * "pub fn into_interface<'a>(self) -> EthernetInterface<'a, 'a, Self>" from stm32f7-discovery::ethernet::EthernetDevice
     */
    fn into_interface<'a>(
        ethernet_device: stm32f7::ethernet::EthernetDevice,
        ip_addr: Ipv4Address,
        eth_addr: EthernetAddress,
    ) -> EthernetInterface<'a, 'a, EthernetDevice> {
        use alloc::BTreeMap;
        use smoltcp::iface::EthernetInterfaceBuilder;
        use smoltcp::iface::NeighborCache;

        let neighbor_cache = NeighborCache::new(BTreeMap::new());
        let ip_cidr = IpCidr::Ipv4(Ipv4Cidr::new(ip_addr, 0));
        EthernetInterfaceBuilder::new(ethernet_device)
            .ethernet_addr(eth_addr)
            .neighbor_cache(neighbor_cache)
            .ip_addrs(vec![ip_cidr])
            // .ipv4_gateway(default_v4_gw)
            .finalize()
    }
}
