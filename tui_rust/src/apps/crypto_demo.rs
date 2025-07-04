use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub enum CryptoMode {
    Hash,
    Sign,
    Verify,
    Encrypt,
    Decrypt,
}

#[derive(Debug, Clone)]
pub struct CryptoDemo {
    pub mode: CryptoMode,
    pub input_text: String,
    pub hash_result: Option<String>,
    pub signature: Option<String>,
    pub public_key: String,
    pub private_key: String,
    pub verify_result: Option<bool>,
    pub encrypted_data: Option<String>,
    pub decrypted_data: Option<String>,
    pub messages: Vec<String>,
}

impl CryptoDemo {
    pub fn new() -> Self {
        let (public_key, private_key) = Self::generate_keypair();
        
        Self {
            mode: CryptoMode::Hash,
            input_text: String::new(),
            hash_result: None,
            signature: None,
            public_key,
            private_key,
            verify_result: None,
            encrypted_data: None,
            decrypted_data: None,
            messages: vec!["Crypto Demo - Enter text to process".to_string()],
        }
    }
    
    fn generate_keypair() -> (String, String) {
        // Simplified key generation for demo
        let mut rng = rand::thread_rng();
        let key_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        let public_key = general_purpose::STANDARD.encode(&key_bytes[0..16]);
        let private_key = general_purpose::STANDARD.encode(&key_bytes[16..32]);
        (public_key, private_key)
    }
    
    pub fn set_mode(&mut self, mode: CryptoMode) {
        self.mode = mode;
        self.clear_results();
        
        let mode_str = match mode {
            CryptoMode::Hash => "Hash Mode - SHA256",
            CryptoMode::Sign => "Sign Mode - Digital Signature",
            CryptoMode::Verify => "Verify Mode - Signature Verification",
            CryptoMode::Encrypt => "Encrypt Mode - Symmetric Encryption",
            CryptoMode::Decrypt => "Decrypt Mode - Symmetric Decryption",
        };
        self.add_message(&format!("Switched to {}", mode_str));
    }
    
    pub fn process_input(&mut self) {
        if self.input_text.is_empty() {
            self.add_message("Please enter some text first");
            return;
        }
        
        match self.mode {
            CryptoMode::Hash => self.compute_hash(),
            CryptoMode::Sign => self.sign_message(),
            CryptoMode::Verify => self.verify_signature(),
            CryptoMode::Encrypt => self.encrypt_data(),
            CryptoMode::Decrypt => self.decrypt_data(),
        }
    }
    
    fn compute_hash(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(&self.input_text);
        let result = hasher.finalize();
        let hash_hex = format!("{:x}", result);
        
        self.hash_result = Some(hash_hex.clone());
        self.add_message(&format!("SHA256 Hash computed: {}...", &hash_hex[..16]));
    }
    
    fn sign_message(&mut self) {
        // Simplified signature (in real implementation would use proper crypto)
        let mut hasher = Sha256::new();
        hasher.update(&self.input_text);
        hasher.update(&self.private_key);
        let result = hasher.finalize();
        let signature = general_purpose::STANDARD.encode(result);
        
        self.signature = Some(signature.clone());
        self.add_message(&format!("Message signed: {}...", &signature[..16]));
    }
    
    fn verify_signature(&mut self) {
        if let Some(sig) = &self.signature {
            // Simplified verification
            let mut hasher = Sha256::new();
            hasher.update(&self.input_text);
            hasher.update(&self.private_key);
            let result = hasher.finalize();
            let expected_sig = general_purpose::STANDARD.encode(result);
            
            let is_valid = sig == &expected_sig;
            self.verify_result = Some(is_valid);
            
            if is_valid {
                self.add_message("✓ Signature verified successfully");
            } else {
                self.add_message("✗ Signature verification failed");
            }
        } else {
            self.add_message("No signature to verify");
        }
    }
    
    fn encrypt_data(&mut self) {
        // Simple XOR encryption for demo
        let key_bytes = general_purpose::STANDARD.decode(&self.private_key).unwrap_or_default();
        let encrypted: Vec<u8> = self.input_text.bytes()
            .enumerate()
            .map(|(i, b)| b ^ key_bytes[i % key_bytes.len()])
            .collect();
        
        let encrypted_b64 = general_purpose::STANDARD.encode(encrypted);
        self.encrypted_data = Some(encrypted_b64.clone());
        self.add_message(&format!("Data encrypted: {}...", &encrypted_b64[..16]));
    }
    
    fn decrypt_data(&mut self) {
        if let Some(encrypted_b64) = &self.encrypted_data {
            if let Ok(encrypted) = general_purpose::STANDARD.decode(encrypted_b64) {
                let key_bytes = general_purpose::STANDARD.decode(&self.private_key).unwrap_or_default();
                let decrypted: Vec<u8> = encrypted.iter()
                    .enumerate()
                    .map(|(i, &b)| b ^ key_bytes[i % key_bytes.len()])
                    .collect();
                
                if let Ok(decrypted_str) = String::from_utf8(decrypted) {
                    self.decrypted_data = Some(decrypted_str.clone());
                    self.add_message(&format!("Data decrypted: {}", decrypted_str));
                } else {
                    self.add_message("Decryption failed - invalid data");
                }
            }
        } else {
            self.add_message("No encrypted data to decrypt");
        }
    }
    
    fn clear_results(&mut self) {
        self.hash_result = None;
        self.signature = None;
        self.verify_result = None;
        self.encrypted_data = None;
        self.decrypted_data = None;
    }
    
    fn add_message(&mut self, msg: &str) {
        self.messages.push(msg.to_string());
        if self.messages.len() > 5 {
            self.messages.remove(0);
        }
    }
    
    pub fn add_char(&mut self, c: char) {
        if self.input_text.len() < 100 {
            self.input_text.push(c);
        }
    }
    
    pub fn remove_char(&mut self) {
        self.input_text.pop();
    }
    
    pub fn clear_input(&mut self) {
        self.input_text.clear();
        self.clear_results();
        self.add_message("Input cleared");
    }
    
    pub fn prepare_zkvm_input(&self) -> Vec<u32> {
        let mode_code = match self.mode {
            CryptoMode::Hash => 0,
            CryptoMode::Sign => 1,
            CryptoMode::Verify => 2,
            CryptoMode::Encrypt => 3,
            CryptoMode::Decrypt => 4,
        };
        
        let mut input = vec![mode_code];
        
        // Add first 8 bytes of input as u32s
        for chunk in self.input_text.bytes().take(32).collect::<Vec<u8>>().chunks(4) {
            let mut bytes = [0u8; 4];
            for (i, &b) in chunk.iter().enumerate() {
                bytes[i] = b;
            }
            input.push(u32::from_le_bytes(bytes));
        }
        
        input
    }
    
    pub fn process_zkvm_result(&mut self, result: &[u32]) {
        if !result.is_empty() && result[0] == 1 {
            self.add_message("✓ Operation verified by zkVM");
        }
    }
}