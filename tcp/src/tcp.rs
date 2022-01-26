use std::io::prelude::*;
use std::net::TcpStream;

pub struct Tcp;

impl Tcp {
    pub fn new() -> Self {
        Tcp
    }

    pub fn get(&self, host: String) -> std::io::Result<()> {
        // let host = "www.rustinaction.com:80";
        let mut conn = TcpStream::connect(host)?;

        conn.write_all(b"GET / HTTP/1.0")?;
        conn.write_all(b"\r\n")?;
        conn.write_all(b"Host: www.rustinaction.com")?;
        conn.write_all(b"\r\n\r\n")?;

        std::io::copy(
            &mut conn,
            &mut std::io::stdout()
            )?;
        Ok(())
    }
}

#[cfg(test)]
mod tcp_test {
    use crate::tcp::Tcp;

    #[test]
    fn test_get() {
        let tcp = Tcp::new();
        let result = tcp.get("www.rustinaction.com:80".to_string());
        assert!(result.is_ok());
    }
}
