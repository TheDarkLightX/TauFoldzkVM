use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, AppState, DemoApp, AppSpecificState};
use crate::apps::{
    pacman::GameState,
    smart_contract::TransactionType,
    vending_machine::VendingState,
    crypto_demo::CryptoMode,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    match &app.state {
        AppState::MainMenu => draw_main_menu(f, app),
        AppState::RunningApp(demo_app) => draw_app_screen(f, app, demo_app),
        AppState::Help => draw_help_screen(f),
    }
}

fn draw_main_menu(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Title
    let title = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("TauFold", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("zkVM", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" - "),
            Span::styled("Demo Applications", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from("Zero-Knowledge Virtual Machine with Folding Schemes"),
    ])
    .style(Style::default())
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan)),
    )
    .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    // App list
    let items: Vec<ListItem> = app
        .available_apps
        .iter()
        .enumerate()
        .map(|(i, demo_app)| {
            let selected = i == app.selected_index;
            let style = if selected {
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let content = vec![
                Line::from(vec![
                    if selected {
                        Span::styled("▶ ", Style::default().fg(Color::Green))
                    } else {
                        Span::raw("  ")
                    },
                    Span::styled(demo_app.name(), style),
                ]),
                Line::from(vec![
                    Span::raw("    "),
                    Span::styled(
                        demo_app.description(),
                        Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC),
                    ),
                ]),
                Line::from(""),
            ];

            ListItem::new(content)
        })
        .collect();

    let apps_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Select Application ")
                .title_alignment(Alignment::Center),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(apps_list, chunks[1]);

    // Instructions
    let instructions = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("↑↓", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" Navigate  "),
            Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" Select  "),
            Span::styled("q/Esc", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(" Quit"),
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

fn draw_app_screen(
    f: &mut Frame,
    app: &App,
    demo_app: &DemoApp,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(5),
            Constraint::Length(3),
        ])
        .split(f.size());

    // App header
    let header = Paragraph::new(vec![Line::from(vec![
        Span::styled(
            demo_app.name(),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" - "),
        Span::styled("Running on zkVM", Style::default().fg(Color::Green)),
    ])])
    .style(Style::default())
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);

    // Main content area - split horizontally
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(chunks[1]);

    // App-specific UI
    draw_app_content(f, app, demo_app, content_chunks[0]);

    // zkVM execution visualization
    draw_zkvm_stats(f, app, content_chunks[1]);

    // Execution stats
    draw_execution_stats(f, app, chunks[2]);

    // Controls
    let controls = Paragraph::new(vec![Line::from(vec![
        Span::styled("Esc", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" Back to Menu  "),
        Span::raw("App-specific controls vary"),
    ])])
    .style(Style::default())
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .alignment(Alignment::Center);
    f.render_widget(controls, chunks[3]);
}

fn draw_app_content(
    f: &mut Frame,
    app: &App,
    demo_app: &DemoApp,
    area: Rect,
) {
    match demo_app {
        DemoApp::Calculator => draw_calculator(f, app, area),
        DemoApp::CryptoDemo => draw_crypto_demo(f, app, area),
        DemoApp::PacmanGame => draw_pacman_game(f, app, area),
        DemoApp::SmartContract => draw_smart_contract(f, app, area),
        DemoApp::VendingMachine => draw_vending_machine(f, app, area),
    }
}

fn draw_calculator(f: &mut Frame, app: &App, area: Rect) {
    if let AppSpecificState::Calculator(calc) = &app.app_state {
        let display = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                &calc.display,
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from("┌───┬───┬───┬───┬───┐"),
            Line::from("│ C │ ( │ ) │ % │ ÷ │"),
            Line::from("├───┼───┼───┼───┼───┤"),
            Line::from("│ 7 │ 8 │ 9 │ × │ √ │"),
            Line::from("├───┼───┼───┼───┼───┤"),
            Line::from("│ 4 │ 5 │ 6 │ - │ ^ │"),
            Line::from("├───┼───┼───┼───┼───┤"),
            Line::from("│ 1 │ 2 │ 3 │ + │ M │"),
            Line::from("├───┼───┴───┼───┼───┤"),
            Line::from("│ 0 │   .   │ = │ R │"),
            Line::from("└───┴───────┴───┴───┘"),
            Line::from(""),
            Line::from(vec![
                Span::styled("Memory: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{:.4}", calc.memory),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("m", Style::default().fg(Color::Cyan)),
                Span::raw(" Store  "),
                Span::styled("r", Style::default().fg(Color::Cyan)),
                Span::raw(" Recall"),
            ]),
        ])
        .style(Style::default())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Calculator ")
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Center);
        f.render_widget(display, area);
    }
}

fn draw_crypto_demo(f: &mut Frame, app: &App, area: Rect) {
    if let AppSpecificState::CryptoDemo(crypto) = &app.app_state {
        let mut lines = vec![];
        
        // Mode selection
        lines.push(Line::from("Operation Modes:"));
        lines.push(Line::from(vec![
            if matches!(crypto.mode, CryptoMode::Hash) {
                Span::styled("[1] Hash", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            } else {
                Span::styled(" 1  Hash", Style::default().fg(Color::Gray))
            },
            Span::raw("  "),
            if matches!(crypto.mode, CryptoMode::Sign) {
                Span::styled("[2] Sign", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            } else {
                Span::styled(" 2  Sign", Style::default().fg(Color::Gray))
            },
            Span::raw("  "),
            if matches!(crypto.mode, CryptoMode::Verify) {
                Span::styled("[3] Verify", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            } else {
                Span::styled(" 3  Verify", Style::default().fg(Color::Gray))
            },
            Span::raw("  "),
            if matches!(crypto.mode, CryptoMode::Encrypt) {
                Span::styled("[4] Encrypt", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            } else {
                Span::styled(" 4  Encrypt", Style::default().fg(Color::Gray))
            },
            Span::raw("  "),
            if matches!(crypto.mode, CryptoMode::Decrypt) {
                Span::styled("[5] Decrypt", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            } else {
                Span::styled(" 5  Decrypt", Style::default().fg(Color::Gray))
            },
        ]));
        
        lines.push(Line::from(""));
        lines.push(Line::from("Input Text:"));
        lines.push(Line::from(vec![
            Span::styled(
                &crypto.input_text,
                Style::default().fg(Color::White).add_modifier(Modifier::UNDERLINED),
            ),
            Span::styled("_", Style::default().fg(Color::White).add_modifier(Modifier::SLOW_BLINK)),
        ]));
        
        lines.push(Line::from(""));
        lines.push(Line::from("Results:"));
        
        if let Some(hash) = &crypto.hash_result {
            lines.push(Line::from(vec![
                Span::styled("SHA256: ", Style::default().fg(Color::Cyan)),
                Span::styled(&hash[..32], Style::default().fg(Color::Yellow)),
                Span::raw("..."),
            ]));
        }
        
        if let Some(sig) = &crypto.signature {
            lines.push(Line::from(vec![
                Span::styled("Signature: ", Style::default().fg(Color::Cyan)),
                Span::styled(&sig[..24], Style::default().fg(Color::Yellow)),
                Span::raw("..."),
            ]));
        }
        
        if let Some(verified) = &crypto.verify_result {
            lines.push(Line::from(vec![
                Span::styled("Verification: ", Style::default().fg(Color::Cyan)),
                if *verified {
                    Span::styled("✓ VALID", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                } else {
                    Span::styled("✗ INVALID", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                },
            ]));
        }
        
        if let Some(encrypted) = &crypto.encrypted_data {
            lines.push(Line::from(vec![
                Span::styled("Encrypted: ", Style::default().fg(Color::Cyan)),
                Span::styled(&encrypted[..24], Style::default().fg(Color::Yellow)),
                Span::raw("..."),
            ]));
        }
        
        if let Some(decrypted) = &crypto.decrypted_data {
            lines.push(Line::from(vec![
                Span::styled("Decrypted: ", Style::default().fg(Color::Cyan)),
                Span::styled(decrypted, Style::default().fg(Color::Green)),
            ]));
        }
        
        lines.push(Line::from(""));
        lines.push(Line::from("Keys:"));
        lines.push(Line::from(vec![
            Span::styled("Public: ", Style::default().fg(Color::Gray)),
            Span::styled(&crypto.public_key[..16], Style::default().fg(Color::Blue)),
            Span::raw("..."),
        ]));
        
        lines.push(Line::from(""));
        for msg in crypto.messages.iter().rev().take(3) {
            lines.push(Line::from(vec![
                Span::styled("→ ", Style::default().fg(Color::Green)),
                Span::raw(msg),
            ]));
        }
        
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("Enter", Style::default().fg(Color::Cyan)),
            Span::raw(" Process  "),
            Span::styled("Delete", Style::default().fg(Color::Cyan)),
            Span::raw(" Clear  "),
            Span::styled("Backspace", Style::default().fg(Color::Cyan)),
            Span::raw(" Delete char"),
        ]));
        
        let widget = Paragraph::new(lines)
            .style(Style::default())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Crypto Demo ")
                    .border_type(BorderType::Rounded),
            );
        f.render_widget(widget, area);
    }
}

fn draw_vending_machine(f: &mut Frame, app: &App, area: Rect) {
    if let AppSpecificState::VendingMachine(vending) = &app.app_state {
        let mut lines = vec![];
        
        // Machine display
        lines.push(Line::from("╔═══════════════════════╗"));
        lines.push(Line::from("║  VENDING MACHINE      ║"));
        lines.push(Line::from("╠═══════════════════════╣"));
        
        // Items display
        for (i, item) in vending.inventory.iter().enumerate() {
            let selected = vending.current_state.is_item_selected(i);
            let style = if selected {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else if item.quantity == 0 {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default()
            };
            
            lines.push(Line::from(vec![
                Span::raw("║ "),
                Span::styled(format!("[{}]", i + 1), style),
                Span::raw(" "),
                Span::styled(format!("{:<10}", &item.name), style),
                Span::raw(" "),
                Span::styled(format!("${:.2}", item.price as f32 / 100.0), style),
                Span::raw(" "),
                if item.quantity == 0 {
                    Span::styled("SOLD OUT", Style::default().fg(Color::Red))
                } else {
                    Span::styled(format!("({})", item.quantity), Style::default().fg(Color::Gray))
                },
                Span::raw(" ║"),
            ]));
        }
        
        lines.push(Line::from("╠═══════════════════════╣"));
        
        // Status display
        let status_msg = match &vending.current_state {
            VendingState::Idle => "Insert coins",
            VendingState::ItemSelected(_) => "Insert payment",
            VendingState::AcceptingPayment(_, _) => "Insert more coins",
            VendingState::Dispensing(_) => "Dispensing...",
            VendingState::ReturningChange(_) => "Returning change",
            VendingState::Error(msg) => msg,
        };
        
        lines.push(Line::from(vec![
            Span::raw("║ "),
            Span::styled(
                format!("{:<21}", status_msg),
                match &vending.current_state {
                    VendingState::Error(_) => Style::default().fg(Color::Red),
                    VendingState::Dispensing(_) => Style::default().fg(Color::Green).add_modifier(Modifier::SLOW_BLINK),
                    _ => Style::default().fg(Color::Yellow),
                },
            ),
            Span::raw(" ║"),
        ]));
        
        lines.push(Line::from(vec![
            Span::raw("║ Balance: "),
            Span::styled(
                format!("${:.2}", vending.balance as f32 / 100.0),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
            Span::raw("         ║"),
        ]));
        
        lines.push(Line::from("╚═══════════════════════╝"));
        
        // Messages
        lines.push(Line::from(""));
        for msg in vending.messages.iter().rev().take(3) {
            lines.push(Line::from(vec![
                Span::styled("→ ", Style::default().fg(Color::Green)),
                Span::raw(msg),
            ]));
        }
        
        // Controls
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("1-3", Style::default().fg(Color::Cyan)),
            Span::raw(" Select  "),
            Span::styled("Q", Style::default().fg(Color::Cyan)),
            Span::raw(" Quarter  "),
            Span::styled("D", Style::default().fg(Color::Cyan)),
            Span::raw(" Dime  "),
            Span::styled("N", Style::default().fg(Color::Cyan)),
            Span::raw(" Nickel  "),
            Span::styled("B", Style::default().fg(Color::Cyan)),
            Span::raw(" Bill  "),
            Span::styled("C", Style::default().fg(Color::Cyan)),
            Span::raw(" Cancel"),
        ]));
        
        let widget = Paragraph::new(lines)
            .style(Style::default())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Vending Machine ")
                    .border_type(BorderType::Rounded),
            );
        f.render_widget(widget, area);
    }
}

fn draw_smart_contract(f: &mut Frame, app: &App, area: Rect) {
    if let AppSpecificState::SmartContract(contract) = &app.app_state {
        let mut lines = vec![];
        
        // Contract header
        lines.push(Line::from(vec![
            Span::styled("Contract: ", Style::default().fg(Color::Gray)),
            Span::styled(&contract.name, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" ("),
            Span::styled(&contract.symbol, Style::default().fg(Color::Yellow)),
            Span::raw(")"),
        ]));
        
        lines.push(Line::from(vec![
            Span::styled("Total Supply: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{} {}", contract.total_supply, contract.symbol),
                Style::default().fg(Color::Green),
            ),
            if contract.paused {
                Span::styled(" [PAUSED]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            } else {
                Span::raw("")
            },
        ]));
        
        lines.push(Line::from(""));
        lines.push(Line::from("Account Balances:"));
        
        // Account balances
        for (address, account) in contract.accounts.iter().take(4) {
            let is_owner = address == &contract.owner;
            lines.push(Line::from(vec![
                Span::styled(
                    format!("{:<12}", address),
                    if is_owner {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Blue)
                    },
                ),
                Span::raw(": "),
                Span::styled(
                    format!("{:>8} {}", account.balance, contract.symbol),
                    Style::default().fg(Color::Green),
                ),
                if is_owner {
                    Span::styled(" (Owner)", Style::default().fg(Color::Gray))
                } else {
                    Span::raw("")
                },
            ]));
        }
        
        lines.push(Line::from(""));
        lines.push(Line::from("Recent Transactions:"));
        
        // Recent transactions
        for tx in contract.get_recent_transactions(5) {
            let tx_icon = match tx.tx_type {
                TransactionType::Transfer => "→",
                TransactionType::Mint => "+",
                TransactionType::Burn => "🔥",
                TransactionType::Deploy => "📝",
                TransactionType::Call => "📞",
            };
            
            lines.push(Line::from(vec![
                Span::styled(tx_icon, Style::default().fg(Color::Cyan)),
                Span::raw(" "),
                Span::styled(
                    format!("{} {} from {} to {}", 
                        tx.amount, contract.symbol,
                        &tx.from[..8], &tx.to[..8]
                    ),
                    Style::default().fg(Color::White),
                ),
            ]));
        }
        
        lines.push(Line::from(""));
        for msg in contract.messages.iter().rev().take(3) {
            lines.push(Line::from(vec![
                Span::styled("→ ", Style::default().fg(Color::Green)),
                Span::raw(msg),
            ]));
        }
        
        lines.push(Line::from(""));
        lines.push(Line::from("Operations:"));
        lines.push(Line::from(vec![
            Span::styled("1", Style::default().fg(Color::Cyan)),
            Span::raw(" Transfer  "),
            Span::styled("2", Style::default().fg(Color::Cyan)),
            Span::raw(" Mint  "),
            Span::styled("3", Style::default().fg(Color::Cyan)),
            Span::raw(" Burn  "),
            Span::styled("P", Style::default().fg(Color::Cyan)),
            Span::raw(" Pause  "),
            Span::styled("U", Style::default().fg(Color::Cyan)),
            Span::raw(" Unpause"),
        ]));
        
        let widget = Paragraph::new(lines)
            .style(Style::default())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Smart Contract ")
                    .border_type(BorderType::Rounded),
            );
        f.render_widget(widget, area);
    }
}

fn draw_pacman_game(f: &mut Frame, app: &App, area: Rect) {
    if let AppSpecificState::PacmanGame(game) = &app.app_state {
        let mut lines = vec![];
        
        // Render maze with player and ghosts
        for y in 0..21 {
            let mut line = String::new();
            for x in 0..19 {
                let pos = (x as u8, y as u8);
                
                if game.maze[y][x] {
                    line.push('█');
                } else if game.player_pos == pos {
                    line.push('C');
                } else if game.ghosts.iter().any(|g| g.position == pos) {
                    let ghost = game.ghosts.iter().find(|g| g.position == pos).unwrap();
                    match ghost.mode {
                        crate::apps::pacman::GhostMode::Frightened => line.push('☺'),
                        crate::apps::pacman::GhostMode::Eaten => line.push('\"'),
                        _ => match ghost.color {
                            crate::apps::pacman::GhostColor::Red => line.push('R'),
                            crate::apps::pacman::GhostColor::Pink => line.push('P'),
                            crate::apps::pacman::GhostColor::Blue => line.push('B'),
                            crate::apps::pacman::GhostColor::Orange => line.push('O'),
                        }
                    }
                } else if game.power_pellets.contains(&pos) {
                    line.push('●');
                } else if game.dots.contains(&pos) {
                    line.push('·');
                } else {
                    line.push(' ');
                }
            }
            lines.push(Line::from(line));
        }
        
        lines.push(Line::from(""));
        
        // Score and lives
        let lives_display = "●".repeat(game.lives as usize);
        lines.push(Line::from(vec![
            Span::styled("Score: ", Style::default().fg(Color::Yellow)),
            Span::styled(
                game.score.to_string(),
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled("Lives: ", Style::default().fg(Color::Red)),
            Span::styled(lives_display, Style::default().fg(Color::Yellow)),
            Span::raw("  "),
            match &game.game_state {
                GameState::PowerUp(timer) => Span::styled(
                    format!("POWER UP! {}", timer / 10),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::RAPID_BLINK),
                ),
                GameState::GameOver => Span::styled(
                    "GAME OVER",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                GameState::Victory => Span::styled(
                    "VICTORY!",
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                ),
                GameState::Paused => Span::styled(
                    "PAUSED",
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                ),
                _ => Span::raw(""),
            },
        ]));
        
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("Arrow Keys", Style::default().fg(Color::Cyan)),
            Span::raw(" Move  "),
            Span::styled("P", Style::default().fg(Color::Cyan)),
            Span::raw(" Pause"),
        ]));
        
        let game_widget = Paragraph::new(lines)
            .style(Style::default())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Pacman ")
                    .border_type(BorderType::Rounded),
            )
            .alignment(Alignment::Center);
        f.render_widget(game_widget, area);
    }
}

fn draw_zkvm_stats(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(5)])
        .split(area);

    // Progress gauges
    let progress_block = Block::default()
        .borders(Borders::ALL)
        .title(" zkVM Execution ")
        .border_type(BorderType::Rounded);

    let inner = progress_block.inner(chunks[0]);
    f.render_widget(progress_block, chunks[0]);

    let gauge_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
        ])
        .split(inner);

    // Calculate progress percentages based on typical values
    let folding_percent = if app.is_executing {
        50
    } else if app.execution_stats.folding_steps > 0 {
        100
    } else {
        0
    };
    
    let constraint_percent = if app.is_executing {
        ((app.execution_stats.constraints as f64 / 10000.0) * 100.0).min(100.0) as u16
    } else if app.execution_stats.constraints > 0 {
        100
    } else {
        0
    };
    
    let proof_percent = if app.is_executing {
        30
    } else if app.execution_stats.proof_size > 0 {
        100
    } else {
        0
    };

    let folding_progress = Gauge::default()
        .block(Block::default().title(format!("Folding ({})", app.execution_stats.folding_steps)))
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent(folding_percent);
    f.render_widget(folding_progress, gauge_chunks[0]);

    let constraint_progress = Gauge::default()
        .block(Block::default().title(format!("Constraints ({})", app.execution_stats.constraints)))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(constraint_percent);
    f.render_widget(constraint_progress, gauge_chunks[1]);

    let proof_progress = Gauge::default()
        .block(Block::default().title(format!("Proof ({} KB)", app.execution_stats.proof_size / 1024)))
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(proof_percent);
    f.render_widget(proof_progress, gauge_chunks[2]);

    // Output log
    let output: Vec<ListItem> = app
        .zkvm_output
        .iter()
        .rev()
        .take(10)
        .map(|line| ListItem::new(Line::from(line.as_str())))
        .collect();

    let output_list = List::new(output)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" zkVM Output ")
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Gray));

    f.render_widget(output_list, chunks[1]);
}

fn draw_execution_stats(f: &mut Frame, app: &App, area: Rect) {
    let stats = vec![
        Line::from(vec![
            Span::styled("Cycles: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                app.execution_stats.cycles.to_string(),
                Style::default().fg(Color::White),
            ),
            Span::raw("  "),
            Span::styled("Constraints: ", Style::default().fg(Color::Green)),
            Span::styled(
                app.execution_stats.constraints.to_string(),
                Style::default().fg(Color::White),
            ),
            Span::raw("  "),
            Span::styled("Folding Steps: ", Style::default().fg(Color::Yellow)),
            Span::styled(
                app.execution_stats.folding_steps.to_string(),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Proof Size: ", Style::default().fg(Color::Magenta)),
            Span::styled(
                format!("{} KB", app.execution_stats.proof_size / 1024),
                Style::default().fg(Color::White),
            ),
            Span::raw("  "),
            Span::styled("Verification: ", Style::default().fg(Color::Blue)),
            Span::styled(
                format!("{} ms", app.execution_stats.verification_time_ms),
                Style::default().fg(Color::White),
            ),
        ]),
    ];

    let stats_widget = Paragraph::new(stats)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Execution Statistics ")
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(stats_widget, area);
}

fn draw_help_screen(f: &mut Frame) {
    let help_text = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "TauFoldzkVM Help",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from("Navigation:"),
        Line::from("  ↑/↓     - Navigate menu"),
        Line::from("  Enter   - Select application"),
        Line::from("  Esc     - Go back / Exit"),
        Line::from("  q       - Quit application"),
        Line::from(""),
        Line::from("In Applications:"),
        Line::from("  Each app has specific controls"),
        Line::from("  Calculator: Use number keys and operators"),
        Line::from("  Pacman: Arrow keys to move"),
        Line::from(""),
        Line::from("The zkVM executes all operations with zero-knowledge proofs"),
        Line::from("Watch the execution stats and folding progress in real-time"),
    ];

    let help = Paragraph::new(help_text)
        .style(Style::default())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Help ")
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(help, f.size());
}