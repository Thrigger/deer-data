mod protocols;
mod conf;

use protocols::*;
use conf::*;

use std::fs::File;
use std::time::Duration;
use pcap_file::pcap::*;
use pcap_file::DataLink;

fn main() {
    /* TODO read arguments and print this is no arguments supplied */
    print_help();

    let conf = Conf { packets: 5 };

    run(conf);
}

fn run(conf: Conf) {
    conf.print_conf();

    let mut packets: Vec<Vec<u8>> = vec![];
    for i in 0..conf.packets {
        packets.push(generate_one_packet());
        println!("{}. {:?}", i, packets.last().unwrap());
    }

    let header = PcapHeader {
        datalink: DataLink::IPV4,
        ..Default::default()
    };

    let out_file = File::create("generated.pcap").unwrap();
    let mut pcap_writer = PcapWriter::with_header(out_file, header).unwrap();

    for raw_packet in &packets {
        let packet: PcapPacket = PcapPacket::new(Duration::new(1, 2) , raw_packet.len() as u32, &raw_packet);

        pcap_writer.write_packet(&packet).unwrap();
    }


}

fn print_help() {
    println!(r##"
---Deer-Data---
Deer-Data is a packet generating application.
-n [number of packets]
    The number of packets given is set with n flag.
"##);
}
