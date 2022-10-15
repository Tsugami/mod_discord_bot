use amiquip::{Connection, Exchange, Publish, Result};

use crate::config::Config;

pub struct RabbitMQ {
    connection: Connection,
}

impl RabbitMQ {
    pub fn new(config: Config) -> Self {
        let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

        return Self { connection };
    }
}
