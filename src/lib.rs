//! Read and write TextGrid annotation files, commonly used in Praat.
//! - Praat: <https://www.fon.hum.uva.nl/praat/>
//! - TextGrid format: <https://www.fon.hum.uva.nl/praat/manual/TextGrid_file_formats.html>
//! 
//! Note that while both the text and binary Praat formats are supported, only UTF-8, and UTF-16 encodings are supported for text.
//! 
//! Praat's special ASCII encoding of characters outside the ASCII range is not supported.
//! See: <https://www.fon.hum.uva.nl/praat/manual/Special_symbols.html>

mod textgrid;
mod errors;

pub use textgrid::TextGrid;
pub use errors::TgError;

#[test]
fn read_tg() {
    use std::path::PathBuf;
    use crate::TextGrid;
    let path = PathBuf::from("/Users/jens/dev/TESTDATA/textgrid/The_Story_of_the_Python_mono_16000_JENS_EDIT.TextGrid");
    let result = TextGrid::from_path(&path);
    assert!(result.is_ok());
    if let Ok(tg) = result {
        // println!("{tg:#?}");
        for line in tg.lines() {
            println!("{line:#?}");
        }
    }
}