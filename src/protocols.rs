mod ip4;

pub trait Protocol {
    fn as_bytes(&self) -> Vec<u8>;
}

pub fn generate_one_packet() -> Vec<u8> {
    ip4::generate_one_packet()
}

