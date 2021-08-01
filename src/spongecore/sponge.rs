use std::fmt;
use std::collections::HashSet;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver, TryRecvError};
use pnet::{packet,datalink,util};
use packet::Packet;

pub use crate::spongecore::config;

pub type BoxedResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type ArpTuple = (util::MacAddr, std::net::Ipv4Addr, std::net::Ipv4Addr);


#[derive(Debug)]
pub struct GenericSpongeError(String);

impl std::error::Error for GenericSpongeError {}

impl fmt::Display for GenericSpongeError {
    fn fmt(&self, f:  &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<std::io::Error> for GenericSpongeError {
    fn from(e: std::io::Error) -> Self {
        GenericSpongeError(e.to_string())
    }
}

#[derive(Debug)]
pub struct Sponge {
    iface: datalink::NetworkInterface,
}

impl Sponge {

    pub fn new(config: config::Config) -> BoxedResult<Sponge> {

        Ok(Sponge{
            iface: datalink::interfaces().into_iter()
                                            .filter(|i: &datalink::NetworkInterface| i.name == config.iface)
                                            .next()
                                            .ok_or(GenericSpongeError(String::from("specified interface not found")))?,
        })
    }

    pub fn run(&self) -> BoxedResult<()> {

        let (ctx, crx): (Sender<Option<ArpTuple>>, Receiver<Option<ArpTuple>>) = mpsc::channel();

        let (mut tx, mut rx) = match datalink::channel(&(self.iface), Default::default()) {
            Ok(datalink::Channel::Ethernet(tx, rx)) => Ok((tx, rx)),
            Ok(_) => Err(GenericSpongeError(String::from("unhandled channel type"))),
            Err(e)  => Err(e.into()),
        }?;
        
        let my_mac = self.iface.mac.unwrap(); // So that I can move it into the thread :D  There is literally no reason to use an Arc to pass self.

        let spewer = thread::spawn(move || {            

            let mut macs: HashSet<ArpTuple> = HashSet::new();
            loop {
                if let Some(arp_tuple) = match crx.try_recv() {
                   Ok(m) => m,
                   Err(TryRecvError::Empty) => None,
                   Err(_) => return (),
                }
                {
                   macs.insert(arp_tuple);
                   println!("New MAC looking for {:?}!!!! Hi, Friend {:?} !!!!", arp_tuple.2, arp_tuple.0);
                }

                for arp_tuple in macs.iter() {

                    tx.build_and_send(1, 42, // ARP(28) + ethernetheader(14.. to hell with vlans) == 42
                        &mut |new_packet| {

                            let (eth_buf, arp_buf) = new_packet.split_at_mut(14);

                            let mut ethernet_packet = packet::ethernet::MutableEthernetPacket::new(eth_buf).unwrap();

                            ethernet_packet.set_source(my_mac);
                            ethernet_packet.set_destination(arp_tuple.0);
                            ethernet_packet.set_ethertype(packet::ethernet::EtherTypes::Arp);

                            let mut arp_layer = packet::arp::MutableArpPacket::new(arp_buf).unwrap();

                            arp_layer.set_hardware_type(packet::arp::ArpHardwareTypes::Ethernet);
                            arp_layer.set_hw_addr_len(6);     // 48-bit MACs
                            arp_layer.set_proto_addr_len(4);  // IPv4 so smol!
                            arp_layer.set_operation(packet::arp::ArpOperations::Reply);
                            arp_layer.set_protocol_type(packet::ethernet::EtherTypes::Ipv4);
                            arp_layer.set_sender_hw_addr(my_mac);
                            arp_layer.set_target_hw_addr(arp_tuple.0);
                            arp_layer.set_sender_proto_addr(arp_tuple.2);
                            arp_layer.set_target_proto_addr(arp_tuple.1);
                    });
                }
            }
        });
        
        let my_mac = self.iface.mac.unwrap();

        loop {
            match rx.next() {
                Ok(wire_packet) => {

                        let e = packet::ethernet::EthernetPacket::new(wire_packet).unwrap();

                        if e.get_ethertype() == packet::ethernet::EtherTypes::Arp {
                            match packet::arp::ArpPacket::new(e.payload()) {
                                /* No support in libpnet to check for PACKET_OUTGOING :( */
                                Some(arp_packet) if arp_packet.get_sender_hw_addr() != my_mac && arp_packet.get_operation() == packet::arp::ArpOperations::Request => match ctx.send(Some((arp_packet.get_sender_hw_addr(), arp_packet.get_sender_proto_addr(), arp_packet.get_target_proto_addr()))) {
                                                Err(e) => panic!("Feels bad, man: {}", e),
                                                _ => {},
                                            },
                                _ => (),
                            };
                        }

                },

                // I dunno.. maybe someone removed the link/interface from the machine suddenly?  Tell the spewer thread to end and then return the error.                
                Err(e) => { ctx.send(None).expect("panicked during shutdown signal to thread"); spewer.join().expect("spewer thread panicked"); return Err(e.into()) },
            }
        }
    }
}
