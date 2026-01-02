#[test]
fn test_log_metric() {
    let env = Env::default();
    let contract_id = env.register_contract(None, MonitoringContract);
    let client = MonitoringContractClient::new(&env, &contract_id);
    
    client.init(&admin);
    client.log_metric(&Symbol::new(&env, "volatility"), &50);
    let status = client.get_health_status();
    assert_eq!(status, Symbol::new(&env, "healthy"));
}
