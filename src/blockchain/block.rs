// blockchain/block.rs
use sha3::{Digest, Sha3_512};

use crate::blockchain::transaction::Transaction;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(
        index: u64,
        timestamp: u128,
        transactions: Vec<Transaction>,
        previous_hash: String,
        nonce: u64,
    ) -> Self {
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(), // sera calculé
            nonce,
        };

        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha3_512::new();

        // Convertir les champs en chaînes et les injecter dans le hasher
        hasher.update(self.index.to_string());
        hasher.update(self.timestamp.to_string());
        for tx in &self.transactions {
            hasher.update(&tx.from);
            hasher.update(&tx.to);
            hasher.update(tx.amount.to_string());
        }
        hasher.update(&self.previous_hash);
        hasher.update(self.nonce.to_string());

        // Retourne le digest en hexadécimal
        format!("{:x}", hasher.finalize())
    }
}
