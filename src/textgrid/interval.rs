// use serde::{ser, Serialize};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::result::Result;
use std::error::Error;

use crate::TgError;


/// Textgrid interval/annotation.
/// ```
/// intervals [1]:
///     xmin = 0
///     xmax = 2.3
///     text = "some annotation value"
/// ```
pub struct Interval {
    id: usize,
    xmin: f64,
    xmax: f64,
    text: String,
}

impl Serialize for Interval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_struct("Interval", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("xmin", &self.xmin)?;
        state.serialize_field("xmax", &self.xmax)?;
        state.serialize_field("text", &self.text)?;
        state.end()
    }
}

// pub struct Serializer {
//     output: String,
// }

// impl Interval {
//     // pub fn to_string(&self) -> String {
//     //     format!("        intervals [{}]:\n            xmin = {}\n            xmax = {}\n            text = \"{}\"\n",
//     //         self.id, self.xmin, self.xmax, self.text)
//     // }
// }

// pub fn to_string<T>(value: &T) -> Result<String, TgError>
// where
//     T: Serialize,
// {
//     let mut serializer = Serializer { output: String::new() };

//     value.serialize(&mut serializer)?;

//     Ok(serializer.output)
// }