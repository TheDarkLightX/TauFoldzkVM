mod app;
mod ui;
mod apps;
mod zkvm;
mod welcome;
mod help;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::{io, time::Duration};

use crate::app::{App, AppState};
use crate::welcome::{should_show_welcome, mark_welcomed};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let mut app = App::new();
    
    // Show welcome screen on first run
    if should_show_welcome() {
        app.state = AppState::Welcome;
    }
    
    let res = run_app(&mut terminal, app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match app.state {
                    AppState::Welcome => match key.code {
                        KeyCode::Enter => {
                            mark_welcomed();
                            app.state = AppState::MainMenu;
                        }
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        _ => {}
                    },
                    AppState::MainMenu => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Enter => app.select_current_app(),
                        KeyCode::Up => app.previous_app(),
                        KeyCode::Down => app.next_app(),
                        KeyCode::Char('?') | KeyCode::F(1) => app.state = AppState::Help,
                        _ => {}
                    },
                    AppState::RunningApp(_) => match key.code {
                        KeyCode::Esc => app.return_to_menu(),
                        KeyCode::Char('?') | KeyCode::F(1) => app.state = AppState::Help,
                        _ => app.handle_app_input(key),
                    },
                    AppState::Help => match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => app.return_to_menu(),
                        _ => {}
                    },
                }
            }
        }

        // Update app state
        app.update().await?;
    }
}
