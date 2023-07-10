use tun_tap::{Iface, Mode};

fn main() {
    let iface = Iface::new("tun0", Mode::Tun).expect("Failed to create a TUN device");
    
    let mut buffer = vec![0; 1504]; // MTU + 4 for the header
    let mut counter = 0;
    loop {
        let n = iface.recv(&mut buffer).unwrap();
        println!("{}: got package: {:x?}", counter, &buffer[..n]);
        counter += 1;
    }
}
