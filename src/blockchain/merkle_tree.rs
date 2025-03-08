// blockchain/merkle_tree.rs
use super::transaction::Transaction;

pub fn build_merkle_tree(transactions: &[Transaction]) -> String {
    println!("Building Merkle tree with {} transactions...", transactions.len());
    // Pour l’instant, on retourne juste une chaîne vide ou fixe
    // Implémenter l’algorithme de construction de l’arbre
    "MerkleRootPlaceholder".to_string()
}
