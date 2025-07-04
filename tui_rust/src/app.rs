use anyhow::Result;
use crossterm::event::KeyEvent;

#[derive(Debug, Clone)]
pub enum DemoApp {
    Calculator,
    CryptoDemo,
    PacmanGame,
    SmartContract,
    VendingMachine,
}

impl DemoApp {
    pub fn name(&self) -> &str {
        match self {
            DemoApp::Calculator => "Calculator",
            DemoApp::CryptoDemo => "Crypto Demo",
            DemoApp::PacmanGame => "Pacman Game",
            DemoApp::SmartContract => "Smart Contract",
            DemoApp::VendingMachine => "Vending Machine",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            DemoApp::Calculator => "Basic calculator with arithmetic operations",
            DemoApp::CryptoDemo => "Cryptographic operations demonstration",
            DemoApp::PacmanGame => "Classic Pacman game implementation",
            DemoApp::SmartContract => "Smart contract execution example",
            DemoApp::VendingMachine => "Vending machine state machine demo",
        }
    }

    pub fn zkvm_path(&self) -> &str {
        match self {
            DemoApp::Calculator => "../apps/calculator.zkvm",
            DemoApp::CryptoDemo => "../apps/crypto_demo.zkvm",
            DemoApp::PacmanGame => "../apps/pacman_game.zkvm",
            DemoApp::SmartContract => "../apps/smart_contract.zkvm",
            DemoApp::VendingMachine => "../apps/vending_machine.zkvm",
        }
    }
}

#[derive(Debug)]
pub enum AppState {
    MainMenu,
    RunningApp(DemoApp),
    Help,
}

pub struct App {
    pub state: AppState,
    pub available_apps: Vec<DemoApp>,
    pub selected_index: usize,
    pub zkvm_output: Vec<String>,
    pub execution_stats: ExecutionStats,
}

#[derive(Debug, Default)]
pub struct ExecutionStats {
    pub cycles: u64,
    pub constraints: u64,
    pub folding_steps: u64,
    pub proof_size: usize,
    pub verification_time_ms: u64,
}

impl App {
    pub fn new() -> Self {
        let available_apps = vec![
            DemoApp::Calculator,
            DemoApp::CryptoDemo,
            DemoApp::PacmanGame,
            DemoApp::SmartContract,
            DemoApp::VendingMachine,
        ];

        Self {
            state: AppState::MainMenu,
            available_apps,
            selected_index: 0,
            zkvm_output: Vec::new(),
            execution_stats: ExecutionStats::default(),
        }
    }

    pub fn next_app(&mut self) {
        if self.selected_index < self.available_apps.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn previous_app(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn select_current_app(&mut self) {
        let app = self.available_apps[self.selected_index].clone();
        self.state = AppState::RunningApp(app);
        self.zkvm_output.clear();
        self.execution_stats = ExecutionStats::default();
    }

    pub fn return_to_menu(&mut self) {
        self.state = AppState::MainMenu;
    }

    pub fn handle_app_input(&mut self, _key: KeyEvent) {
        // This will be extended per app
        match &self.state {
            AppState::RunningApp(app) => {
                match app {
                    DemoApp::Calculator => {
                        // Handle calculator input
                    }
                    DemoApp::PacmanGame => {
                        // Handle game input
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub async fn update(&mut self) -> Result<()> {
        // Update app state, run zkVM if needed
        match &self.state {
            AppState::RunningApp(_app) => {
                // Simulate zkVM execution
                // In real implementation, this would call the actual zkVM
            }
            _ => {}
        }
        Ok(())
    }
}