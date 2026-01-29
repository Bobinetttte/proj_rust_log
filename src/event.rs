#[derive(PartialEq, Debug, Clone)]
pub enum Gravity {
    Debug,
    Information,
    Warning,
    Error,
    Critical,
}

pub struct Event {
    pub id : String,
    pub event_type : String,
    pub gravity_level : Gravity,
}