use super::Interval;

/// Textgrid tier.
pub struct Tier {
    id: usize,
    class: TierClass,
    name: String,
    xmin: f64,
    xmax: f64,
    intervals_size: usize, // e.g. "intervals: size = 147" (= self.intervals.len())
    intervals: Vec<Interval>, // enumerated e.g. "intervals [1]: ..., intervals [2]: ..."
}

pub enum TierClass {
    Interval,
    Text,
    Invalid
}

impl From<String> for TierClass {
    fn from(s: String) -> Self {
        match s.as_str() {
            "IntervalTier" => Self::Interval,
            "TextTier" => Self::Text,
            _ => Self::Invalid,
        }
    }
}