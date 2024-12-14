#[derive(Debug, Clone, Copy)]
pub struct UnsolvedDay {
    year: u16,
    day: u8,
}

impl UnsolvedDay {
    pub fn new(year: u16, day: u8) -> Self {
        Self {
            year,
            day,
        }
    }

    pub fn year(&self) -> u16 {
        self.year
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn alt(&self) -> Option<&'static str> {
        None
    }
}
