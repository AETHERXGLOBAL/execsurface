use std::env;
use std::fs;
use std::net::TcpStream;
use std::process::Command;

fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("noop") => {}
        Some("spawn") => {
            let status = Command::new("/bin/true").status().expect("spawn /bin/true");
            assert!(status.success());
        }
        Some("file") => {
            let path = args.next().expect("file path");
            let _ = fs::read(path).expect("read fixture file");
        }
        Some("network") => {
            let address = args.next().expect("socket address");
            let _stream = TcpStream::connect(address).expect("connect fixture listener");
        }
        other => panic!("unknown fixture mode: {other:?}"),
    }
}
