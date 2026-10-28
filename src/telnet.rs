use std::io::{Read, Write};
use std::net::TcpStream;

pub struct Telnet {
    stream: TcpStream,
}

impl Drop for Telnet {
    fn drop(&mut self) {
        let mut buffer = [0u8; 1024];
        self.send("echo '__ENDS'_'EXECUTION__' >> /end.txt && cat /end.txt")
            .expect("Failed to send the magic word");

        loop {
            match self.stream.read(&mut buffer).unwrap() {
                0 => break,
                _ => {
                    if str::from_utf8(&buffer)
                        .unwrap_or_default()
                        .contains("__ENDS_EXECUTION__")
                    {
                        println!("Got the magic word, breaking now");
                        break;
                    }
                }
            }
        }
        self.stream
            .shutdown(std::net::Shutdown::Both)
            .expect("Failed to properly shutdown the stream");
    }
}

impl Telnet {
    pub fn new(host: &str, port: u16) -> std::io::Result<Self> {
        let stream = TcpStream::connect((host, port))?;
        Ok(Telnet { stream })
    }

    pub fn send(&mut self, data: &str) -> std::io::Result<()> {
        let data = match data.ends_with("\r\n") {
            true => data.to_string(),
            false => format!("{}\r\n", data),
        };
        self.stream.write_all(data.as_bytes())?;
        self.stream.flush()?;
        Ok(())
    }
}
