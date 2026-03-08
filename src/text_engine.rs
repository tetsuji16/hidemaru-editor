use ropey::Rope;
use std::fs::File;
use std::io::{BufReader, BufWriter, Result};
use encoding_rs::UTF_8;

pub struct TextEngine {
    pub buffer: Rope,
    pub file_path: Option<String>,
}

impl TextEngine {
    pub fn new() -> Self {
        Self {
            buffer: Rope::new(),
            file_path: None,
        }
    }

    pub fn load_from_file(&mut self, path: &str) -> Result<()> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        
        // --- Encoding handling (Simplified for now, assuming UTF-8 or similar) ---
        // In full Hidemaru, we'd use encoding_rs to detect SJIS/EUC-JP etc.
        self.buffer = Rope::from_reader(reader)?;
        self.file_path = Some(path.to_string());
        Ok(())
    }

    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        self.buffer.write_to(writer)?;
        Ok(())
    }

    pub fn get_text(&self) -> String {
        self.buffer.to_string()
    }

    pub fn set_text(&mut self, text: &str) {
        self.buffer = Rope::from_str(text);
    }
}
