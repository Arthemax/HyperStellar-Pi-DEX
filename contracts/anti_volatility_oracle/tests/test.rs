#[test]
fn test_check_volatility() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AntiVolatilityOracleContract);
    let client = AntiVolatilityOracleContractClient::new(&env, &contract_id);
    
    client.init(&admin, &5);
    let report = client.check_volatility(&Symbol::new(&env, "bitcoin"));
    assert!(report.is_rejected);  // Bitcoin should be rejected
}
