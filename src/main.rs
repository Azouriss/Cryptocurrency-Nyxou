// src/main.rs
mod blockchain;
mod consensus;
mod network;

use crate::blockchain::block::Block;
use crate::blockchain::blockchain::Blockchain;
use crate::blockchain::transaction::Transaction;

use pqcrypto_dilithium::dilithium2;

fn main() {
    // Initialiser la blockchain
    let mut my_blockchain = Blockchain::new(19_500_000);

    // Génération d'une paire de clés PQ
    let (public_key, secret_key) = dilithium2::keypair();

    // Exemple de transaction signée
    let mut tx1 = Transaction {
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        amount: 50,
        public_key: None,
        signature: None,
    };

    // On signe cette transaction
    tx1.sign_transaction(&secret_key, &public_key);

    // Vérification (optionnelle ici)
    if tx1.verify_transaction() {
        println!("Transaction 1 signée et valide !");
    } else {
        println!("Transaction 1 invalide !");
    }

    let tx2 = Transaction {
        from: "Bob".to_string(),
        to: "Charlie".to_string(),
        amount: 30,
        public_key: None,
        signature: None,
    };

    // Récupère le hash du bloc précédent
    let prev_hash = my_blockchain.chain.last().unwrap().hash.clone();

    // On crée le nouveau bloc
    let new_block = Block::new(
        my_blockchain.chain.len() as u64,
        1234567890,
        vec![tx1, tx2],
        prev_hash,
        0,
    );

    my_blockchain.add_block(new_block, 50);

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
