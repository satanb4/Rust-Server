use std::net::TcpListener;

pub struct Server {
    addr: String,
}
impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr: addr, //If the name is the same, no need for assignment
        }
    }
    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        
        // For loop annotations, we can use loop naming
        // 'outer: loop{
        //     loop{
        //         break 'outer;
        //     }
        //}
    }
}