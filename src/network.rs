// ToDo ...
let mut ethernet_interface = ethernet::EthernetDevice::new(
        Default::default(),
        Default::default(),
        rcc,
        syscfg,
        &mut gpio,
        ethernet_mac,
        ethernet_dma,
    ).map(|device| device.into_interface());
    if let Err(e) = ethernet_interface {
        println!("ethernet init failed: {:?}", e);
    };

    let mut sockets = SocketSet::new(Vec::new());
    // pub const IP_ADDR: Ipv4Address = Ipv4Address([141, 52, 46, 198]);
    let endpoint = IpEndpoint::new(IpAddress::Ipv4(IP_ADDR), 15);

    let tcp_rx_buffer = TcpSocketBuffer::new(vec![0; ethernet::MTU]);
    let tcp_tx_buffer = TcpSocketBuffer::new(vec![0; ethernet::MTU]);
    let mut example_tcp_socket = TcpSocket::new(tcp_rx_buffer, tcp_tx_buffer);
    example_tcp_socket.listen(endpoint).unwrap();
    sockets.add(example_tcp_socket);