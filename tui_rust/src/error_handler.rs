use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

pub struct ErrorDialog {
    pub message: String,
    pub details: Option<String>,
    pub show_help: bool,
}

impl ErrorDialog {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            details: None,
            show_help: false,
        }
    }

    pub fn with_details(mut self, details: &str) -> Self {
        self.details = Some(details.to_string());
        self
    }

    pub fn with_help(mut self) -> Self {
        self.show_help = true;
        self
    }

    pub fn draw(&self, f: &mut Frame) {
        // Create a centered popup
        let popup_area = center_rect(50, 30, f.size());
        
        // Clear the area
        f.render_widget(Clear, popup_area);
        
        // Create layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),    // Title
                Constraint::Min(3),       // Message
                Constraint::Length(if self.details.is_some() { 4 } else { 0 }),  // Details
                Constraint::Length(if self.show_help { 3 } else { 2 }),  // Instructions
            ])
            .split(popup_area);

        // Background
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Red))
            .style(Style::default().bg(Color::Black));
        f.render_widget(block, popup_area);

        // Title
        let title = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("❌ Error", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            ])
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Red)),
        );
        f.render_widget(title, chunks[0]);

        // Message
        let message_lines = wrap_text(&self.message, (chunks[1].width - 4) as usize);
        let message = Paragraph::new(message_lines)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(message, chunks[1]);

        // Details (if any)
        if let Some(details) = &self.details {
            let detail_lines = wrap_text(details, (chunks[2].width - 4) as usize);
            let details_widget = Paragraph::new(detail_lines)
                .style(Style::default().fg(Color::Gray))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::TOP)
                        .border_type(BorderType::Plain)
                        .border_style(Style::default().fg(Color::Gray)),
                );
            f.render_widget(details_widget, chunks[2]);
        }

        // Instructions
        let instructions_chunk = if self.details.is_some() { chunks[3] } else { chunks[2] };
        
        let mut instruction_lines = vec![
            Line::from(vec![
                Span::styled("Press ", Style::default().fg(Color::Gray)),
                Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled(" to continue", Style::default().fg(Color::Gray)),
            ])
        ];

        if self.show_help {
            instruction_lines.push(Line::from(vec![
                Span::styled("Press ", Style::default().fg(Color::Gray)),
                Span::styled("?", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(" for help", Style::default().fg(Color::Gray)),
            ]));
        }

        let instructions = Paragraph::new(instruction_lines)
            .alignment(Alignment::Center);
        f.render_widget(instructions, instructions_chunk);
    }
}

// Helper function to center a rectangle
fn center_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

// Helper function to wrap text
fn wrap_text(text: &str, width: usize) -> Vec<Line> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in words {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + word.len() + 1 <= width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(Line::from(current_line.clone()));
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() {
        lines.push(Line::from(current_line));
    }

    if lines.is_empty() {
        lines.push(Line::from(""));
    }

    lines
}

// Common error messages
impl ErrorDialog {
    pub fn zkvm_execution_failed(error: &str) -> Self {
        ErrorDialog::new("zkVM Execution Failed")
            .with_details(&format!("The zero-knowledge virtual machine encountered an error: {}", error))
            .with_help()
    }

    pub fn tau_not_found() -> Self {
        ErrorDialog::new("Tau Runtime Not Found")
            .with_details("The Tau language runtime is not installed or not in PATH. Running in demo mode.")
    }

    pub fn invalid_input(field: &str) -> Self {
        ErrorDialog::new("Invalid Input")
            .with_details(&format!("Please check your input for: {}", field))
            .with_help()
    }

    pub fn calculation_error(operation: &str) -> Self {
        ErrorDialog::new("Calculation Error")
            .with_details(&format!("Error performing {}: Division by zero or invalid operation", operation))
    }

    pub fn network_error() -> Self {
        ErrorDialog::new("Network Error")
            .with_details("Failed to connect to distributed proving network. Operating in local mode.")
    }
}