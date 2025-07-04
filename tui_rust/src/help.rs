use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::{AppState, DemoApp};

pub fn draw_help_screen(f: &mut Frame, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),    // Title
            Constraint::Min(10),      // Content
            Constraint::Length(3),    // Footer
        ])
        .split(f.size());

    // Title
    let title = match app_state {
        AppState::Welcome => "Welcome Help",
        AppState::MainMenu => "Main Menu Help",
        AppState::RunningApp(app) => match app {
            DemoApp::Calculator => "Calculator Help",
            DemoApp::CryptoDemo => "Crypto Demo Help",
            DemoApp::PacmanGame => "Pacman Game Help",
            DemoApp::SmartContract => "Smart Contract Help",
            DemoApp::VendingMachine => "Vending Machine Help",
        },
        AppState::Help => "General Help",
    };

    let title_widget = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(title, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ])
    ])
    .style(Style::default())
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .alignment(Alignment::Center);
    f.render_widget(title_widget, chunks[0]);

    // Content based on current state
    let help_content = get_help_content(app_state);
    
    let content_widget = Paragraph::new(help_content)
        .style(Style::default())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(content_widget, chunks[1]);

    // Footer
    let footer = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Press ", Style::default().fg(Color::Gray)),
            Span::styled("Esc", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(" to close help", Style::default().fg(Color::Gray)),
        ])
    ])
    .style(Style::default())
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);
}

fn get_help_content(app_state: &AppState) -> Vec<Line> {
    match app_state {
        AppState::Welcome => vec![
            Line::from(vec![
                Span::styled("Navigation:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Enter", Style::default().fg(Color::Green)),
                Span::raw("  - Continue to main menu"),
            ]),
            Line::from(vec![
                Span::styled("q/Esc", Style::default().fg(Color::Red)),
                Span::raw("  - Exit application"),
            ]),
            Line::from(""),
            Line::from("This welcome screen appears only on first run."),
            Line::from("It explains the five demo applications and zkVM concepts."),
        ],
        
        AppState::MainMenu => vec![
            Line::from(vec![
                Span::styled("Navigation:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("↑/↓", Style::default().fg(Color::Green)),
                Span::raw("     - Navigate between applications"),
            ]),
            Line::from(vec![
                Span::styled("Enter", Style::default().fg(Color::Green)),
                Span::raw("   - Select highlighted application"),
            ]),
            Line::from(vec![
                Span::styled("?/F1", Style::default().fg(Color::Cyan)),
                Span::raw("    - Show this help screen"),
            ]),
            Line::from(vec![
                Span::styled("q/Esc", Style::default().fg(Color::Red)),
                Span::raw("   - Quit application"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Available Applications:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from("🧮 Calculator - Arithmetic with zkVM proofs"),
            Line::from("🔐 Crypto Demo - Hash, sign, encrypt operations"),
            Line::from("👾 Pacman - Full game with ghost AI"),
            Line::from("💰 Smart Contract - Token operations"),
            Line::from("🥤 Vending Machine - FSM demonstration"),
        ],
        
        AppState::RunningApp(app) => get_app_specific_help(app),
        
        AppState::Help => vec![
            Line::from(vec![
                Span::styled("Global Keyboard Shortcuts:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("?/F1", Style::default().fg(Color::Cyan)),
                Span::raw("    - Show help (context-sensitive)"),
            ]),
            Line::from(vec![
                Span::styled("Esc", Style::default().fg(Color::Red)),
                Span::raw("     - Go back / Exit"),
            ]),
            Line::from(vec![
                Span::styled("q", Style::default().fg(Color::Red)),
                Span::raw("       - Quit application"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("About zkVM Mode:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            if std::env::var("ZKVM_DEMO_MODE").is_ok() {
                Line::from(vec![
                    Span::styled("Current Mode: ", Style::default().fg(Color::Gray)),
                    Span::styled("Demo Mode", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ])
            } else {
                Line::from(vec![
                    Span::styled("Current Mode: ", Style::default().fg(Color::Gray)),
                    Span::styled("Full zkVM", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                ])
            },
            Line::from(""),
            Line::from("Demo Mode: Applications work fully, zkVM stats simulated"),
            Line::from("Full zkVM: Real zero-knowledge proofs via Tau runtime"),
            Line::from(""),
            Line::from("Both modes provide complete application functionality."),
            Line::from("The difference is in proof generation and verification."),
        ],
    }
}

fn get_app_specific_help(app: &DemoApp) -> Vec<Line> {
    match app {
        DemoApp::Calculator => vec![
            Line::from(vec![
                Span::styled("Calculator Controls:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Numbers:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("0-9", Style::default().fg(Color::Green)),
                Span::raw("     - Input digits"),
            ]),
            Line::from(vec![
                Span::styled(".", Style::default().fg(Color::Green)),
                Span::raw("       - Decimal point"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Operations:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("+", Style::default().fg(Color::Green)),
                Span::raw("       - Addition"),
            ]),
            Line::from(vec![
                Span::styled("-", Style::default().fg(Color::Green)),
                Span::raw("       - Subtraction"),
            ]),
            Line::from(vec![
                Span::styled("*", Style::default().fg(Color::Green)),
                Span::raw("       - Multiplication"),
            ]),
            Line::from(vec![
                Span::styled("/", Style::default().fg(Color::Green)),
                Span::raw("       - Division"),
            ]),
            Line::from(vec![
                Span::styled("=", Style::default().fg(Color::Green)),
                Span::raw("       - Calculate result"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Memory:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("m", Style::default().fg(Color::Green)),
                Span::raw("       - Store in memory"),
            ]),
            Line::from(vec![
                Span::styled("r", Style::default().fg(Color::Green)),
                Span::raw("       - Recall from memory"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Other:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("c", Style::default().fg(Color::Green)),
                Span::raw("       - Clear display"),
            ]),
            Line::from(vec![
                Span::styled("Esc", Style::default().fg(Color::Red)),
                Span::raw("     - Return to menu"),
            ]),
            Line::from(""),
            Line::from("Each calculation generates a zero-knowledge proof!"),
        ],
        
        DemoApp::CryptoDemo => vec![
            Line::from(vec![
                Span::styled("Crypto Demo Controls:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Mode Selection:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("1", Style::default().fg(Color::Green)),
                Span::raw("       - Hash Mode (SHA256)"),
            ]),
            Line::from(vec![
                Span::styled("2", Style::default().fg(Color::Green)),
                Span::raw("       - Sign Mode (Digital Signature)"),
            ]),
            Line::from(vec![
                Span::styled("3", Style::default().fg(Color::Green)),
                Span::raw("       - Verify Mode (Check Signature)"),
            ]),
            Line::from(vec![
                Span::styled("4", Style::default().fg(Color::Green)),
                Span::raw("       - Encrypt Mode (Symmetric)"),
            ]),
            Line::from(vec![
                Span::styled("5", Style::default().fg(Color::Green)),
                Span::raw("       - Decrypt Mode"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Text Input:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("a-z A-Z", Style::default().fg(Color::Green)),
                Span::raw("  - Type text to process"),
            ]),
            Line::from(vec![
                Span::styled("Space", Style::default().fg(Color::Green)),
                Span::raw("    - Add space"),
            ]),
            Line::from(vec![
                Span::styled("Backspace", Style::default().fg(Color::Green)),
                Span::raw(" - Delete character"),
            ]),
            Line::from(vec![
                Span::styled("Delete", Style::default().fg(Color::Green)),
                Span::raw("   - Clear all input"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Actions:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("Enter", Style::default().fg(Color::Green)),
                Span::raw("    - Process with current mode"),
            ]),
            Line::from(vec![
                Span::styled("Esc", Style::default().fg(Color::Red)),
                Span::raw("      - Return to menu"),
            ]),
            Line::from(""),
            Line::from("All operations use zero-knowledge proofs for privacy!"),
        ],
        
        DemoApp::PacmanGame => vec![
            Line::from(vec![
                Span::styled("Pacman Game Controls:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Movement:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("↑", Style::default().fg(Color::Green)),
                Span::raw("       - Move Up"),
            ]),
            Line::from(vec![
                Span::styled("↓", Style::default().fg(Color::Green)),
                Span::raw("       - Move Down"),
            ]),
            Line::from(vec![
                Span::styled("←", Style::default().fg(Color::Green)),
                Span::raw("       - Move Left"),
            ]),
            Line::from(vec![
                Span::styled("→", Style::default().fg(Color::Green)),
                Span::raw("       - Move Right"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Game Controls:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("p", Style::default().fg(Color::Green)),
                Span::raw("       - Pause/Resume game"),
            ]),
            Line::from(vec![
                Span::styled("Esc", Style::default().fg(Color::Red)),
                Span::raw("     - Return to menu"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Game Elements:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("C", Style::default().fg(Color::Yellow)),
                Span::raw("       - Your character (Pacman)"),
            ]),
            Line::from(vec![
                Span::styled("R/P/B/O", Style::default().fg(Color::Magenta)),
                Span::raw(" - Ghosts (Red/Pink/Blue/Orange)"),
            ]),
            Line::from(vec![
                Span::styled("·", Style::default().fg(Color::White)),
                Span::raw("       - Dots (collect for points)"),
            ]),
            Line::from(vec![
                Span::styled("●", Style::default().fg(Color::Yellow)),
                Span::raw("       - Power pellets (eat ghosts!)"),
            ]),
            Line::from(vec![
                Span::styled("█", Style::default().fg(Color::Blue)),
                Span::raw("       - Walls"),
            ]),
            Line::from(""),
            Line::from("Objective: Collect all dots while avoiding ghosts!"),
            Line::from("Every move is cryptographically verified by zkVM."),
        ],
        
        DemoApp::SmartContract => vec![
            Line::from(vec![
                Span::styled("Smart Contract Controls:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Token Operations:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("1", Style::default().fg(Color::Green)),
                Span::raw("       - Transfer tokens (100 TAU)"),
            ]),
            Line::from(vec![
                Span::styled("2", Style::default().fg(Color::Green)),
                Span::raw("       - Mint tokens (500 TAU)"),
            ]),
            Line::from(vec![
                Span::styled("3", Style::default().fg(Color::Green)),
                Span::raw("       - Burn tokens (50 TAU)"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Contract Management:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("p", Style::default().fg(Color::Green)),
                Span::raw("       - Pause contract"),
            ]),
            Line::from(vec![
                Span::styled("u", Style::default().fg(Color::Green)),
                Span::raw("       - Unpause contract"),
            ]),
            Line::from(vec![
                Span::styled("Esc", Style::default().fg(Color::Red)),
                Span::raw("     - Return to menu"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Contract Features:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from("• ERC20-like token (TAU)"),
            Line::from("• Owner-only minting"),
            Line::from("• Transaction history"),
            Line::from("• Pausable functionality"),
            Line::from("• Balance verification"),
            Line::from(""),
            Line::from("All operations executed in zero-knowledge VM!"),
            Line::from("Contract state changes are cryptographically proven."),
        ],
        
        DemoApp::VendingMachine => vec![
            Line::from(vec![
                Span::styled("Vending Machine Controls:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Item Selection:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("1-8", Style::default().fg(Color::Green)),
                Span::raw("     - Select item number"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Payment:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("q", Style::default().fg(Color::Green)),
                Span::raw("       - Insert Quarter ($0.25)"),
            ]),
            Line::from(vec![
                Span::styled("d", Style::default().fg(Color::Green)),
                Span::raw("       - Insert Dime ($0.10)"),
            ]),
            Line::from(vec![
                Span::styled("n", Style::default().fg(Color::Green)),
                Span::raw("       - Insert Nickel ($0.05)"),
            ]),
            Line::from(vec![
                Span::styled("b", Style::default().fg(Color::Green)),
                Span::raw("       - Insert Bill ($1.00)"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Actions:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("c", Style::default().fg(Color::Green)),
                Span::raw("       - Cancel transaction"),
            ]),
            Line::from(vec![
                Span::styled("Esc", Style::default().fg(Color::Red)),
                Span::raw("     - Return to menu"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("State Machine:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from("Idle → Item Selected → Payment → Dispensing → Change"),
            Line::from(""),
            Line::from("Process:"),
            Line::from("1. Select item with number key"),
            Line::from("2. Insert coins/bills until sufficient payment"),
            Line::from("3. Machine dispenses item and returns change"),
            Line::from(""),
            Line::from("Every state transition proven by zero-knowledge VM!"),
        ],
    }
}

pub fn should_show_help_hint(app_state: &AppState) -> bool {
    !matches!(app_state, AppState::Help)
}

pub fn get_status_bar_text(app_state: &AppState) -> Vec<Span> {
    let mode_text = if std::env::var("ZKVM_DEMO_MODE").is_ok() {
        vec![
            Span::styled("Demo Mode", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
        ]
    } else {
        vec![
            Span::styled("zkVM", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
        ]
    };

    let help_hint = vec![
        Span::styled("?", Style::default().fg(Color::Cyan)),
        Span::raw(" Help | "),
        Span::styled("Esc", Style::default().fg(Color::Red)),
        Span::raw(" Back"),
    ];

    [mode_text, help_hint].concat()
}