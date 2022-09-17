use std::fmt;

pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Time {
    pub fn new(hour: impl Into<u8>, minute: impl Into<u8>, second: impl Into<u8>) -> Time {
        Time {
            hour: hour.into(),
            minute: minute.into(),
            second: second.into(),
        }
    }

    pub fn isoformat(&self) -> String {
        format!("{:0>2}:{:0>2}:{:0>2}", self.hour, self.minute, self.second)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.isoformat())
    }
}
