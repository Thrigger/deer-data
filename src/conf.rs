pub struct Conf {
    pub packets: i64,
    /* TODO add a packet conf that can be specified how to generate packets, enum? vec? struct? */
}

impl Conf {
    pub fn print_conf(&self) {
        println!("Current Config");
        println!("Packets: {}", self.packets);
    }
}

