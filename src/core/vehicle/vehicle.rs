pub enum LogType {
    Info,
    Warning,
    Error,
    Critical,
}

pub struct Log {
    pub log_type: LogType,
    pub message: String,
}

impl Log {
    pub fn from(log_type: LogType, msg: &str) -> Self {
        Self {
            log_type,
            message: String::from(msg),
        }
    }
}

pub struct VehicleState {
    pub armed: bool,
}

pub struct Vehicle {
    pub logs: Vec<Log>,
    pub state: VehicleState,
}
