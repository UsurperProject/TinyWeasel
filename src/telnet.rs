use std::io::{Read, Write, self};
use std::net::TcpStream;
use std::time::{Duration, Instant, SystemTime};

pub struct Telnet {
    stream: TcpStream,
}

impl Drop for Telnet {
    fn drop(&mut self) {
        let mut buffer = [0u8; 1024];
        self.stream.write_all(b"echo '__ENDS'_'EXECUTION__' >> /end.txt && cat /end.txt\r\n")   .expect("Failed to send the magic word");
        self.stream.flush().expect("Failed to flush stream");
        loop {
            match self.stream.read(&mut buffer).unwrap() {
                0 => break,
                _ => {
                    if str::from_utf8(&buffer)
                        .unwrap_or_default()
                        .contains("__ENDS_EXECUTION__")
                    {
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

    pub fn send(&mut self, cmd: &str, duration: Option<u64>) -> std::io::Result<String> {
        let duration = duration.unwrap_or_else(|| 10);

        let id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let start = format!("__S{id}__");
        let end = format!("__E{id}__");

        let line = format!("echo '__S''{id}''__'; {cmd}; echo '__E''{id}''__'\n");

        self.stream.write_all(line.as_bytes())?;
        self.stream.flush()?;

        let mut raw: Vec<u8> = Vec::new();
        let mut chunk = [0u8; 4096];
        let deadline = Instant::now() + Duration::from_secs(duration);


        while Instant::now() < deadline {
            match self.stream.read(&mut chunk) {
                Ok(0) => {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "connection closed",
                    ))
                }
                Ok(n) => raw.extend_from_slice(&chunk[..n]),
                Err(e)
                if matches!(
                        e.kind(),
                        io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
                    ) => {}
                Err(e) => return Err(e),
            }
            let text = String::from_utf8_lossy(&strip_iac(&raw)).into_owned();

            if let (Some(s), Some(e)) = (text.find(&start), text.rfind(&end)) {
                let from = s + start.len();
                if e >= from {
                    return Ok(text[from..e]
                        .trim_matches(|c| c == '\r' || c == '\n')
                        .to_string());
                }
            }
        }


        Err(io::Error::new(io::ErrorKind::TimedOut, "command couldnt' execute in time, it might be stucked.. "))

    }
}

fn strip_iac(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len());
    let mut i = 0;
    while i < data.len() {
        if data[i] == 0xFF && i + 1 < data.len() {
            match data[i + 1] {
                0xFF => {
                    out.push(0xFF); // escaped literal 0xFF
                    i += 2;
                }
                251..=254 => i += 3, // WILL / WONT / DO / DONT + option
                _ => i += 2,
            }
        } else {
            out.push(data[i]);
            i += 1;
        }
    }
    out
}