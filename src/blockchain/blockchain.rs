// blockchain/blockchain.rs
use super::block::{Block};

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    // Crée une nouvelle blockchain avec un bloc génésis
    pub fn new() -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: 0,
            transactions: vec![],
            previous_hash: "0".to_string(),
            hash: "0".to_string(),
            nonce: 0,
        };

        Blockchain {
            chain: vec![genesis_block],
        }
    }

    // Retourne le dernier bloc de la chaîne
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    // Ajoute un bloc à la blockchain
    pub fn add_block(&mut self, mut new_block: Block) {
        let latest_block = self.get_latest_block();
        new_block.previous_hash = latest_block.hash.clone();

        // Recalcule le hash pour prendre en compte `previous_hash`
        new_block.hash = new_block.calculate_hash();

        self.chain.push(new_block);
    }
}
