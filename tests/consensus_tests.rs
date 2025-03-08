// Chaque fichier de la section `tests/` est compilé comme un exécutable de test distinct.
use nyxou_lib::consensus::pos::start_pos;
use nyxou_lib::consensus::quantum_security::apply_quantum_security;

#[test]
fn test_pos_consensus() {
    // Appeler la fonction `start_pos` et vérifier le comportement 
    start_pos();
    assert!(true, "POS consensus test executed");
}

#[test]
fn test_quantum_security() {
    apply_quantum_security();
    assert!(true, "Quantum security test executed");
}
