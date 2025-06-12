pub struct Server {
    address: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { address: addr }
    }

    pub fn run(&mut self) {
        println!("Listening to {}", self.address);
    }
}
