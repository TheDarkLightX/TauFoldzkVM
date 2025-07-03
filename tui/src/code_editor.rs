//! Advanced code editor with syntax highlighting

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use std::collections::HashMap;

pub struct CodeEditor {
    pub syntax_highlighter: SyntaxHighlighter,
    pub auto_complete: AutoComplete,
    pub error_markers: HashMap<usize, String>,
}

pub struct SyntaxHighlighter {
    keywords: HashMap<String, Color>,
    instruction_colors: HashMap<String, Color>,
}

pub struct AutoComplete {
    suggestions: Vec<String>,
    current_context: String,
}

impl CodeEditor {
    pub fn new() -> Self {
        Self {
            syntax_highlighter: SyntaxHighlighter::new(),
            auto_complete: AutoComplete::new(),
            error_markers: HashMap::new(),
        }
    }
    
    pub fn highlight_line(&self, line: &str, line_number: usize) -> Line {
        let mut spans = Vec::new();
        
        // Line number
        spans.push(Span::styled(
            format!("{:4} ", line_number),
            Style::default().fg(Color::DarkGray),
        ));
        
        // Error marker
        if let Some(error) = self.error_markers.get(&line_number) {
            spans.push(Span::styled("⚠ ", Style::default().fg(Color::Red)));
        } else {
            spans.push(Span::raw("  "));
        }
        
        // Syntax highlighting
        let highlighted_spans = self.syntax_highlighter.highlight(line);
        spans.extend(highlighted_spans);
        
        Line::from(spans)
    }
    
    pub fn get_suggestions(&mut self, context: &str) -> Vec<String> {
        self.auto_complete.get_suggestions(context)
    }
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        let mut keywords = HashMap::new();
        let mut instruction_colors = HashMap::new();
        
        // TauFoldZKVM instructions
        let instructions = vec![
            ("PUSH", Color::Cyan),
            ("POP", Color::Cyan),
            ("DUP", Color::Cyan),
            ("SWAP", Color::Cyan),
            ("ADD", Color::Yellow),
            ("SUB", Color::Yellow),
            ("MUL", Color::Yellow),
            ("DIV", Color::Yellow),
            ("MOD", Color::Yellow),
            ("AND", Color::Magenta),
            ("OR", Color::Magenta),
            ("XOR", Color::Magenta),
            ("NOT", Color::Magenta),
            ("SHL", Color::Magenta),
            ("SHR", Color::Magenta),
            ("EQ", Color::Green),
            ("NEQ", Color::Green),
            ("LT", Color::Green),
            ("GT", Color::Green),
            ("LTE", Color::Green),
            ("GTE", Color::Green),
            ("JMP", Color::Red),
            ("JZ", Color::Red),
            ("JNZ", Color::Red),
            ("CALL", Color::Red),
            ("RET", Color::Red),
            ("LOAD", Color::Blue),
            ("STORE", Color::Blue),
            ("MLOAD", Color::Blue),
            ("MSTORE", Color::Blue),
            ("HASH", Color::LightRed),
            ("VERIFY", Color::LightRed),
            ("SIGN", Color::LightRed),
            ("HALT", Color::White),
            ("NOP", Color::DarkGray),
            ("DEBUG", Color::LightCyan),
            ("ASSERT", Color::LightYellow),
            ("LOG", Color::LightCyan),
        ];
        
        for (inst, color) in instructions {
            instruction_colors.insert(inst.to_string(), color);
            instruction_colors.insert(inst.to_lowercase(), color);
        }
        
        Self {
            keywords,
            instruction_colors,
        }
    }
    
    pub fn highlight(&self, line: &str) -> Vec<Span> {
        let mut spans = Vec::new();
        
        // Check for comments
        if let Some(comment_pos) = line.find("//") {
            let (code, comment) = line.split_at(comment_pos);
            
            // Highlight code part
            spans.extend(self.highlight_code(code));
            
            // Highlight comment
            spans.push(Span::styled(
                comment,
                Style::default().fg(Color::DarkGray),
            ));
        } else {
            spans.extend(self.highlight_code(line));
        }
        
        spans
    }
    
    fn highlight_code(&self, code: &str) -> Vec<Span> {
        let mut spans = Vec::new();
        let mut current_word = String::new();
        let mut current_pos = 0;
        
        for (i, ch) in code.char_indices() {
            if ch.is_whitespace() {
                if !current_word.is_empty() {
                    spans.push(self.highlight_word(&current_word));
                    current_word.clear();
                }
                spans.push(Span::raw(ch.to_string()));
            } else {
                current_word.push(ch);
            }
            current_pos = i;
        }
        
        if !current_word.is_empty() {
            spans.push(self.highlight_word(&current_word));
        }
        
        spans
    }
    
    fn highlight_word(&self, word: &str) -> Span {
        // Check if it's an instruction
        if let Some(&color) = self.instruction_colors.get(word) {
            return Span::styled(word, Style::default().fg(color));
        }
        
        // Check if it's a number
        if word.parse::<i64>().is_ok() || 
           word.starts_with("0x") || 
           word.starts_with("0b") {
            return Span::styled(word, Style::default().fg(Color::LightGreen));
        }
        
        // Check if it's a label (ends with :)
        if word.ends_with(':') {
            return Span::styled(word, Style::default().fg(Color::LightBlue));
        }
        
        // Default
        Span::raw(word)
    }
}

impl AutoComplete {
    pub fn new() -> Self {
        let suggestions = vec![
            // Instructions
            "ADD", "SUB", "MUL", "DIV", "MOD",
            "AND", "OR", "XOR", "NOT", "SHL", "SHR",
            "EQ", "NEQ", "LT", "GT", "LTE", "GTE",
            "PUSH", "POP", "DUP", "SWAP",
            "JMP", "JZ", "JNZ", "CALL", "RET",
            "LOAD", "STORE", "MLOAD", "MSTORE",
            "HASH", "VERIFY", "SIGN",
            "HALT", "NOP", "DEBUG", "ASSERT", "LOG",
            "READ", "WRITE", "SEND", "RECV",
            "TIME", "RAND", "ID",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        
        Self {
            suggestions,
            current_context: String::new(),
        }
    }
    
    pub fn get_suggestions(&mut self, context: &str) -> Vec<String> {
        self.current_context = context.to_string();
        
        if context.is_empty() {
            return self.suggestions.clone();
        }
        
        let context_upper = context.to_uppercase();
        self.suggestions
            .iter()
            .filter(|s| s.starts_with(&context_upper))
            .cloned()
            .collect()
    }
}