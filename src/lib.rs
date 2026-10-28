mod telnet;

#[cfg(test)]
mod tests {
    use super::*;
    use telnet::Telnet;

    #[test]
    fn it_connects() {
        let mut TELNET =
            Telnet::new("localhost", 2157).expect("Failed to connect to the telnet server");
    }

    #[test]
    fn it_executes_command() {
        let mut TELNET =
            Telnet::new("localhost", 2157).expect("Failed to connect to the telnet server");
        TELNET
            .send("echo 'Hello, World!'")
            .expect("Failed to send command");
    }
}
