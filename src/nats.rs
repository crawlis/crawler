pub struct NatsPublisher {
    conn: nats::Connection,
}

impl NatsPublisher {
    pub fn new(uri: &str) -> Result<NatsPublisher, std::io::Error> {
        let conn = nats::connect(uri)?;
        Ok(NatsPublisher { conn })
    }
    pub fn publish(&self, key: &str, value: Vec<u8>) -> Result<(), std::io::Error> {
        self.conn.publish(key, value)
    }
    pub fn close(self) -> Result<(), std::io::Error> {
        self.conn.close()
    }
}
