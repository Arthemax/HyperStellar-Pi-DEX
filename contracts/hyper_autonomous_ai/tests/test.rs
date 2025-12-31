#[test]
fn test_make_decision() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HyperAutonomousAiContract);
    let client = HyperAutonomousAiContractClient::new(&env, &contract_id);
    
    client.init(&admin);
    let input = Map::new(&env);
    input.set(Symbol::new(&env, "volatility"), 10);
    let decision = client.make_decision(&Symbol::new(&env, "rejection_agent"), &input);
    assert_eq!(decision.action, Symbol::new(&env, "reject"));
}
