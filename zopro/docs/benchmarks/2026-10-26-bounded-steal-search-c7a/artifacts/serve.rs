//! Serve exactly one benchmark archive over loopback for SSM retrieval.
use std::{fs::File, io::{Read,Write}, net::TcpListener, time::{Duration,Instant}};
fn main() -> std::io::Result<()> {
    let path=std::env::args_os().nth(1).expect("archive path");
    let listener=TcpListener::bind("127.0.0.1:8765")?;
    listener.set_nonblocking(true)?;
    let deadline=Instant::now()+Duration::from_secs(180);
    while Instant::now()<deadline {
        let (mut connection,_)=match listener.accept() {
            Ok(value)=>value,
            Err(error) if error.kind()==std::io::ErrorKind::WouldBlock=>{
                std::thread::sleep(Duration::from_millis(100));continue;
            },
            Err(error)=>return Err(error),
        };
        connection.set_read_timeout(Some(Duration::from_secs(5)))?;
        connection.set_write_timeout(Some(Duration::from_secs(120)))?;
        let mut request=[0u8;4096]; let bytes=connection.read(&mut request)?;
        if !request[..bytes].starts_with(b"GET /review-results.tar.gz HTTP/1.") {
            connection.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")?;
            continue;
        }
        let mut file=File::open(&path)?;
        write!(connection,"HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",file.metadata()?.len())?;
        std::io::copy(&mut file,&mut connection)?;
        connection.flush()?;
        return Ok(());
    }
    Err(std::io::Error::new(std::io::ErrorKind::TimedOut,"no retrieval connection"))
}
