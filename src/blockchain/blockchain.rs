// blockchain/blockchain.rs
use super::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub max_supply: u64,
    pub current_supply: u64,
}

impl Blockchain {
    // Crée une nouvelle blockchain avec un bloc génésis
    pub fn new(max_supply: u64) -> Self {
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
            max_supply,
            current_supply: 0,
        }
    }

    // Retourne le dernier bloc de la chaîne
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    // Ajoute un bloc à la blockchain
    pub fn add_block(&mut self, mut new_block: Block, reward: u64) {
        let latest_block = self.get_latest_block();
        new_block.previous_hash = latest_block.hash.clone();

        // Vérifie si la récompense dépasse max_supply
        if self.current_supply + reward <= self.max_supply {
            self.current_supply += reward; // Met à jour l'offre en circulation

            // Recalcule le hash pour prendre en compte `previous_hash`
            new_block.hash = new_block.calculate_hash();

            self.chain.push(new_block);
        } else {
            println!("Impossible d'ajouter un bloc : la reward dépasse max_supply !");
        }
    }
}
