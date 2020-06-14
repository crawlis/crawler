pub struct NatsPublisher {
    conn: nats::Connection,
    subject: String,
}

impl NatsPublisher {
    pub fn new(uri: &str, subject: &str) -> std::io::Result<NatsPublisher> {
        let conn = nats::connect(uri)?;
        Ok(NatsPublisher {
            conn,
            subject: String::from(subject),
        })
    }
    pub fn publish(&self, key: &str, message: Vec<u8>) -> std::io::Result<()> {
        let subject = format!("{}.{}", &self.subject, key);
        self.conn.publish(&subject, message)
    }
    pub fn close(self) {
        self.conn.close()
    }
}

pub struct NatsSubscriber {
    conn: nats::Connection,
    sub: nats::Subscription,
}

impl NatsSubscriber {
    pub fn new(uri: &str, subject: &str) -> std::io::Result<NatsSubscriber> {
        let conn = nats::connect(uri)?;
        let sub = format!("{}.*", subject);
        let sub = conn.queue_subscribe(&sub, "crawler")?;
        Ok(NatsSubscriber { conn, sub })
    }
    pub fn get_next_message(&self) -> Option<nats::Message> {
        self.sub.next()
    }
    pub fn close(self) {
        self.conn.close()
    }
}
