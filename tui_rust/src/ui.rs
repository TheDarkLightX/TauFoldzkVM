use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, AppState, DemoApp};

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
    _app: &App,
    demo_app: &DemoApp,
    area: Rect,
) {
    match demo_app {
        DemoApp::Calculator => draw_calculator(f, area),
        DemoApp::PacmanGame => draw_pacman_game(f, area),
        _ => {
            let placeholder = Paragraph::new("App UI Coming Soon...")
                .style(Style::default().fg(Color::Gray))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Application Display ")
                        .border_type(BorderType::Rounded),
                )
                .alignment(Alignment::Center);
            f.render_widget(placeholder, area);
        }
    }
}

fn draw_calculator(f: &mut Frame, area: Rect) {
    let display = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "0",
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from("┌───┬───┬───┬───┐"),
        Line::from("│ 7 │ 8 │ 9 │ ÷ │"),
        Line::from("├───┼───┼───┼───┤"),
        Line::from("│ 4 │ 5 │ 6 │ × │"),
        Line::from("├───┼───┼───┼───┤"),
        Line::from("│ 1 │ 2 │ 3 │ - │"),
        Line::from("├───┼───┼───┼───┤"),
        Line::from("│ 0 │ . │ = │ + │"),
        Line::from("└───┴───┴───┴───┘"),
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

fn draw_pacman_game(f: &mut Frame, area: Rect) {
    let game = Paragraph::new(vec![
        Line::from("╔═══════════════════╗"),
        Line::from("║·········•·········║"),
        Line::from("║·┌─┐·┌─────┐·┌─┐·║"),
        Line::from("║•│ │·│     │·│ │•║"),
        Line::from("║·└─┘·└─────┘·└─┘·║"),
        Line::from("║·········C·········║"),
        Line::from("║·┌─┐·┌┐ ┌┐·┌─┐·║"),
        Line::from("║·└─┘·││ ││·└─┘·║"),
        Line::from("║·····││ ││·····║"),
        Line::from("╚═══════════════════╝"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Score: ", Style::default().fg(Color::Yellow)),
            Span::styled("0", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled("Lives: ", Style::default().fg(Color::Red)),
            Span::styled("●●●", Style::default().fg(Color::Yellow)),
        ]),
    ])
    .style(Style::default())
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Pacman ")
            .border_type(BorderType::Rounded),
    )
    .alignment(Alignment::Center);
    f.render_widget(game, area);
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

    let folding_progress = Gauge::default()
        .block(Block::default().title("Folding"))
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent(75);
    f.render_widget(folding_progress, gauge_chunks[0]);

    let constraint_progress = Gauge::default()
        .block(Block::default().title("Constraints"))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(60);
    f.render_widget(constraint_progress, gauge_chunks[1]);

    let proof_progress = Gauge::default()
        .block(Block::default().title("Proof Generation"))
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(40);
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