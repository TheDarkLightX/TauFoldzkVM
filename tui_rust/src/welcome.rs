use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn should_show_welcome() -> bool {
    // Check if this is the first run
    let config_dir = dirs::config_dir()
        .map(|d| d.join("tau-zkvm-tui"))
        .unwrap_or_default();
    
    let welcome_file = config_dir.join(".welcomed");
    !welcome_file.exists()
}

pub fn mark_welcomed() {
    if let Some(config_dir) = dirs::config_dir() {
        let dir = config_dir.join("tau-zkvm-tui");
        std::fs::create_dir_all(&dir).ok();
        std::fs::write(dir.join(".welcomed"), "").ok();
    }
}

pub fn draw_welcome_screen(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(8),    // Title
            Constraint::Min(10),      // Content
            Constraint::Length(4),    // Instructions
        ])
        .split(f.size());

    // Title
    let title_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Welcome to ", Style::default().fg(Color::White)),
            Span::styled("TauFold", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("zkVM", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(" Interactive Demo", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from("Experience Zero-Knowledge Virtual Machines Through Interactive Applications"),
        Line::from(""),
    ];
    
    let title = Paragraph::new(title_lines)
        .style(Style::default())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    // Content
    let content_lines = vec![
        Line::from(vec![
            Span::styled("What You'll Explore:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("🧮 Calculator", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" - Every calculation generates a zero-knowledge proof"),
        ]),
        Line::from(vec![
            Span::raw("   Verify arithmetic without revealing the numbers!"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("🔐 Crypto Demo", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" - Hash, sign, and encrypt with ZK guarantees"),
        ]),
        Line::from(vec![
            Span::raw("   Cryptographic operations with privacy preservation"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("👾 Pacman Game", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" - Play a game where every move is cryptographically verified"),
        ]),
        Line::from(vec![
            Span::raw("   Prove you won fairly without revealing your strategy!"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("💰 Smart Contract", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" - Execute token transfers in a zero-knowledge VM"),
        ]),
        Line::from(vec![
            Span::raw("   Private transactions with public verification"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("🥤 Vending Machine", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" - A finite state machine with proof of correct operation"),
        ]),
        Line::from(vec![
            Span::raw("   Demonstrate state transitions are valid without revealing state"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("How It Works:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from("• Each application runs inside a zero-knowledge virtual machine"),
        Line::from("• Every operation generates cryptographic proofs using folding schemes"),
        Line::from("• Watch real-time constraint generation and proof statistics"),
        Line::from("• All computations are verifiable without revealing inputs"),
        Line::from(""),
        if std::env::var("ZKVM_DEMO_MODE").is_ok() {
            Line::from(vec![
                Span::styled("ℹ️  Running in ", Style::default().fg(Color::Gray)),
                Span::styled("Demo Mode", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(" (Tau not detected)", Style::default().fg(Color::Gray)),
            ])
        } else {
            Line::from(vec![
                Span::styled("✅ ", Style::default().fg(Color::Green)),
                Span::styled("Tau Runtime Detected", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled(" - Full zkVM functionality enabled", Style::default().fg(Color::Gray)),
            ])
        },
    ];
    
    let content = Paragraph::new(content_lines)
        .style(Style::default())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(content, chunks[1]);

    // Instructions
    let instructions = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Press ", Style::default().fg(Color::White)),
            Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(" to start exploring", Style::default().fg(Color::White)),
        ]),
    ])
    .style(Style::default())
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .alignment(Alignment::Center);
    f.render_widget(instructions, chunks[2]);
}