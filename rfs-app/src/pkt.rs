

pub enum RfsCommand {
    
}

pub struct RfsTelemetry {
    pub id: u64,
    pub data: String,
}


pub enum RfsPacket {
    Command(RfsCommand),
    Telemetry(RfsTelemetry),
}