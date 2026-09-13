use std::error::Error;
use std::fmt::Display;
use mavlink::{ConnectionAddress, MavConnection};
use mavlink::dialects::common::MavMessage;
pub struct Connection {
    pub mavlink_conn: Option<Box<dyn MavConnection<MavMessage> + Send + Sync>>,
    pub address: String,
    pub loading: bool,
}

#[derive(Debug)]
pub enum ConnectionError {
    InvalidAddress,
    MavlinkConnError(std::io::Error)
}

impl Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionError::InvalidAddress => write!(f, "invalid address"),
            ConnectionError::MavlinkConnError(e) => write!(f, "mavlink connection error: {}", e),
        }
    }
}

impl Error for ConnectionError {}

impl Connection {
    pub fn new(
        address: &str
    ) -> Result<Connection, ConnectionError>{
        ConnectionAddress::parse_address(
            address,
        ).map_err(
            |_|  ConnectionError::InvalidAddress
        )?;

        let conn =
            mavlink::connect::<MavMessage>(&address)
                .map_err(
                    |err| ConnectionError::MavlinkConnError(err)
                )?;

        Ok(Connection {
            mavlink_conn: Some(Box::new(conn)),
            address: String::new(),
            loading: false,
        })
    }
}