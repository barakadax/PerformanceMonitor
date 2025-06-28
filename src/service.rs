use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::Packet;
use pnet::util::MacAddr;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::ipv6::Ipv6Packet;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let interfaces: Vec<datalink::NetworkInterface> = datalink::interfaces();
    println!("Available interfaces:");
    for iface in &interfaces {
        println!("  {}: mac={:?} (is_up={}, is_broadcast={}, is_loopback={})", iface.name, iface.mac, iface.is_up(), iface.is_broadcast(), iface.is_loopback());
    }

    let interface: datalink::NetworkInterface = interfaces
        .into_iter()
        .find(|iface| iface.mac.is_some() && !iface.is_loopback())
        .expect("No suitable interface with MAC address found");

    let mut config: datalink::Config = datalink::Config::default();
    config.promiscuous = true;
    let (_, mut rx) = match datalink::channel(&interface, config) {
        Ok(Ethernet(_tx, rx)) => (_tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Failed to create datalink channel: {}", e),
    };

    println!("Listening on {}", interface.name);

    let mac: MacAddr = interface.mac.unwrap_or(MacAddr::zero());

    println!("{} local mac", mac);

    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(eth_packet) = EthernetPacket::new(packet) {
                    let direction = if eth_packet.get_destination() == mac {
                        "Incoming"
                    } else if eth_packet.get_source() == mac {
                        "Outgoing"
                    } else {
                        "Unknown"
                    };
                    let payload = eth_packet.payload();
                    let mut port_info: String = String::from("N/A");
                    let mut msg_size = payload.len();
                    let mut ip_version = "Other";
                    // Try IPv4 first
                    if let Some(ipv4) = Ipv4Packet::new(payload) {
                        ip_version = "IPv4";
                        match ipv4.get_next_level_protocol() {
                            IpNextHeaderProtocols::Tcp => {
                                if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
                                    port_info = format!("TCP src_port={}, dst_port={}", tcp.get_source(), tcp.get_destination());
                                    msg_size = tcp.payload().len();
                                }
                            }
                            IpNextHeaderProtocols::Udp => {
                                if let Some(udp) = UdpPacket::new(ipv4.payload()) {
                                    port_info = format!("UDP src_port={}, dst_port={}", udp.get_source(), udp.get_destination());
                                    msg_size = udp.payload().len();
                                }
                            }
                            _ => {
                                port_info = format!("{}", ipv4.get_next_level_protocol());
                                msg_size = ipv4.payload().len();
                            }
                        }
                    } else if let Some(ipv6) = Ipv6Packet::new(payload) {
                        ip_version = "IPv6";
                        match ipv6.get_next_header() {
                            IpNextHeaderProtocols::Tcp => {
                                if let Some(tcp) = TcpPacket::new(ipv6.payload()) {
                                    port_info = format!("TCPv6 src_port={}, dst_port={}", tcp.get_source(), tcp.get_destination());
                                    msg_size = tcp.payload().len();
                                }
                            }
                            IpNextHeaderProtocols::Udp => {
                                if let Some(udp) = UdpPacket::new(ipv6.payload()) {
                                    port_info = format!("UDPv6 src_port={}, dst_port={}", udp.get_source(), udp.get_destination());
                                    msg_size = udp.payload().len();
                                }
                            }
                            _ => {
                                port_info = format!("{}", ipv6.get_next_header());
                                msg_size = ipv6.payload().len();
                            }
                        }
                    }
                    println!("{} packet: {} protocol={} size={} bytes | {:?}", ip_version, direction, port_info, msg_size, eth_packet);
                } else {
                    println!("Received non-Ethernet packet: {:?}", packet);
                }
            }
            Err(e) => {
                println!("An error occurred while reading: {}", e);
            }
        }
    }
}
