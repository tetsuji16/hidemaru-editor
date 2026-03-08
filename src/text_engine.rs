use ropey::Rope;
use std::fs::File;
use std::io::{Read, BufWriter, Result};

use regex::Regex;
use encoding_rs::{SHIFT_JIS, UTF_8};

#[derive(Default)]
pub struct TextEngine {
    pub buffer: Rope,
    pub file_path: Option<String>,
}

pub struct SearchResult {
    pub start_byte: usize,
    pub end_byte: usize,
}

impl TextEngine {
    pub fn new() -> Self {
        Self {
            buffer: Rope::new(),
            file_path: None,
        }
    }

    pub fn load_from_file(&mut self, path: &str) -> Result<()> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Try UTF-8 first
        let (text, _, had_errors) = UTF_8.decode(&buffer);
        if !had_errors {
            self.buffer = Rope::from_str(&text);
        } else {
            // Fallback to Shift-JIS (Common in Japan/Hidemaru)
            let (text, _, _) = SHIFT_JIS.decode(&buffer);
            self.buffer = Rope::from_str(&text);
        }

        self.file_path = Some(path.to_string());
        Ok(())
    }

    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        self.buffer.write_to(writer)?;
        Ok(())
    }

    pub fn find(&self, pattern: &str, start_char: usize) -> Option<SearchResult> {
        let text = self.buffer.to_string(); // Caching or chunking needed for 10M+ lines
        let re = Regex::new(pattern).ok()?;
        
        let start_byte = self.buffer.char_to_byte(start_char);
        if start_byte >= text.len() { return None; }

        if let Some(m) = re.find(&text[start_byte..]) {
            Some(SearchResult {
                start_byte: start_byte + m.start(),
                end_byte: start_byte + m.end(),
            })
        } else {
            None
        }
    }

    pub fn get_text(&self) -> String {
        self.buffer.to_string()
    }

    pub fn set_text(&mut self, text: &str) {
        self.buffer = Rope::from_str(text);
    }
}
