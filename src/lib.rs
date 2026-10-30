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
    fn command_output_matching() {
        let mut TELNET =
            Telnet::new("localhost", 2157).expect("Failed to connect to the telnet server");
        let response = TELNET
            .send("echo 'Hello, World!'", None)
            .expect("Failed to send command");

        assert_eq!(response, "Hello, World!");
    }

    #[test]
    #[should_panic]
    fn command_output_not_matching() {
        let mut TELNET = Telnet::new("localhost", 2157).expect("Failed to connect to the telnet server");
        let response = TELNET.send("echo 'Hello, World!'", None).expect("Failed to send command");
        assert_eq!("Hello", response);
    }
}
