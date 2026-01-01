#[test]
fn test_register_entity() {
    let env = Env::default();
    let contract_id = env.register_contract(None, InterplanetaryEconomyContract);
    let client = InterplanetaryEconomyContractClient::new(&env, &contract_id);
    
    client.init(&admin);
    client.register_entity(&Symbol::new(&env, "Earth"), &1000000);
    let entities = client.get_entities();
    assert!(entities.contains_key(Symbol::new(&env, "Earth")));
}
