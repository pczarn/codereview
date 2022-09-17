use std::fmt;

mod date;
pub use self::date::Date;

mod time;
pub use self::time::Time;

pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

impl DateTime {
    pub fn new(date: impl Into<Date>, time: impl Into<Time>) -> DateTime {
        DateTime {
            date: date.into(),
            time: time.into(),
        }
    }

    pub fn create(
        year: impl Into<u16>,
        month: impl Into<u8>,
        day: impl Into<u8>,
        hour: impl Into<u8>,
        minute: impl Into<u8>,
        second: impl Into<u8>,
    ) -> DateTime {
        DateTime::new(Date::new(year, month, day), Time::new(hour, minute, second))
    }

    pub fn isoformat(&self) -> String {
        format!("{}T{}", self.date.isoformat(), self.time.isoformat())
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.isoformat())
    }
}
