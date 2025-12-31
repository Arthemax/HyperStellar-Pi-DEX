#[test]
fn test_process_transaction() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TransactionContract);
    let client = TransactionContractClient::new(&env, &contract_id);
    
    client.init(&admin, &pi_coin_id, &verification_id);
    let tx = client.process_transaction(&sender, &receiver, &100, &Symbol::new(&env, "p2p"));
    assert_eq!(tx.status, Symbol::new(&env, "completed"));
}
