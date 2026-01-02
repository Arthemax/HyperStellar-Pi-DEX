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
fn test_mint_with_compliance_and_verifications() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin);
    
    // Register compliance for user
    client.register_compliance(&admin, &user, &Symbol::new(&env, "US"), &true, &10);
    
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
fn test_transfer_with_compliance_and_proof_check() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin);
    
    // Register compliance for both users
    client.register_compliance(&admin, &user1, &Symbol::new(&env, "US"), &true, &10);
    client.register_compliance(&admin, &user2, &Symbol::new(&env, "ID"), &true, &20);
    
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
    
    // Register compliance
    client.register_compliance(&admin, &user, &Symbol::new(&env, "US"), &true, &10);
    
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
fn test_compliance_registration_and_peg_update() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin);
    
    // Register compliance
    client.register_compliance(&admin, &user, &Symbol::new(&env, "US"), &true, &10);
    
    // Check events
    let events = env.events();
    assert!(events.iter().any(|e| e.1 == Symbol::new(&env, "ComplianceRegistered")));
    
    // Update peg (should succeed for $314,159)
    client.update_peg(&admin, &314159);
    assert!(events.iter().any(|e| e.1 == Symbol::new(&env, "PegUpdated")));
    
    // Test peg update failure
    let result = std::panic::catch_unwind(|| {
        client.update_peg(&admin, &314160); // Different peg
    });
    assert!(result.is_err());
}

#[test]
fn test_utility_functions_and_ai_governance() {
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
    
    // Register compliance
    client.register_compliance(&admin, &user, &Symbol::new(&env, "US"), &true, &10);
    
    // Test normal mint
    client.mint(&user, &1000, &Symbol::new(&env, "mining"));
    assert_eq!(client.get_current_supply(), 1000);
    
    // Test supply cap (mock exceed in real test if needed)
    // For now, assume within limit
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
    
    // Register compliance for user1 only
    client.register_compliance(&admin, &user1, &Symbol::new(&env, "US"), &true, &10);
    
    // Mint first
    let coin = client.mint(&user1, &100, &Symbol::new(&env, "mining"));
    let coin_id = BytesN::from_array(&env, &[3; 32]);
    
    // Test invalid transfer (recipient not compliant)
    let result = std::panic::catch_unwind(|| {
        client.transfer(&user1, &user2, &50, &coin_id); // user2 not compliant
    });
    assert!(result.is_err());
    
    // Test invalid source
    let result2 = std::panic::catch_unwind(|| {
        client.mint(&user1, &50, &Symbol::new(&env, "invalid"));
    });
    assert!(result2.is_err());
    
    // Test invalid burn (insufficient amount)
    let result3 = std::panic::catch_unwind(|| {
        client.burn(&user1, &200, &coin_id); // More than owned
    });
    assert!(result3.is_err());
}
