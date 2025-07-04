use anyhow::Result;
use crossterm::event::{KeyEvent, KeyCode};
use crate::zkvm::ZkVMRunner;
use crate::apps::{
    calculator::Calculator,
    crypto_demo::{CryptoDemo, CryptoMode},
    pacman::{PacmanGame, Direction},
    smart_contract::{SmartContract, ContractMethod},
    vending_machine::{VendingMachine, VendingAction},
};
use std::sync::Arc;
use tokio::sync::Mutex;

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
    pub zkvm_runner: Option<Arc<Mutex<ZkVMRunner>>>,
    pub is_executing: bool,
    pub app_state: AppSpecificState,
}

#[derive(Debug, Default, Clone)]
pub struct ExecutionStats {
    pub cycles: u64,
    pub constraints: u64,
    pub folding_steps: u64,
    pub proof_size: usize,
    pub verification_time_ms: u64,
}

#[derive(Clone)]
pub enum AppSpecificState {
    Calculator(Calculator),
    CryptoDemo(CryptoDemo),
    PacmanGame(PacmanGame),
    SmartContract(SmartContract),
    VendingMachine(VendingMachine),
    None,
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
            zkvm_runner: None,
            is_executing: false,
            app_state: AppSpecificState::None,
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
        
        // Initialize app-specific state
        self.app_state = match &app {
            DemoApp::Calculator => AppSpecificState::Calculator(Calculator::new()),
            DemoApp::CryptoDemo => AppSpecificState::CryptoDemo(CryptoDemo::new()),
            DemoApp::PacmanGame => AppSpecificState::PacmanGame(PacmanGame::new()),
            DemoApp::SmartContract => AppSpecificState::SmartContract(SmartContract::new()),
            DemoApp::VendingMachine => AppSpecificState::VendingMachine(VendingMachine::new()),
        };
        
        // Create zkVM runner for the app
        let runner = ZkVMRunner::new(app.zkvm_path());
        self.zkvm_runner = Some(Arc::new(Mutex::new(runner)));
        
        self.state = AppState::RunningApp(app);
        self.zkvm_output.clear();
        self.execution_stats = ExecutionStats::default();
    }
    

    pub fn return_to_menu(&mut self) {
        self.state = AppState::MainMenu;
    }

    pub fn handle_app_input(&mut self, key: KeyEvent) {
        match (&self.state, &mut self.app_state) {
            (AppState::RunningApp(DemoApp::Calculator), AppSpecificState::Calculator(calc)) => {
                match key.code {
                    KeyCode::Char(c) if c.is_digit(10) => calc.input_digit(c),
                    KeyCode::Char('.') => calc.input_decimal(),
                    KeyCode::Char('+') => calc.set_operation(crate::apps::calculator::Operation::Add),
                    KeyCode::Char('-') => calc.set_operation(crate::apps::calculator::Operation::Subtract),
                    KeyCode::Char('*') => calc.set_operation(crate::apps::calculator::Operation::Multiply),
                    KeyCode::Char('/') => calc.set_operation(crate::apps::calculator::Operation::Divide),
                    KeyCode::Char('=') | KeyCode::Enter => {
                        calc.calculate();
                        self.is_executing = true;
                    }
                    KeyCode::Char('c') | KeyCode::Char('C') => calc.clear(),
                    KeyCode::Char('m') => calc.memory_store(),
                    KeyCode::Char('r') => calc.memory_recall(),
                    _ => {}
                }
            }
            (AppState::RunningApp(DemoApp::CryptoDemo), AppSpecificState::CryptoDemo(crypto)) => {
                match key.code {
                    KeyCode::Char('1') => crypto.set_mode(CryptoMode::Hash),
                    KeyCode::Char('2') => crypto.set_mode(CryptoMode::Sign),
                    KeyCode::Char('3') => crypto.set_mode(CryptoMode::Verify),
                    KeyCode::Char('4') => crypto.set_mode(CryptoMode::Encrypt),
                    KeyCode::Char('5') => crypto.set_mode(CryptoMode::Decrypt),
                    KeyCode::Char(c) if c.is_alphanumeric() || c == ' ' => crypto.add_char(c),
                    KeyCode::Backspace => crypto.remove_char(),
                    KeyCode::Enter => {
                        crypto.process_input();
                        self.is_executing = true;
                    }
                    KeyCode::Delete => crypto.clear_input(),
                    _ => {}
                }
            }
            (AppState::RunningApp(DemoApp::PacmanGame), AppSpecificState::PacmanGame(game)) => {
                match key.code {
                    KeyCode::Up => {
                        game.move_player(Direction::Up);
                        self.is_executing = true;
                    }
                    KeyCode::Down => {
                        game.move_player(Direction::Down);
                        self.is_executing = true;
                    }
                    KeyCode::Left => {
                        game.move_player(Direction::Left);
                        self.is_executing = true;
                    }
                    KeyCode::Right => {
                        game.move_player(Direction::Right);
                        self.is_executing = true;
                    }
                    KeyCode::Char('p') | KeyCode::Char('P') => {
                        // Toggle pause
                        if game.game_state == crate::apps::pacman::GameState::Playing {
                            game.game_state = crate::apps::pacman::GameState::Paused;
                        } else if game.game_state == crate::apps::pacman::GameState::Paused {
                            game.game_state = crate::apps::pacman::GameState::Playing;
                        }
                    }
                    _ => {}
                }
                // Update game state
                game.update();
            }
            (AppState::RunningApp(DemoApp::SmartContract), AppSpecificState::SmartContract(contract)) => {
                match key.code {
                    KeyCode::Char('1') => {
                        // Transfer tokens
                        let _ = contract.execute_method(
                            "0xABCD1234",
                            ContractMethod::Transfer {
                                to: "0xDEF5678".to_string(),
                                amount: 100,
                            },
                        );
                        self.is_executing = true;
                    }
                    KeyCode::Char('2') => {
                        // Mint tokens
                        let _ = contract.execute_method(
                            &contract.owner.clone(),
                            ContractMethod::Mint {
                                to: "0x9876543".to_string(),
                                amount: 500,
                            },
                        );
                        self.is_executing = true;
                    }
                    KeyCode::Char('3') => {
                        // Burn tokens
                        let _ = contract.execute_method(
                            "0xABCD1234",
                            ContractMethod::Burn { amount: 50 },
                        );
                        self.is_executing = true;
                    }
                    KeyCode::Char('p') => contract.pause(),
                    KeyCode::Char('u') => contract.unpause(),
                    _ => {}
                }
            }
            (AppState::RunningApp(DemoApp::VendingMachine), AppSpecificState::VendingMachine(vending)) => {
                match key.code {
                    KeyCode::Char(c) if c.is_digit(10) => {
                        let index = c.to_digit(10).unwrap() as usize;
                        if index > 0 && index <= vending.inventory.len() {
                            vending.process_action(VendingAction::SelectItem(index - 1));
                            self.is_executing = true;
                        }
                    }
                    KeyCode::Char('q') => {
                        vending.process_action(VendingAction::InsertCoin(25));
                        self.is_executing = true;
                    }
                    KeyCode::Char('d') => {
                        vending.process_action(VendingAction::InsertCoin(10));
                        self.is_executing = true;
                    }
                    KeyCode::Char('n') => {
                        vending.process_action(VendingAction::InsertCoin(5));
                        self.is_executing = true;
                    }
                    KeyCode::Char('b') => {
                        vending.process_action(VendingAction::InsertBill(100));
                        self.is_executing = true;
                    }
                    KeyCode::Char('c') => {
                        vending.process_action(VendingAction::Cancel);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub async fn update(&mut self) -> Result<()> {
        if self.is_executing {
            if let Some(runner) = &self.zkvm_runner {
                let runner_lock = runner.clone();
                
                // Prepare input based on current app
                let input = match (&self.state, &self.app_state) {
                    (AppState::RunningApp(DemoApp::Calculator), AppSpecificState::Calculator(calc)) => {
                        calc.prepare_zkvm_input()
                    }
                    (AppState::RunningApp(DemoApp::CryptoDemo), AppSpecificState::CryptoDemo(crypto)) => {
                        crypto.prepare_zkvm_input()
                    }
                    (AppState::RunningApp(DemoApp::PacmanGame), AppSpecificState::PacmanGame(game)) => {
                        game.prepare_zkvm_input()
                    }
                    (AppState::RunningApp(DemoApp::SmartContract), AppSpecificState::SmartContract(contract)) => {
                        contract.prepare_zkvm_input()
                    }
                    (AppState::RunningApp(DemoApp::VendingMachine), AppSpecificState::VendingMachine(vending)) => {
                        vending.prepare_zkvm_input()
                    }
                    _ => vec![],
                };
                
                if !input.is_empty() {
                    match runner_lock.lock().await.execute(input).await {
                        Ok(result) => {
                            self.execution_stats = ExecutionStats {
                                cycles: result.cycles,
                                constraints: result.constraints_generated,
                                folding_steps: result.folding_steps,
                                proof_size: result.proof_size,
                                verification_time_ms: result.verification_time_ms,
                            };
                            
                            // Update app state with result
                            match &mut self.app_state {
                                AppSpecificState::Calculator(calc) => {
                                    calc.process_zkvm_result(&result.output);
                                }
                                AppSpecificState::CryptoDemo(crypto) => {
                                    crypto.process_zkvm_result(&result.output);
                                }
                                AppSpecificState::PacmanGame(game) => {
                                    game.process_zkvm_result(&result.output);
                                }
                                AppSpecificState::SmartContract(contract) => {
                                    contract.process_zkvm_result(&result.output);
                                }
                                AppSpecificState::VendingMachine(vending) => {
                                    vending.process_zkvm_result(&result.output);
                                }
                                _ => {}
                            }
                            
                            // Add trace log to output
                            for log_entry in result.trace_log {
                                self.zkvm_output.push(log_entry);
                            }
                            
                            self.zkvm_output.push(format!("✓ Proof generated in {}ms", result.verification_time_ms));
                        }
                        Err(e) => {
                            self.zkvm_output.push(format!("❌ Error: {}", e));
                        }
                    }
                }
                
                self.is_executing = false;
            }
        }
        Ok(())
    }
}