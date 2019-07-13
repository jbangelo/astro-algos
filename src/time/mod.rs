use std::convert::From;

pub mod date;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct JD {
    value: f64,
}

impl JD {
    pub fn to_f64(&self) -> f64 {
        self.value
    }
}
impl From<f64> for JD {
    fn from(item: f64) -> Self {
        assert!(item >= 0.0, "Invalid JD value: {}", item);
        JD { value: item }
    }
}

impl From<JD> for f64 {
    fn from(item: JD) -> Self {
        item.value
    }
}
