//! Format: https://www.fon.hum.uva.nl/praat/manual/TextGrid_file_formats.html
//! 
//! Long format:
//! ```
//! File type = "ooTextFile"
//! Object class = "TextGrid"
//! 
//! xmin = 0
//! xmax = 2.3
//! tiers? <exists>
//! size = 3
//! item []:
//!    item [1]:
//!       class = "IntervalTier"
//!       name = "Mary"
//!       xmin = 0
//!       xmax = 2.3
//!       intervals: size = 1
//!       intervals [1]:
//!          xmin = 0
//!          xmax = 2.3
//!          text = ""
//!    item [2]:
//!       class = "IntervalTier"
//!       name = "John"
//!       xmin = 0
//!       xmax = 2.3
//!       intervals: size = 1
//!       intervals [1]:
//!          xmin = 0
//!          xmax = 2.3
//!          text = ""
//!    item [3]:
//!       class = "TextTier"
//!       name = "bell"
//!       xmin = 0
//!       xmax = 2.3
//!       points: size = 0
//! ```
//! 
//! Short format:
//! ```
//! File type = "ooTextFile"
//! Object class = "TextGrid"
//!    
//! 0
//! 2.3
//! <exists>
//! 3
//! "IntervalTier"
//! "Mary"
//! 0
//! 2.3
//! 1
//! 0
//! 2.3
//! ""
//! "IntervalTier"
//! "John"
//! 0
//! 2.3
//! 1
//! 0
//! 2.3
//! ""
//! "TextTier"
//! "bell"
//! 0
//! 2.3
//! 0
//! ```
//! 
//! Even shorter format (`!` is a comment):
//! Cited from <https://www.fon.hum.uva.nl/praat/manual/TextGrid_file_formats.html> (is this the binary format?):
//! > [...] a human-readable TextGrid file that can be interpreted by Praat could look as follows:
//! ```
//! "ooTextFile"
//! "TextGrid"
//! 0 2.3 ! time domain of TextGrid
//! <exists>
//! 3 tiers
//! "IntervalTier" "Mary" ! type and name of tier 1
//! 0 2.3 ! time domain of tier 1
//! 1 interval coming
//! 0 2.3 "" ! interval 1 on tier 1
//! "IntervalTier" "John" ! type and name of tier 2
//! 0 2.3 ! time domain of tier 2
//! 1 interval coming
//! 0 2.3 "" ! interval 1 on tier 2
//! "TextTier" "bell" ! type and name of tier 3
//! 0 2.3 ! time domain of tier 3
//! 0 points coming
//! ```
//! 
//! Binary format annotated (from <https://github.com/Legisign/Praat-textgrids/blob/master/Binary%20file%20format.txt>):
//! ```
//! # Header
//! b'ooBinaryFile\x08TextGrid'         # where \x08 = len('TextGrid')
//! grid_xmin : double
//! grid_xmax : double
//! exists : bool
//! tiers : int
//! 
//! # Per Tier
//! str_len : Byte
//! tier_type : str_len * Byte
//! str_len : int
//! tier_name : str_len * Byte
//! (tier_xmin : double)                 # discarded
//! (tier_xmax : double)                 # discarded
//! elements : int
//! 
//! # Per Point
//! xpos : double
//! str_len : short
//! str_len != -1:
//!     text : str_len * Byte
//! else:
//!     # discard the -1
//!     str_len : short * 2             # double read value for UTF-16
//!     text : str_len * Byte
//! 
//! # Per Interval
//! xmin : double
//! xmax : double
//! srt_len : short
//! str_len != -1:
//!     text : str_len * Byte
//! else:
//!     # discard the -1
//!     str_len short
//!     text : str_len * Byte
//! ```

use std::{path::Path, fs::File, io::Read, char::decode_utf16};

use super::encoding::Utf16Endian;

#[derive(Debug)]
pub struct TextGrid {
    raw: String,
}

impl TextGrid {

    pub fn from_path(path: &Path) -> std::io::Result<TextGrid> {
        let mut buf: Vec<u8> = Vec::new();
        let len = File::open(path)?.read_to_end(&mut buf)?; // a few kb to 20mb, bufread?

        let string = bytes2string(buf)?;

        Ok(Self{raw: string})
    }

    pub fn lines(&self) -> std::str::Lines {
        self.raw.lines()
    }
}

/// Parses bytes into string, assuming it is encoded as UTF-8 or UTF-16.
fn bytes2string(bytes: Vec<u8>) -> std::io::Result<String> {
    match bytes.len() {
        0 => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "TextGrid file is empty.")),
        _ => {
            let mut bom = [0; 2];
            let _n = bytes.take(2).read(&mut bom)?;

            if bytes.len() % 2 != 0 {
                let msg = "TextGrid file is not 16bit aligned.";
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, msg));
            }

            match Utf16Endian::from(bom) {
                Utf16Endian::Big => {
                    let iter = (0 .. bytes.len() / 2)
                        .map(|i| u16::from_be_bytes([bytes[2*i], bytes[2*i+1]]));
                    decode_utf16(iter).collect::<Result<String, _>>()
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
                },
                Utf16Endian::Little => {
                    let iter = (0 .. bytes.len() / 2)
                        .map(|i| u16::from_le_bytes([bytes[2*i], bytes[2*i+1]]));
                    decode_utf16(iter).collect::<Result<String, _>>()
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
                },
                // Assume UTF-8 if no BOM.
                Utf16Endian::Invalid => {
                    String::from_utf8(bytes)
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
                },
            }
        }
    }
}