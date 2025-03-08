mod consensus;
mod network; 
mod blockchain; 

use crate::blockchain::block::Block;
use crate::blockchain::blockchain::Blockchain;
use crate::blockchain::transaction::Transaction;

fn main() {
    // Initialiser la blockchain
    let mut my_blockchain = Blockchain::new();

    // Exemple de transactions
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
    let new_block = Block::new(1, 1234567890, vec![tx1, tx2], String::new(), 0);

    // Ajout du bloc dans la blockchain
    my_blockchain.add_block(new_block);

    // Exemple d'utilisation des autres modules
    consensus::pos::start_pos(); // Proof of Stake
    consensus::quantum_security::apply_quantum_security();
    consensus::validator::register_validator("Validator1");

    network::p2p::start_p2p(); // Réseau P2P
    network::node::create_node("Node1");

    // Utilisation de la fonction build_merkle_tree
    use crate::blockchain::merkle_tree::build_merkle_tree;
    build_merkle_tree(&[]);

    // Affichage de la blockchain
    println!("État de la blockchain :\n{:#?}", my_blockchain);
}
