//! TauFoldZKVM Terminal User Interface
//!
//! Interactive TUI for developing, debugging, and running zkVM programs.

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Clear, List, ListItem, ListState, Paragraph, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Tabs, Wrap,
    },
    Frame, Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};
use taufold_zkvm::{Instruction, Program, VirtualMachine, VmState};

mod app;
mod code_editor;
mod debugger;
mod executor;
mod file_browser;

use app::{App, AppMode, TabIndex};

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.mode {
                    AppMode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Tab => app.next_tab(),
                        KeyCode::BackTab => app.previous_tab(),
                        KeyCode::Char('e') => app.mode = AppMode::Editor,
                        KeyCode::Char('d') => app.mode = AppMode::Debugger,
                        KeyCode::Char('r') => app.run_program()?,
                        KeyCode::Char('o') => app.open_file()?,
                        KeyCode::Char('s') => app.save_file()?,
                        KeyCode::Char('n') => app.new_file(),
                        KeyCode::Char('?') => app.mode = AppMode::Help,
                        _ => {}
                    },
                    AppMode::Editor => match key.code {
                        KeyCode::Esc => app.mode = AppMode::Normal,
                        _ => app.handle_editor_input(key.code)?,
                    },
                    AppMode::Debugger => match key.code {
                        KeyCode::Esc => app.mode = AppMode::Normal,
                        KeyCode::Char('n') => app.debug_step()?,
                        KeyCode::Char('c') => app.debug_continue()?,
                        KeyCode::Char('b') => app.toggle_breakpoint()?,
                        KeyCode::Char('r') => app.debug_restart()?,
                        _ => {}
                    },
                    AppMode::Help => match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => app.mode = AppMode::Normal,
                        _ => {}
                    },
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Main content
            Constraint::Length(3),  // Status bar
        ])
        .split(f.size());

    // Header with tabs
    let titles = vec!["Editor", "Debugger", "Execution", "Memory", "Constraints"];
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(" TauFoldZKVM TUI "))
        .select(app.current_tab as usize)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(tabs, chunks[0]);

    // Main content area
    match app.current_tab {
        TabIndex::Editor => render_editor(f, app, chunks[1]),
        TabIndex::Debugger => render_debugger(f, app, chunks[1]),
        TabIndex::Execution => render_execution(f, app, chunks[1]),
        TabIndex::Memory => render_memory(f, app, chunks[1]),
        TabIndex::Constraints => render_constraints(f, app, chunks[1]),
    }

    // Status bar
    render_status_bar(f, app, chunks[2]);

    // Help overlay if needed
    if matches!(app.mode, AppMode::Help) {
        render_help_overlay(f, f.size());
    }
}

fn render_editor<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    // Code editor
    let editor_block = Block::default()
        .borders(Borders::ALL)
        .title(" Code Editor ")
        .border_style(if matches!(app.mode, AppMode::Editor) {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        });

    let code_lines: Vec<ListItem> = app
        .editor_content
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let line_number = format!("{:4} ", i + 1);
            let spans = vec![
                Span::styled(line_number, Style::default().fg(Color::DarkGray)),
                Span::raw(line),
            ];
            ListItem::new(Line::from(spans))
        })
        .collect();

    let code_list = List::new(code_lines)
        .block(editor_block)
        .style(Style::default().fg(Color::White));

    f.render_stateful_widget(code_list, chunks[0], &mut app.editor_state);

    // Instruction palette
    let instructions_block = Block::default()
        .borders(Borders::ALL)
        .title(" Instructions ");

    let instructions: Vec<ListItem> = vec![
        "Arithmetic: ADD, SUB, MUL, DIV, MOD",
        "Bitwise: AND, OR, XOR, NOT, SHL, SHR",
        "Comparison: EQ, NEQ, LT, GT, LTE, GTE",
        "Memory: LOAD, STORE, PUSH, POP",
        "Control: JMP, JZ, JNZ, CALL, RET",
        "Crypto: HASH, VERIFY, SIGN",
        "System: HALT, NOP, DEBUG",
    ]
    .iter()
    .map(|i| ListItem::new(*i))
    .collect();

    let instructions_list = List::new(instructions)
        .block(instructions_block)
        .style(Style::default().fg(Color::Green));

    f.render_widget(instructions_list, chunks[1]);
}

fn render_debugger<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(chunks[0]);

    // Code view with current line highlighted
    let code_block = Block::default()
        .borders(Borders::ALL)
        .title(" Code (Line: {}) ");

    let code_lines: Vec<ListItem> = app
        .current_program
        .as_ref()
        .map(|p| &p.instructions)
        .unwrap_or(&vec![])
        .iter()
        .enumerate()
        .map(|(i, inst)| {
            let is_current = i == app.debug_state.program_counter as usize;
            let has_breakpoint = app.breakpoints.contains(&(i as u32));
            
            let mut style = Style::default();
            if is_current {
                style = style.bg(Color::Blue).fg(Color::White);
            }
            if has_breakpoint {
                style = style.fg(Color::Red);
            }

            let marker = if has_breakpoint { "●" } else { " " };
            let line = format!("{} {:4} {}", marker, i, inst);
            ListItem::new(line).style(style)
        })
        .collect();

    let code_list = List::new(code_lines).block(code_block);
    f.render_widget(code_list, left_chunks[0]);

    // Breakpoints list
    let breakpoints_block = Block::default()
        .borders(Borders::ALL)
        .title(" Breakpoints ");

    let breakpoint_items: Vec<ListItem> = app
        .breakpoints
        .iter()
        .map(|&bp| ListItem::new(format!("Line {}", bp)))
        .collect();

    let breakpoints_list = List::new(breakpoint_items).block(breakpoints_block);
    f.render_widget(breakpoints_list, left_chunks[1]);

    // Right side - Stack and Registers
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Stack view
    let stack_block = Block::default()
        .borders(Borders::ALL)
        .title(" Stack ");

    let stack_items: Vec<ListItem> = app
        .debug_state
        .stack
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &val)| {
            let style = if i == 0 {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            };
            ListItem::new(format!("[{}] 0x{:08X} ({})", i, val, val)).style(style)
        })
        .collect();

    let stack_list = List::new(stack_items).block(stack_block);
    f.render_widget(stack_list, right_chunks[0]);

    // Registers view
    let registers_block = Block::default()
        .borders(Borders::ALL)
        .title(" Registers ");

    let register_text = app
        .debug_state
        .registers
        .iter()
        .enumerate()
        .map(|(i, &val)| format!("R{}: 0x{:08X}", i, val))
        .collect::<Vec<_>>()
        .join("\n");

    let registers = Paragraph::new(register_text)
        .block(registers_block)
        .wrap(Wrap { trim: true });

    f.render_widget(registers, right_chunks[1]);
}

fn render_execution<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    // Execution trace
    let trace_block = Block::default()
        .borders(Borders::ALL)
        .title(" Execution Trace ");

    let trace_items: Vec<ListItem> = app
        .execution_trace
        .iter()
        .map(|entry| {
            ListItem::new(format!(
                "[Cycle {}] PC: {} | {} | Stack: {:?}",
                entry.cycle, entry.pc, entry.instruction, entry.stack_after
            ))
        })
        .collect();

    let trace_list = List::new(trace_items)
        .block(trace_block)
        .style(Style::default().fg(Color::Yellow));

    f.render_stateful_widget(trace_list, chunks[0], &mut app.trace_state);

    // Performance metrics
    let metrics_block = Block::default()
        .borders(Borders::ALL)
        .title(" Performance Metrics ");

    let metrics_text = if let Some(result) = &app.last_execution_result {
        format!(
            "Status: {}\nCycles: {}\nInstructions: {}\nExecution Time: {} ms\nInstructions/sec: {:.2}\nConstraint Validations: {}\nConstraint Violations: {}",
            if result.success { "SUCCESS" } else { "FAILED" },
            result.stats.cycles_executed,
            result.stats.instructions_executed,
            result.stats.execution_time_ms,
            result.stats.instructions_per_second,
            result.stats.constraint_validations,
            result.stats.constraint_violations
        )
    } else {
        "No execution data available.\nPress 'r' to run a program.".to_string()
    };

    let metrics = Paragraph::new(metrics_text)
        .block(metrics_block)
        .style(Style::default().fg(Color::Green));

    f.render_widget(metrics, chunks[1]);
}

fn render_memory<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    // Memory view
    let memory_block = Block::default()
        .borders(Borders::ALL)
        .title(" Memory View ");

    let memory_lines: Vec<ListItem> = (0..256)
        .step_by(4)
        .map(|addr| {
            let values: Vec<String> = (0..4)
                .map(|i| {
                    let idx = addr + i;
                    if idx < app.debug_state.memory.len() {
                        format!("{:08X}", app.debug_state.memory[idx])
                    } else {
                        "--------".to_string()
                    }
                })
                .collect();
            
            ListItem::new(format!(
                "0x{:04X}: {} {} {} {}",
                addr, values[0], values[1], values[2], values[3]
            ))
        })
        .collect();

    let memory_list = List::new(memory_lines)
        .block(memory_block)
        .style(Style::default().fg(Color::White));

    f.render_stateful_widget(memory_list, chunks[0], &mut app.memory_state);

    // Memory statistics
    let stats_block = Block::default()
        .borders(Borders::ALL)
        .title(" Memory Stats ");

    let non_zero_count = app.debug_state.memory.iter().filter(|&&v| v != 0).count();
    let stats_text = format!(
        "Total Size: {} words\nUsed: {} words\nFree: {} words\nUtilization: {:.1}%",
        app.debug_state.memory.len(),
        non_zero_count,
        app.debug_state.memory.len() - non_zero_count,
        (non_zero_count as f64 / app.debug_state.memory.len() as f64) * 100.0
    );

    let stats = Paragraph::new(stats_text)
        .block(stats_block)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(stats, chunks[1]);
}

fn render_constraints<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    // Constraint violations
    let violations_block = Block::default()
        .borders(Borders::ALL)
        .title(" Constraint Violations ");

    let violation_items: Vec<ListItem> = if app.constraint_violations.is_empty() {
        vec![ListItem::new("No constraint violations detected ✓").style(Style::default().fg(Color::Green))]
    } else {
        app.constraint_violations
            .iter()
            .map(|v| {
                ListItem::new(format!(
                    "[Cycle {}] {} - {}",
                    v.cycle, v.instruction, v.details
                )).style(Style::default().fg(Color::Red))
            })
            .collect()
    };

    let violations_list = List::new(violation_items).block(violations_block);
    f.render_widget(violations_list, chunks[0]);

    // Constraint statistics
    let stats_block = Block::default()
        .borders(Borders::ALL)
        .title(" Constraint Statistics ");

    let stats_text = format!(
        "Total Constraints: ~40,000 per step\nConstraint Budget Used: ~700 per instruction (~1.75%)\n\nInstruction Complexity:\n- Arithmetic: ~200 constraints\n- Bitwise: ~64 constraints\n- Comparison: ~120 constraints\n- Memory: ~96 constraints\n- Control Flow: ~80 constraints\n- Cryptographic: ~280 constraints"
    );

    let stats = Paragraph::new(stats_text)
        .block(stats_block)
        .style(Style::default().fg(Color::Magenta));

    f.render_widget(stats, chunks[1]);
}

fn render_status_bar<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(area);

    // Mode indicator
    let mode_text = match app.mode {
        AppMode::Normal => "NORMAL",
        AppMode::Editor => "EDITOR",
        AppMode::Debugger => "DEBUG",
        AppMode::Help => "HELP",
    };
    
    let mode_color = match app.mode {
        AppMode::Normal => Color::Green,
        AppMode::Editor => Color::Blue,
        AppMode::Debugger => Color::Yellow,
        AppMode::Help => Color::Cyan,
    };

    let mode = Paragraph::new(format!(" {} ", mode_text))
        .style(Style::default().bg(mode_color).fg(Color::Black))
        .alignment(Alignment::Center);
    f.render_widget(mode, chunks[0]);

    // File info
    let file_info = Paragraph::new(format!(
        " {} {}",
        app.current_file.as_ref().unwrap_or(&"[New File]".to_string()),
        if app.is_modified { "*" } else { "" }
    ))
    .alignment(Alignment::Center);
    f.render_widget(file_info, chunks[1]);

    // Help hint
    let help = Paragraph::new(" Press ? for help ")
        .alignment(Alignment::Right);
    f.render_widget(help, chunks[2]);
}

fn render_help_overlay<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let help_text = r#"
╭─────────────────────── Help ───────────────────────╮
│                                                     │
│  Global Commands:                                   │
│    Tab/Shift+Tab : Switch between tabs             │
│    q            : Quit (in Normal mode)            │
│    ?            : Show this help                   │
│    Esc          : Return to Normal mode            │
│                                                     │
│  Editor Commands:                                   │
│    e            : Enter Editor mode                │
│    n            : New file                         │
│    o            : Open file                        │
│    s            : Save file                        │
│    r            : Run program                      │
│                                                     │
│  Debugger Commands:                                 │
│    d            : Enter Debugger mode              │
│    n            : Step to next instruction         │
│    c            : Continue execution               │
│    b            : Toggle breakpoint                │
│    r            : Restart debugging                │
│                                                     │
│  TauFoldZKVM Instructions:                          │
│    Arithmetic   : ADD, SUB, MUL, DIV, MOD          │
│    Bitwise     : AND, OR, XOR, NOT, SHL, SHR      │
│    Comparison  : EQ, NEQ, LT, GT, LTE, GTE        │
│    Memory      : LOAD, STORE, PUSH, POP           │
│    Control     : JMP, JZ, JNZ, CALL, RET          │
│    Crypto      : HASH, VERIFY, SIGN               │
│    System      : HALT, NOP, DEBUG, ASSERT, LOG    │
│                                                     │
╰─────────────────────────────────────────────────────╯"#;

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    let help = Paragraph::new(help_text)
        .block(block)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);

    // Center the help overlay
    let help_area = centered_rect(60, 80, area);
    f.render_widget(Clear, help_area);
    f.render_widget(help, help_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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