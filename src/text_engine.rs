use ropey::Rope;
use std::fs::File;
use std::io::{Read, BufWriter, Result};

use regex::Regex;
use encoding_rs::{SHIFT_JIS, UTF_8};

#[derive(Clone, Debug)]
pub struct TextChange {
    pub start_char: usize,
    pub old_text: String,
    pub new_text: String,
}

#[derive(Default)]
pub struct TextEngine {
    pub buffer: Rope,
    pub file_path: Option<String>,
    pub undo_stack: Vec<TextChange>,
    pub redo_stack: Vec<TextChange>,
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
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn apply_change(&mut self, change: TextChange, save_history: bool) {
        if change.old_text == change.new_text { return; }

        let start_char = change.start_char;
        
        // Remove old text
        if !change.old_text.is_empty() {
            let char_count = change.old_text.chars().count();
            self.buffer.remove(start_char..start_char + char_count);
        }

        // Insert new text
        if !change.new_text.is_empty() {
            self.buffer.insert(start_char, &change.new_text);
        }

        if save_history {
            self.undo_stack.push(change);
            self.redo_stack.clear();
        }
    }

    pub fn compute_delta(old: &str, new: &str) -> TextChange {
        let old_chars: Vec<char> = old.chars().collect();
        let new_chars: Vec<char> = new.chars().collect();
        
        let mut prefix_len = 0;
        while prefix_len < old_chars.len() && prefix_len < new_chars.len() && old_chars[prefix_len] == new_chars[prefix_len] {
            prefix_len += 1;
        }
        
        let mut suffix_len = 0;
        while suffix_len < (old_chars.len() - prefix_len) && suffix_len < (new_chars.len() - prefix_len) && 
              old_chars[old_chars.len() - 1 - suffix_len] == new_chars[new_chars.len() - 1 - suffix_len] {
            suffix_len += 1;
        }
        
        let old_delta: String = old_chars[prefix_len..old_chars.len() - suffix_len].iter().collect();
        let new_delta: String = new_chars[prefix_len..new_chars.len() - suffix_len].iter().collect();
        
        TextChange {
            start_char: prefix_len,
            old_text: old_delta,
            new_text: new_delta,
        }
    }

    pub fn undo(&mut self) -> Option<TextChange> {
        let change = self.undo_stack.pop()?;
        
        // Reverse the change
        let char_count_new = change.new_text.chars().count();
        self.buffer.remove(change.start_char..change.start_char + char_count_new);
        self.buffer.insert(change.start_char, &change.old_text);
        
        self.redo_stack.push(change.clone());
        Some(change)
    }

    pub fn redo(&mut self) -> Option<TextChange> {
        let change = self.redo_stack.pop()?;
        
        // Re-apply the change
        let char_count_old = change.old_text.chars().count();
        self.buffer.remove(change.start_char..change.start_char + char_count_old);
        self.buffer.insert(change.start_char, &change.new_text);
        
        self.undo_stack.push(change.clone());
        Some(change)
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
