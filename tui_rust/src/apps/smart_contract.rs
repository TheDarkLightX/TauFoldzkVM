use std::collections::HashMap;
use chrono::{DateTime, Utc, Local};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: u64,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub timestamp: DateTime<Utc>,
    pub tx_type: TransactionType,
    pub status: TransactionStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    Transfer,
    Mint,
    Burn,
    Deploy,
    Call,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct Account {
    pub address: String,
    pub balance: u64,
    pub nonce: u64,
}

#[derive(Debug, Clone)]
pub enum ContractMethod {
    Transfer { to: String, amount: u64 },
    Mint { to: String, amount: u64 },
    Burn { amount: u64 },
    Approve { spender: String, amount: u64 },
    GetBalance { address: String },
}

#[derive(Debug, Clone)]
pub struct SmartContract {
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
    pub owner: String,
    pub accounts: HashMap<String, Account>,
    pub transactions: Vec<Transaction>,
    pub allowances: HashMap<(String, String), u64>, // (owner, spender) -> amount
    pub next_tx_id: u64,
    pub messages: Vec<String>,
    pub paused: bool,
}

impl SmartContract {
    pub fn new() -> Self {
        let owner = "0xABCD1234".to_string();
        let mut accounts = HashMap::new();
        
        // Initialize owner account with initial supply
        accounts.insert(owner.clone(), Account {
            address: owner.clone(),
            balance: 1_000_000,
            nonce: 0,
        });
        
        // Add some demo accounts
        accounts.insert("0xDEF5678".to_string(), Account {
            address: "0xDEF5678".to_string(),
            balance: 0,
            nonce: 0,
        });
        
        accounts.insert("0x9876543".to_string(), Account {
            address: "0x9876543".to_string(),
            balance: 0,
            nonce: 0,
        });
        
        Self {
            name: "TauToken".to_string(),
            symbol: "TAU".to_string(),
            total_supply: 1_000_000,
            owner,
            accounts,
            transactions: vec![],
            allowances: HashMap::new(),
            next_tx_id: 1,
            messages: vec!["Smart Contract deployed successfully".to_string()],
            paused: false,
        }
    }
    
    pub fn execute_method(&mut self, from: &str, method: ContractMethod) -> Result<(), String> {
        if self.paused && !matches!(method, ContractMethod::GetBalance { .. }) {
            return Err("Contract is paused".to_string());
        }
        
        match method {
            ContractMethod::Transfer { to, amount } => {
                self.transfer(from, &to, amount)
            }
            ContractMethod::Mint { to, amount } => {
                self.mint(from, &to, amount)
            }
            ContractMethod::Burn { amount } => {
                self.burn(from, amount)
            }
            ContractMethod::Approve { spender, amount } => {
                self.approve(from, &spender, amount)
            }
            ContractMethod::GetBalance { address } => {
                let balance = self.get_balance(&address);
                self.add_message(&format!("Balance of {}: {} TAU", address, balance));
                Ok(())
            }
        }
    }
    
    fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), String> {
        // Check balances
        let from_balance = self.get_balance(from);
        if from_balance < amount {
            return Err(format!("Insufficient balance: {} < {}", from_balance, amount));
        }
        
        // Update balances
        self.accounts.get_mut(from).unwrap().balance -= amount;
        
        if let Some(to_account) = self.accounts.get_mut(to) {
            to_account.balance += amount;
        } else {
            self.accounts.insert(to.to_string(), Account {
                address: to.to_string(),
                balance: amount,
                nonce: 0,
            });
        }
        
        // Record transaction
        let tx = Transaction {
            id: self.next_tx_id,
            from: from.to_string(),
            to: to.to_string(),
            amount,
            timestamp: Utc::now(),
            tx_type: TransactionType::Transfer,
            status: TransactionStatus::Confirmed,
        };
        
        self.transactions.push(tx);
        self.next_tx_id += 1;
        
        self.add_message(&format!("Transfer: {} TAU from {} to {}", amount, from, to));
        Ok(())
    }
    
    fn mint(&mut self, from: &str, to: &str, amount: u64) -> Result<(), String> {
        // Only owner can mint
        if from != self.owner {
            return Err("Only owner can mint tokens".to_string());
        }
        
        // Update balance and total supply
        if let Some(to_account) = self.accounts.get_mut(to) {
            to_account.balance += amount;
        } else {
            self.accounts.insert(to.to_string(), Account {
                address: to.to_string(),
                balance: amount,
                nonce: 0,
            });
        }
        
        self.total_supply += amount;
        
        // Record transaction
        let tx = Transaction {
            id: self.next_tx_id,
            from: from.to_string(),
            to: to.to_string(),
            amount,
            timestamp: Utc::now(),
            tx_type: TransactionType::Mint,
            status: TransactionStatus::Confirmed,
        };
        
        self.transactions.push(tx);
        self.next_tx_id += 1;
        
        self.add_message(&format!("Minted: {} TAU to {}", amount, to));
        Ok(())
    }
    
    fn burn(&mut self, from: &str, amount: u64) -> Result<(), String> {
        let balance = self.get_balance(from);
        if balance < amount {
            return Err(format!("Insufficient balance to burn: {} < {}", balance, amount));
        }
        
        // Update balance and total supply
        self.accounts.get_mut(from).unwrap().balance -= amount;
        self.total_supply -= amount;
        
        // Record transaction
        let tx = Transaction {
            id: self.next_tx_id,
            from: from.to_string(),
            to: "0x0".to_string(),
            amount,
            timestamp: Utc::now(),
            tx_type: TransactionType::Burn,
            status: TransactionStatus::Confirmed,
        };
        
        self.transactions.push(tx);
        self.next_tx_id += 1;
        
        self.add_message(&format!("Burned: {} TAU from {}", amount, from));
        Ok(())
    }
    
    fn approve(&mut self, owner: &str, spender: &str, amount: u64) -> Result<(), String> {
        self.allowances.insert((owner.to_string(), spender.to_string()), amount);
        self.add_message(&format!("Approved: {} can spend {} TAU from {}", spender, amount, owner));
        Ok(())
    }
    
    pub fn get_balance(&self, address: &str) -> u64 {
        self.accounts.get(address).map(|a| a.balance).unwrap_or(0)
    }
    
    pub fn get_allowance(&self, owner: &str, spender: &str) -> u64 {
        self.allowances.get(&(owner.to_string(), spender.to_string())).copied().unwrap_or(0)
    }
    
    pub fn pause(&mut self) {
        if !self.paused {
            self.paused = true;
            self.add_message("Contract paused");
        }
    }
    
    pub fn unpause(&mut self) {
        if self.paused {
            self.paused = false;
            self.add_message("Contract unpaused");
        }
    }
    
    pub fn get_recent_transactions(&self, count: usize) -> Vec<&Transaction> {
        let len = self.transactions.len();
        if len > count {
            self.transactions[len - count..].iter().collect()
        } else {
            self.transactions.iter().collect()
        }
    }
    
    fn add_message(&mut self, msg: &str) {
        self.messages.push(format!("[{}] {}", Local::now().format("%H:%M:%S"), msg));
        if self.messages.len() > 8 {
            self.messages.remove(0);
        }
    }
    
    pub fn prepare_zkvm_input(&self) -> Vec<u32> {
        vec![
            self.total_supply as u32,
            self.accounts.len() as u32,
            self.transactions.len() as u32,
            if self.paused { 1 } else { 0 },
        ]
    }
    
    pub fn process_zkvm_result(&mut self, result: &[u32]) {
        if !result.is_empty() && result[0] == 1 {
            self.add_message("✓ Contract state verified by zkVM");
        }
    }
}