//! File browser component for the TUI

use std::path::{Path, PathBuf};
use std::fs;

pub struct FileBrowser {
    pub current_path: PathBuf,
    pub entries: Vec<FileEntry>,
    pub selected_index: usize,
    pub show_hidden: bool,
}

#[derive(Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: std::time::SystemTime,
}

impl FileBrowser {
    pub fn new() -> Self {
        let current_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        let mut browser = Self {
            current_path: current_path.clone(),
            entries: Vec::new(),
            selected_index: 0,
            show_hidden: false,
        };
        browser.refresh();
        browser
    }
    
    pub fn refresh(&mut self) {
        self.entries.clear();
        
        // Add parent directory entry
        if let Some(parent) = self.current_path.parent() {
            self.entries.push(FileEntry {
                path: parent.to_path_buf(),
                name: "..".to_string(),
                is_dir: true,
                size: 0,
                modified: std::time::SystemTime::now(),
            });
        }
        
        // Read directory entries
        if let Ok(entries) = fs::read_dir(&self.current_path) {
            let mut file_entries: Vec<FileEntry> = entries
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let metadata = entry.metadata().ok()?;
                    let name = entry.file_name().to_string_lossy().to_string();
                    
                    // Skip hidden files if not showing them
                    if !self.show_hidden && name.starts_with('.') {
                        return None;
                    }
                    
                    Some(FileEntry {
                        path: entry.path(),
                        name,
                        is_dir: metadata.is_dir(),
                        size: metadata.len(),
                        modified: metadata.modified().unwrap_or(std::time::SystemTime::now()),
                    })
                })
                .collect();
            
            // Sort: directories first, then by name
            file_entries.sort_by(|a, b| {
                match (a.is_dir, b.is_dir) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                }
            });
            
            self.entries.extend(file_entries);
        }
        
        // Reset selection if out of bounds
        if self.selected_index >= self.entries.len() {
            self.selected_index = 0;
        }
    }
    
    pub fn select_next(&mut self) {
        if !self.entries.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.entries.len();
        }
    }
    
    pub fn select_previous(&mut self) {
        if !self.entries.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.entries.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }
    
    pub fn enter_selected(&mut self) -> Option<PathBuf> {
        if let Some(entry) = self.entries.get(self.selected_index) {
            if entry.is_dir {
                self.current_path = entry.path.clone();
                self.selected_index = 0;
                self.refresh();
                None
            } else {
                Some(entry.path.clone())
            }
        } else {
            None
        }
    }
    
    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.refresh();
    }
    
    pub fn format_size(size: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = size as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        if unit_index == 0 {
            format!("{} {}", size as u64, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }
}