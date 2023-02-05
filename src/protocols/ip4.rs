use super::Protocol;

pub struct Ip4 {
   version: u8,
   header_len: u8,
   dscp: u8,
   ecn: u8,
   total_len: u16,
   id: u16,
   flags: u8,
   frag_offset: u16,
   time_to_live: u8,
   protocol: u8,
   header_checksum: u16,
   src: u32,
   dst: u32,
   option: Option<Vec<u8>>,
   data: Vec<u8>,
}

impl Ip4 {
    fn new(src: u32, dst: u32, protocol: u8, data: Vec<u8>) -> Ip4 {
        let header_len: u8 = 5;
        let total_len: u16 = header_len as u16 * 4 + data.len() as u16;

        Ip4 {version: 4,
        header_len,
        dscp: 0,
        ecn: 0,
        total_len,
        id: 0,
        flags: 0,
        frag_offset: 0,
        time_to_live: 10,
        protocol,
        header_checksum: 0,
        src,
        dst,
        option: None,
        data,
        }
    }

    fn calculate_checksum(&self) -> u16 {
        let mut sum: u32 = 0;
        let bytes = self.as_bytes();

        let mut i: usize = 0;
        while i < self.header_len as usize * 4 {
            if i != 10 {
                sum += ((bytes[i] as u32) << 8) | (bytes[i+1] as u32);
            }
            i += 2;
        }
        let over = (sum >> 8) as u16;
        sum &= 0x0000FFFF;
        sum += over as u32;
        
        let sum_u16 = sum as u16;
        !sum_u16
    }
}

impl Protocol for Ip4 {
    fn as_bytes(&self) -> Vec<u8> {
        let mut output: Vec<u8> = vec![];
        let ids = self.id.to_be_bytes();
        let frag_offsets = self.frag_offset.to_be_bytes();

        output.push((self.version << 4) | self.header_len);
        output.push((self.dscp << 2) | self.ecn);
        for byte in self.total_len.to_be_bytes() {
            output.push(byte);
        }
        output.push(ids[0]);
        output.push(ids[1]);
        output.push((self.flags << 5) | frag_offsets[0]);
        output.push(frag_offsets[1]);
        output.push(self.time_to_live);
        output.push(self.protocol);
        for byte in self.header_checksum.to_be_bytes() {
            output.push(byte);
        }
        for byte in self.src.to_be_bytes() {
            output.push(byte);
        }
        for byte in self.dst.to_be_bytes() {
            output.push(byte);
        }
        match &self.option {
            None => (),
            Some(o) => {
                for byte in o {
                    output.push(*byte);
                }
            }
        };
        for byte in &self.data {
            output.push(*byte);
        }

        output
    }

}

pub fn generate_one_packet() -> Vec<u8> {
    let data = vec![0x55; 40];
    let src = 127 << 24 | 1;
    let mut packet = Ip4::new(src, src, 6, data);
    packet.header_checksum = packet.calculate_checksum();

    packet.as_bytes()
}

