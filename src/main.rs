use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    // Crée une nouvelle blockchain avec un "bloc génésis"
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
        new_block.hash = calculate_hash(
            new_block.index, 
            new_block.timestamp, 
            &new_block.transactions, 
            &new_block.previous_hash,
            new_block.nonce,
        );
        self.chain.push(new_block);
    }
}

// Calcule le hash du bloc en combinant ses données
pub fn calculate_hash(
    index: u64,
    timestamp: u128,
    transactions: &[Transaction],
    previous_hash: &str,
    nonce: u64,
) -> String {
    let mut hasher = Sha256::new();

    // On concatène tout pour avoir un input unique
    hasher.update(index.to_string());
    hasher.update(timestamp.to_string());
    for tx in transactions {
        hasher.update(&tx.from);
        hasher.update(&tx.to);
        hasher.update(tx.amount.to_string());
    }
    hasher.update(previous_hash);
    hasher.update(nonce.to_string());

    // On convertit le résultat en une chaîne hexadécimale
    format!("{:x}", hasher.finalize())
}

fn main() {
    // Initialiser la blockchain
    let mut my_blockchain = Blockchain::new();

    // Exemple : création de transactions
    let tx1 = Transaction {
        from: String::from("Alice"),
        to: String::from("Bob"),
        amount: 50,
    };

    let tx2 = Transaction {
        from: String::from("Bob"),
        to: String::from("Charlie"),
        amount: 30,
    };

    // Création d'un bloc
    let new_block = Block {
        index: 1,
        timestamp: 1234567890,
        transactions: vec![tx1, tx2],
        previous_hash: String::new(), // sera rempli par add_block
        hash: String::new(),         // sera calculé par add_block
        nonce: 0,
    };

    // Ajout du bloc dans la blockchain
    my_blockchain.add_block(new_block);

    // Affichage de la blockchain
    println!("État de la blockchain :\n{:#?}", my_blockchain);
}
