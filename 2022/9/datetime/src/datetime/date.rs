use std::fmt;

pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Date {
    pub fn new(year: impl Into<u16>, month: impl Into<u8>, day: impl Into<u8>) -> Date {
        Date {
            year: year.into(),
            month: month.into(),
            day: day.into(),
        }
    }

    pub fn isoformat(&self) -> String {
        format!("{:0>4}-{:0>2}-{:0>2}", self.year, self.month, self.day)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.isoformat())
    }
}
