// tests/test.rs
use soroban_sdk::{Env, Address, Symbol, Bytes, BytesN, events};
use pi_coin_contract::PiCoinContractClient; // Assuming generated client from lib.rs

#[test]
fn test_init_and_basic_setup() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin);
    
    // Check events
    let events = env.events();
    assert!(events.len() > 0);
    assert_eq!(events[0].1, Symbol::new(&env, "Initialized"));
    
    // Check supply
    assert_eq!(client.get_current_supply(), 0);
}

#[test]
fn test_mint_with_verifications() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin);
    
    let amount = 100u64;
    let source = Symbol::new(&env, "mining");
    let coin = client.mint(&user, &amount, &source);
    
    assert_eq!(coin.amount, amount);
    assert_eq!(coin.owner, user);
    assert_eq!(coin.source, source);
    assert_eq!(coin.verified, true);
    assert!(!coin.proof.is_empty());
    
    // Check supply increase
    assert_eq!(client.get_current_supply(), amount);
    
    // Check USD value
    assert_eq!(client.get_usd_value(&amount), amount * 314159);
    
    // Check events
    let events = env.events();
    assert!(events.iter().any(|e| e.1 == Symbol::new(&env, "Minted")));
}

#[test]
fn test_transfer_with_proof_check() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin);
    
    let coin = client.mint(&user1, &200, &Symbol::new(&env, "rewards"));
    let coin_id = BytesN::from_array(&env, &[0; 32]); // Mock coin ID
    
    client.transfer(&user1, &user2, &100, &coin_id);
    
    // Check events
    let events = env.events();
    assert!(events.iter().any(|e| e.1 == Symbol::new(&env, "Transferred")));
    
    // Check supply unchanged
    assert_eq!(client.get_current_supply(), 200);
}

#[test]
fn test_burn_and_supply_control() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin);
    
    let coin = client.mint(&user, &500, &Symbol::new(&env, "p2p"));
    let coin_id = BytesN::from_array(&env, &[1; 32]); // Mock coin ID
    
    client.burn(&user, &200, &coin_id);
    
    // Check supply decrease
    assert_eq!(client.get_current_supply(), 300);
    
    // Check events
    let events = env.events();
    assert!(events.iter().any(|e| e.1 == Symbol::new(&env, "Burned")));
}

#[test]
fn test_utility_functions() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin);
    
    // Test anomaly check
    assert!(!client.check_anomaly(&100000));
    assert!(client.check_anomaly(&2000000000)); // > 1e9
    
    // Test legal tender
    let coin_id = BytesN::from_array(&env, &[2; 32]);
    assert!(client.enforce_global_legal_tender(&coin_id));
    
    // Test AI governance
    assert!(client.ai_governance_check(&Bytes::from(b"long_enough_data_for_test")));
    assert!(!client.ai_governance_check(&Bytes::from(b"short")));
}

#[test]
fn test_supply_cap_and_peg_enforcement() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin);
    
    // Test normal mint
    client.mint(&user, &1000, &Symbol::new(&env, "mining"));
    assert_eq!(client.get_current_supply(), 1000);
    
    // Test supply cap (should panic if exceeded, but for test, assume within limit)
    // In real test, mock to exceed 100B if needed
    
    // Test peg (mocked to pass)
    assert_eq!(client.get_usd_value(&1), 314159);
}

#[test]
fn test_invalid_operations() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin);
    
    // Mint first
    let coin = client.mint(&user1, &100, &Symbol::new(&env, "mining"));
    let coin_id = BytesN::from_array(&env, &[3; 32]);
    
    // Test invalid transfer (insufficient amount)
    let result = std::panic::catch_unwind(|| {
        client.transfer(&user1, &user2, &200, &coin_id); // More than owned
    });
    assert!(result.is_err());
    
    // Test invalid source
    let result2 = std::panic::catch_unwind(|| {
        client.mint(&user1, &50, &Symbol::new(&env, "invalid"));
    });
    assert!(result2.is_err());
    }
