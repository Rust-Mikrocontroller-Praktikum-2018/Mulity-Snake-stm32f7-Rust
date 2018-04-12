extern crate smoltcp;

use alloc::Vec;
use smoltcp::iface::EthernetInterface;
// use smoltcp::phy::wait as phy_wait;
use smoltcp::socket::{Socket, SocketSet, TcpSocket, TcpSocketBuffer};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address, Ipv4Cidr};
use stm32f7::ethernet;
use stm32f7::ethernet::EthernetDevice;
use stm32f7::system_clock;

/**
 *
 */
pub struct Network {
    ethernet_interface: EthernetInterface<'static, 'static, EthernetDevice>,
    // sockets: SocketSet<'static, 'static, 'static>,
    network_mode: NetworkMode,
    sockets: SocketSet<'static, 'static, 'static>, // Todo FRAGE HIER <---------------------------------------------------------
    tcp_handle: smoltcp::socket::SocketHandle,
    tcp_active: bool,
}

/**
 * Operate as either client or server.
 */
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
        let local_ip_addr = Ipv4Address([192, 168, 0, 42]);

        let remote_ip_addr = IpAddress::v4(192, 168, 0, 50);
        let remote_port = 50000_u16;

        let tcp_rx_buffer = TcpSocketBuffer::new(vec![0; ethernet::MTU]);
        let tcp_tx_buffer = TcpSocketBuffer::new(vec![0; ethernet::MTU]);
        let mut tcp_socket = TcpSocket::new(tcp_rx_buffer, tcp_tx_buffer);

        let mut ethernet_interface = ethernet_device.into_interface(local_ip_addr);

        // tcp_socket.listen(endpoint).unwrap();
        let mut sockets = SocketSet::new(Vec::new());
        let tcp_handle = sockets.add(tcp_socket);

        Network {
            ethernet_interface: ethernet_interface,
            network_mode: network_mode,
            sockets: sockets,
            tcp_handle: tcp_handle,
            tcp_active: false,
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

    fn print_data_as_char(data: &[u8]) {
        if data.len() > 0 {
            // data = data.split(|&b| b == b'\n').collect::<Vec<_>>().concat();
            hprint!("Data received: ");
            for x in data {
                hprint!("{}", *x as char);
            }
            hprintln!("");
        }
    }

    fn operate_server(&mut self) {
        // let timestamp = Instant::now();
        let timestamp = Instant::from_millis(system_clock::ticks() as i64);
        self.ethernet_interface
            .poll(&mut self.sockets, timestamp)
            .expect("poll error");

        // tcp:6969: respond "hello"
        {
            let mut socket = self.sockets.get::<TcpSocket>(self.tcp_handle);
            if !socket.is_open() {
                socket.listen(6969).unwrap();
            }

            if socket.is_active() && !self.tcp_active {
                hprintln!("tcp:6970 connected");
            } else if !socket.is_active() && self.tcp_active {
                hprintln!("tcp:6970 disconnected");
            }
            self.tcp_active = socket.is_active();

            if socket.may_recv() {
                let data = socket
                    .recv(|buffer| {
                        //FRAGE <---------------------------------------------
                        let mut data = buffer;
                        Network::print_data_as_char(data);
                        (data.len(), data)
                    })
                    .unwrap();
                if socket.can_send() && data.len() > 0 {
                    hprintln!("tcp:6969 send greeting");
                    // write!(socket, "ping\n").unwrap();
                    let ping_slice = vec!['p' as u8, 'i' as u8, 'n' as u8, 'g' as u8].as_slice();
                    socket.send_slice(ping_slice).unwrap();
                    hprintln!("tcp:6969 close");
                    socket.close();
                }
            } else if socket.may_send() {
                hprintln!("tcp:6970 close");
                socket.close();
            }
        }
        // phy_wait(fd, iface.poll_delay(&sockets, timestamp)).expect("wait error");
    }

    fn operate_client(&mut self) {
        let mut socket = self.sockets.get::<TcpSocket>(self.tcp_handle);
        {
            if !socket.is_open() {
                let remote_ip_addr = IpAddress::v4(192, 168, 0, 50);
                let remote_port = 50000_u16;
                let local_port = 49500_u16;
                socket
                    .connect((remote_ip_addr, remote_port), local_port)
                    .unwrap();
            }
        }

        {
            // let timestamp = Instant::now();
            let timestamp = Instant::from_millis(system_clock::ticks() as i64);
            self.ethernet_interface
                .poll(&mut self.sockets, timestamp)
                .expect("poll error");
        }

        {
            // let mut socket = self.sockets.get::<TcpSocket>(self.tcp_handle); // Moved up
            if socket.is_active() && !self.tcp_active {
                hprintln!("connected");
            } else if !socket.is_active() && self.tcp_active {
                hprintln!("disconnected");
                // break;
                panic!("Abort! - Disconnected");
            }
            self.tcp_active = socket.is_active();

            if socket.may_recv() {
                /* let data =  */
                socket
                    .recv(|data| {
                        // let mut data = data.to_owned();
                        let mut data = data;
                        Network::print_data_as_char(data);
                        (data.len(), data)
                    })
                    .unwrap();
                if socket.can_send()
                /* && data.len() > 0 */
                {
                    hprintln!("send data: pong");
                    // socket.send_slice(&data[..]).unwrap();
                    let pong_slice = vec!['p' as u8, 'o' as u8, 'n' as u8, 'g' as u8].as_slice();
                    socket.send_slice(pong_slice).unwrap();
                    // write!(socket, "pong\n").unwrap();
                }
            } else if socket.may_send() {
                // (könnte) may send -> line is empty -> nothing to listen?
                hprintln!("close");
                socket.close();
            }
        }

        // phy_wait(fd, self.ethernet_interface.poll_delay(&self.sockets, timestamp)).expect("wait error");
    }

    // /**
    //  * private: creating Interface
    //  * (Copying)
    //  * "pub fn into_interface<'a>(self) -> EthernetInterface<'a, 'a, Self>" from stm32f7-discovery::ethernet::EthernetDevice
    //  */
    // fn into_interface<'a>(
    //     ethernet_device: stm32f7::ethernet::EthernetDevice,
    //     ip_addr: Ipv4Address,

    // ) -> EthernetInterface<'a, 'a, EthernetDevice> {
    //     use alloc::BTreeMap;
    //     use smoltcp::iface::EthernetInterfaceBuilder;
    //     use smoltcp::iface::NeighborCache;

    //     let neighbor_cache = NeighborCache::new(BTreeMap::new());
    //     let ip_cidr = IpCidr::Ipv4(Ipv4Cidr::new(ip_addr, 0));
    //     EthernetInterfaceBuilder::new(ethernet_device)
    //         .ethernet_addr(eth_addr)
    //         .neighbor_cache(neighbor_cache)
    //         .ip_addrs(vec![ip_cidr])
    //         // .ipv4_gateway(default_v4_gw)
    //         .finalize()
    // }
}
