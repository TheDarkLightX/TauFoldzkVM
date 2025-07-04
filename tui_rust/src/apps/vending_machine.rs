
#[derive(Debug, Clone, PartialEq)]
pub enum VendingState {
    Idle,
    ItemSelected(usize),
    AcceptingPayment(usize, u32), // (item_index, amount_needed)
    Dispensing(usize),
    ReturningChange(u32),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct VendingItem {
    pub name: String,
    pub price: u32,
    pub quantity: u32,
    pub code: String,
}

impl VendingItem {
    pub fn new(name: &str, price: u32, quantity: u32, code: &str) -> Self {
        Self {
            name: name.to_string(),
            price,
            quantity,
            code: code.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum VendingAction {
    SelectItem(usize),
    InsertCoin(u32),
    InsertBill(u32),
    Cancel,
    CollectItem,
    CollectChange,
}

#[derive(Debug, Clone)]
pub struct VendingMachine {
    pub current_state: VendingState,
    pub inventory: Vec<VendingItem>,
    pub balance: u32,
    pub total_sales: u32,
    pub messages: Vec<String>,
}

impl VendingState {
    pub fn is_item_selected(&self, index: usize) -> bool {
        match self {
            VendingState::ItemSelected(i) | VendingState::AcceptingPayment(i, _) => *i == index,
            _ => false,
        }
    }
}

impl VendingMachine {
    pub fn new() -> Self {
        let inventory = vec![
            VendingItem::new("Cola", 150, 10, "A1"),
            VendingItem::new("Chips", 100, 8, "A2"),
            VendingItem::new("Candy Bar", 75, 15, "A3"),
            VendingItem::new("Water", 125, 12, "B1"),
            VendingItem::new("Energy Drink", 200, 5, "B2"),
            VendingItem::new("Cookies", 90, 10, "B3"),
            VendingItem::new("Sandwich", 350, 4, "C1"),
            VendingItem::new("Fruit Cup", 175, 6, "C2"),
        ];
        
        Self {
            current_state: VendingState::Idle,
            inventory,
            balance: 0,
            total_sales: 0,
            messages: vec!["Welcome! Select an item.".to_string()],
        }
    }
    
    pub fn process_action(&mut self, action: VendingAction) {
        match action {
            VendingAction::SelectItem(index) => self.select_item_by_index(index),
            VendingAction::InsertCoin(cents) => self.insert_money(cents),
            VendingAction::InsertBill(cents) => self.insert_money(cents),
            VendingAction::Cancel => self.cancel_transaction(),
            VendingAction::CollectItem => self.collect_item(),
            VendingAction::CollectChange => self.collect_change(),
        }
    }
    
    fn select_item_by_index(&mut self, index: usize) {
        if index >= self.inventory.len() {
            self.current_state = VendingState::Error("Invalid selection".to_string());
            self.add_message("Invalid selection");
            return;
        }
        
        let (item_name, item_price, item_quantity) = {
            let item = &self.inventory[index];
            (item.name.clone(), item.price, item.quantity)
        };
        
        if item_quantity == 0 {
            self.current_state = VendingState::Error("Out of stock".to_string());
            self.add_message(&format!("{} is out of stock", item_name));
            return;
        }
        
        self.current_state = VendingState::ItemSelected(index);
        self.add_message(&format!("Selected: {} - ${:.2}", item_name, item_price as f32 / 100.0));
        
        if self.balance >= item_price {
            self.dispense_item(index);
        } else {
            let needed = item_price - self.balance;
            self.current_state = VendingState::AcceptingPayment(index, needed);
            self.add_message(&format!("Please insert ${:.2}", needed as f32 / 100.0));
        }
    }
    
    fn insert_money(&mut self, cents: u32) {
        self.balance += cents;
        self.add_message(&format!("Inserted: ${:.2} (Total: ${:.2})", 
            cents as f32 / 100.0, 
            self.balance as f32 / 100.0));
        
        match &self.current_state {
            VendingState::AcceptingPayment(index, _) => {
                let item = &self.inventory[*index];
                if self.balance >= item.price {
                    self.dispense_item(*index);
                } else {
                    let needed = item.price - self.balance;
                    self.current_state = VendingState::AcceptingPayment(*index, needed);
                    self.add_message(&format!("Still need: ${:.2}", needed as f32 / 100.0));
                }
            }
            VendingState::ItemSelected(index) => {
                let item = &self.inventory[*index];
                if self.balance >= item.price {
                    self.dispense_item(*index);
                }
            }
            _ => {}
        }
    }
    
    fn cancel_transaction(&mut self) {
        if self.balance > 0 {
            self.current_state = VendingState::ReturningChange(self.balance);
            self.add_message(&format!("Returning ${:.2}", self.balance as f32 / 100.0));
            self.balance = 0;
        } else {
            self.current_state = VendingState::Idle;
            self.add_message("Transaction cancelled");
        }
    }
    
    fn collect_item(&mut self) {
        if let VendingState::Dispensing(_) = self.current_state {
            self.current_state = VendingState::Idle;
            self.add_message("Thank you! Enjoy your purchase.");
        }
    }
    
    fn collect_change(&mut self) {
        if let VendingState::ReturningChange(_) = self.current_state {
            self.current_state = VendingState::Idle;
            self.add_message("Change collected");
        }
    }
    
    fn dispense_item(&mut self, index: usize) {
        let (item_name, item_price) = {
            let item = &mut self.inventory[index];
            item.quantity -= 1;
            (item.name.clone(), item.price)
        };
        
        let change = self.balance - item_price;
        self.total_sales += item_price;
        self.balance = 0;
        
        self.current_state = VendingState::Dispensing(index);
        self.add_message(&format!("Dispensing {}...", item_name));
        
        if change > 0 {
            self.add_message(&format!("Change: ${:.2}", change as f32 / 100.0));
            // In a real implementation, we'd handle change return separately
        }
    }
    
    fn add_message(&mut self, msg: &str) {
        self.messages.push(msg.to_string());
        if self.messages.len() > 5 {
            self.messages.remove(0);
        }
    }
    
    pub fn get_display(&self) -> String {
        match &self.current_state {
            VendingState::Idle => "Ready".to_string(),
            VendingState::ItemSelected(idx) => {
                format!("Selected: {}", self.inventory[*idx].name)
            }
            VendingState::AcceptingPayment(idx, amount) => {
                format!("{} - Need: ${:.2}", self.inventory[*idx].name, *amount as f32 / 100.0)
            }
            VendingState::Dispensing(idx) => {
                format!("Dispensing: {}", self.inventory[*idx].name)
            }
            VendingState::ReturningChange(amount) => {
                format!("Change: ${:.2}", *amount as f32 / 100.0)
            }
            VendingState::Error(msg) => msg.clone(),
        }
    }
    
    pub fn prepare_zkvm_input(&self) -> Vec<u32> {
        match &self.current_state {
            VendingState::Idle => vec![0],
            VendingState::ItemSelected(idx) => vec![1, *idx as u32],
            VendingState::AcceptingPayment(idx, amount) => vec![2, *idx as u32, *amount],
            VendingState::Dispensing(idx) => vec![3, *idx as u32],
            VendingState::ReturningChange(amount) => vec![4, *amount],
            VendingState::Error(_) => vec![5],
        }
    }
    
    pub fn process_zkvm_result(&mut self, result: &[u32]) {
        if !result.is_empty() && result[0] == 1 {
            // Transaction verified by zkVM
            self.add_message("Transaction verified ✓");
        }
    }
}