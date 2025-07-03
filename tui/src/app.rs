//! Application state and logic for TauFoldZKVM TUI

use anyhow::Result;
use crossterm::event::KeyCode;
use ratatui::widgets::ListState;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use taufold_zkvm::{
    ExecutionResult, Instruction, Program, TraceEntry, VirtualMachine, VmState,
};

use crate::debugger::DebuggerState;
use crate::executor::ProgramExecutor;
use crate::file_browser::FileBrowser;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    Normal,
    Editor,
    Debugger,
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TabIndex {
    Editor = 0,
    Debugger = 1,
    Execution = 2,
    Memory = 3,
    Constraints = 4,
}

pub struct App {
    // UI State
    pub mode: AppMode,
    pub current_tab: TabIndex,
    pub is_modified: bool,
    pub show_file_browser: bool,
    pub show_command_palette: bool,
    
    // Editor State
    pub editor_content: String,
    pub editor_state: ListState,
    pub cursor_position: (usize, usize), // (line, column)
    pub selection_start: Option<(usize, usize)>,
    pub clipboard: String,
    pub undo_stack: Vec<String>,
    pub redo_stack: Vec<String>,
    
    // File State
    pub current_file: Option<String>,
    pub recent_files: Vec<PathBuf>,
    pub file_browser: FileBrowser,
    
    // Program State
    pub current_program: Option<Program>,
    pub program_executor: ProgramExecutor,
    pub last_execution_result: Option<ExecutionResult>,
    
    // Debugger State
    pub debug_state: VmState,
    pub debugger: DebuggerState,
    pub breakpoints: HashSet<u32>,
    pub watch_expressions: Vec<String>,
    pub call_stack_view: Vec<String>,
    
    // Execution State
    pub execution_trace: Vec<TraceEntry>,
    pub trace_state: ListState,
    pub execution_history: Vec<ExecutionResult>,
    
    // Memory State
    pub memory_state: ListState,
    pub memory_search: String,
    pub memory_highlights: HashMap<usize, HighlightType>,
    
    // Constraint State
    pub constraint_violations: Vec<ConstraintViolation>,
    pub constraint_coverage: HashMap<String, usize>,
    
    // Performance Profiling
    pub instruction_timings: HashMap<String, Vec<u64>>,
    pub hotspots: Vec<(u32, u64)>, // (PC, cumulative time)
    
    // UI Enhancements
    pub theme: Theme,
    pub syntax_highlighting: bool,
    pub line_numbers: bool,
    pub minimap: bool,
    pub status_messages: Vec<(String, MessageType)>,
    pub command_history: Vec<String>,
    pub search_query: String,
    pub search_results: Vec<(usize, usize)>,
    pub current_search_result: usize,
}

#[derive(Debug, Clone)]
pub struct ConstraintViolation {
    pub cycle: u64,
    pub instruction: String,
    pub details: String,
    pub severity: ViolationSeverity,
}

#[derive(Debug, Clone, Copy)]
pub enum ViolationSeverity {
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Copy)]
pub enum HighlightType {
    Read,
    Write,
    Execute,
    Breakpoint,
}

#[derive(Debug, Clone, Copy)]
pub enum Theme {
    Dark,
    Light,
    Solarized,
    Monokai,
    TauFold, // Custom theme
}

#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Info,
    Success,
    Warning,
    Error,
}

impl App {
    pub fn new() -> Self {
        let mut editor_state = ListState::default();
        editor_state.select(Some(0));
        
        let mut trace_state = ListState::default();
        trace_state.select(Some(0));
        
        let mut memory_state = ListState::default();
        memory_state.select(Some(0));

        Self {
            mode: AppMode::Normal,
            current_tab: TabIndex::Editor,
            is_modified: false,
            show_file_browser: false,
            show_command_palette: false,
            
            editor_content: Self::default_program(),
            editor_state,
            cursor_position: (0, 0),
            selection_start: None,
            clipboard: String::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            
            current_file: None,
            recent_files: Vec::new(),
            file_browser: FileBrowser::new(),
            
            current_program: None,
            program_executor: ProgramExecutor::new(),
            last_execution_result: None,
            
            debug_state: VmState::default(),
            debugger: DebuggerState::new(),
            breakpoints: HashSet::new(),
            watch_expressions: Vec::new(),
            call_stack_view: Vec::new(),
            
            execution_trace: Vec::new(),
            trace_state,
            execution_history: Vec::new(),
            
            memory_state,
            memory_search: String::new(),
            memory_highlights: HashMap::new(),
            
            constraint_violations: Vec::new(),
            constraint_coverage: HashMap::new(),
            
            instruction_timings: HashMap::new(),
            hotspots: Vec::new(),
            
            theme: Theme::TauFold,
            syntax_highlighting: true,
            line_numbers: true,
            minimap: true,
            status_messages: Vec::new(),
            command_history: Vec::new(),
            search_query: String::new(),
            search_results: Vec::new(),
            current_search_result: 0,
        }
    }
    
    fn default_program() -> String {
        r#"// TauFoldZKVM Example Program
// Calculate (42 + 58) * 2 = 200

PUSH 42        // Push first number
PUSH 58        // Push second number
ADD            // Add them together
DUP            // Duplicate the result
PUSH 2         // Push multiplier
MUL            // Multiply
DEBUG          // Print result
HALT           // Stop execution"#.to_string()
    }
    
    pub fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            TabIndex::Editor => TabIndex::Debugger,
            TabIndex::Debugger => TabIndex::Execution,
            TabIndex::Execution => TabIndex::Memory,
            TabIndex::Memory => TabIndex::Constraints,
            TabIndex::Constraints => TabIndex::Editor,
        }
    }
    
    pub fn previous_tab(&mut self) {
        self.current_tab = match self.current_tab {
            TabIndex::Editor => TabIndex::Constraints,
            TabIndex::Debugger => TabIndex::Editor,
            TabIndex::Execution => TabIndex::Debugger,
            TabIndex::Memory => TabIndex::Execution,
            TabIndex::Constraints => TabIndex::Memory,
        }
    }
    
    pub fn handle_editor_input(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Char(c) => {
                self.push_undo_state();
                self.insert_char(c);
                self.is_modified = true;
            }
            KeyCode::Enter => {
                self.push_undo_state();
                self.insert_newline();
                self.is_modified = true;
            }
            KeyCode::Backspace => {
                self.push_undo_state();
                self.delete_char_before_cursor();
                self.is_modified = true;
            }
            KeyCode::Delete => {
                self.push_undo_state();
                self.delete_char_at_cursor();
                self.is_modified = true;
            }
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Up => self.move_cursor_up(),
            KeyCode::Down => self.move_cursor_down(),
            KeyCode::Home => self.move_cursor_to_line_start(),
            KeyCode::End => self.move_cursor_to_line_end(),
            KeyCode::PageUp => self.scroll_up(10),
            KeyCode::PageDown => self.scroll_down(10),
            _ => {}
        }
        Ok(())
    }
    
    pub fn run_program(&mut self) -> Result<()> {
        // Parse the editor content into instructions
        match self.parse_editor_content() {
            Ok(program) => {
                self.current_program = Some(program.clone());
                let mut vm = VirtualMachine::new();
                let result = vm.execute(program)?;
                
                // Update execution history and trace
                self.last_execution_result = Some(result.clone());
                self.execution_history.push(result.clone());
                self.execution_trace = result.trace.clone();
                
                // Update constraint violations
                self.constraint_violations = result.constraint_violations.iter()
                    .map(|v| ConstraintViolation {
                        cycle: v.cycle,
                        instruction: v.instruction.clone(),
                        details: v.details.clone(),
                        severity: ViolationSeverity::Error,
                    })
                    .collect();
                
                // Update performance profiling
                self.update_performance_metrics(&result);
                
                // Add status message
                let msg = if result.success {
                    ("Program executed successfully!".to_string(), MessageType::Success)
                } else {
                    (format!("Execution failed: {}", result.error.as_ref().unwrap_or(&"Unknown error".to_string())), MessageType::Error)
                };
                self.status_messages.push(msg);
                
                // Switch to execution tab
                self.current_tab = TabIndex::Execution;
            }
            Err(e) => {
                self.status_messages.push((format!("Parse error: {}", e), MessageType::Error));
            }
        }
        Ok(())
    }
    
    pub fn debug_step(&mut self) -> Result<()> {
        if let Some(program) = &self.current_program {
            self.debugger.step(&mut self.debug_state, program)?;
            self.update_debug_views();
        }
        Ok(())
    }
    
    pub fn debug_continue(&mut self) -> Result<()> {
        if let Some(program) = &self.current_program {
            while !self.debug_state.halted && 
                  !self.breakpoints.contains(&self.debug_state.program_counter) {
                self.debugger.step(&mut self.debug_state, program)?;
            }
            self.update_debug_views();
        }
        Ok(())
    }
    
    pub fn toggle_breakpoint(&mut self) -> Result<()> {
        let current_line = self.debug_state.program_counter;
        if self.breakpoints.contains(&current_line) {
            self.breakpoints.remove(&current_line);
        } else {
            self.breakpoints.insert(current_line);
        }
        Ok(())
    }
    
    pub fn debug_restart(&mut self) -> Result<()> {
        self.debug_state = VmState::default();
        self.debugger = DebuggerState::new();
        Ok(())
    }
    
    pub fn open_file(&mut self) -> Result<()> {
        self.show_file_browser = true;
        Ok(())
    }
    
    pub fn save_file(&mut self) -> Result<()> {
        if let Some(path) = &self.current_file {
            std::fs::write(path, &self.editor_content)?;
            self.is_modified = false;
            self.status_messages.push(("File saved successfully!".to_string(), MessageType::Success));
        } else {
            self.status_messages.push(("No file to save. Use Save As.".to_string(), MessageType::Warning));
        }
        Ok(())
    }
    
    pub fn new_file(&mut self) {
        self.editor_content = Self::default_program();
        self.current_file = None;
        self.is_modified = false;
        self.cursor_position = (0, 0);
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
    
    // Editor helper methods
    fn push_undo_state(&mut self) {
        self.undo_stack.push(self.editor_content.clone());
        self.redo_stack.clear();
    }
    
    fn insert_char(&mut self, c: char) {
        let (line, col) = self.cursor_position;
        let mut lines: Vec<&str> = self.editor_content.lines().collect();
        
        if line < lines.len() {
            let current_line = lines[line].to_string();
            let (before, after) = current_line.split_at(col.min(current_line.len()));
            lines[line] = &format!("{}{}{}", before, c, after);
            
            self.editor_content = lines.join("\n");
            self.cursor_position.1 += 1;
        }
    }
    
    fn insert_newline(&mut self) {
        let (line, col) = self.cursor_position;
        let mut lines: Vec<String> = self.editor_content.lines().map(|s| s.to_string()).collect();
        
        if line < lines.len() {
            let current_line = lines[line].clone();
            let (before, after) = current_line.split_at(col.min(current_line.len()));
            lines[line] = before.to_string();
            lines.insert(line + 1, after.to_string());
            
            self.editor_content = lines.join("\n");
            self.cursor_position = (line + 1, 0);
        }
    }
    
    fn delete_char_before_cursor(&mut self) {
        let (line, col) = self.cursor_position;
        if col > 0 {
            let mut lines: Vec<String> = self.editor_content.lines().map(|s| s.to_string()).collect();
            if line < lines.len() {
                let current_line = &mut lines[line];
                current_line.remove(col - 1);
                self.editor_content = lines.join("\n");
                self.cursor_position.1 -= 1;
            }
        } else if line > 0 {
            // Join with previous line
            let mut lines: Vec<String> = self.editor_content.lines().map(|s| s.to_string()).collect();
            let current_line = lines.remove(line);
            let prev_line_len = lines[line - 1].len();
            lines[line - 1].push_str(&current_line);
            self.editor_content = lines.join("\n");
            self.cursor_position = (line - 1, prev_line_len);
        }
    }
    
    fn delete_char_at_cursor(&mut self) {
        let (line, col) = self.cursor_position;
        let mut lines: Vec<String> = self.editor_content.lines().map(|s| s.to_string()).collect();
        
        if line < lines.len() && col < lines[line].len() {
            lines[line].remove(col);
            self.editor_content = lines.join("\n");
        }
    }
    
    fn move_cursor_left(&mut self) {
        if self.cursor_position.1 > 0 {
            self.cursor_position.1 -= 1;
        } else if self.cursor_position.0 > 0 {
            self.cursor_position.0 -= 1;
            let lines: Vec<&str> = self.editor_content.lines().collect();
            if self.cursor_position.0 < lines.len() {
                self.cursor_position.1 = lines[self.cursor_position.0].len();
            }
        }
    }
    
    fn move_cursor_right(&mut self) {
        let lines: Vec<&str> = self.editor_content.lines().collect();
        let (line, col) = self.cursor_position;
        
        if line < lines.len() && col < lines[line].len() {
            self.cursor_position.1 += 1;
        } else if line < lines.len() - 1 {
            self.cursor_position = (line + 1, 0);
        }
    }
    
    fn move_cursor_up(&mut self) {
        if self.cursor_position.0 > 0 {
            self.cursor_position.0 -= 1;
            let lines: Vec<&str> = self.editor_content.lines().collect();
            if self.cursor_position.0 < lines.len() {
                self.cursor_position.1 = self.cursor_position.1.min(lines[self.cursor_position.0].len());
            }
        }
    }
    
    fn move_cursor_down(&mut self) {
        let lines: Vec<&str> = self.editor_content.lines().collect();
        if self.cursor_position.0 < lines.len() - 1 {
            self.cursor_position.0 += 1;
            self.cursor_position.1 = self.cursor_position.1.min(lines[self.cursor_position.0].len());
        }
    }
    
    fn move_cursor_to_line_start(&mut self) {
        self.cursor_position.1 = 0;
    }
    
    fn move_cursor_to_line_end(&mut self) {
        let lines: Vec<&str> = self.editor_content.lines().collect();
        if self.cursor_position.0 < lines.len() {
            self.cursor_position.1 = lines[self.cursor_position.0].len();
        }
    }
    
    fn scroll_up(&mut self, lines: usize) {
        self.cursor_position.0 = self.cursor_position.0.saturating_sub(lines);
    }
    
    fn scroll_down(&mut self, lines: usize) {
        let line_count = self.editor_content.lines().count();
        self.cursor_position.0 = (self.cursor_position.0 + lines).min(line_count.saturating_sub(1));
    }
    
    fn parse_editor_content(&self) -> Result<Program> {
        let mut instructions = Vec::new();
        
        for line in self.editor_content.lines() {
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            
            // Remove inline comments
            let line = if let Some(pos) = line.find("//") {
                line[..pos].trim()
            } else {
                line
            };
            
            // Parse instruction
            let parts: Vec<&str> = line.split_whitespace().collect();
            if !parts.is_empty() {
                let mnemonic = parts[0].to_uppercase();
                let args: Vec<u32> = parts[1..]
                    .iter()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                
                match Instruction::parse(&mnemonic, &args) {
                    Ok(inst) => instructions.push(inst),
                    Err(e) => return Err(anyhow::anyhow!("Parse error: {}", e)),
                }
            }
        }
        
        Ok(Program::new(instructions))
    }
    
    fn update_debug_views(&mut self) {
        // Update call stack view
        self.call_stack_view = self.debug_state.call_stack
            .iter()
            .enumerate()
            .map(|(i, &addr)| format!("Frame {}: Return to PC {}", i, addr))
            .collect();
    }
    
    fn update_performance_metrics(&mut self, result: &ExecutionResult) {
        // Update instruction timings
        for entry in &result.trace {
            let inst_name = entry.instruction.mnemonic().to_string();
            self.instruction_timings
                .entry(inst_name)
                .or_insert_with(Vec::new)
                .push(1); // Placeholder timing
        }
        
        // Update hotspots
        let mut pc_counts: HashMap<u32, u64> = HashMap::new();
        for entry in &result.trace {
            *pc_counts.entry(entry.pc).or_insert(0) += 1;
        }
        
        self.hotspots = pc_counts.into_iter().collect();
        self.hotspots.sort_by_key(|&(_, count)| std::cmp::Reverse(count));
    }
}