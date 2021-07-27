use log::{info, error};
use solana_client::client_error::Result as ClientResult;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::system_transaction;
use solana_core::test_validator::TestValidator;
use solana_streamer::socket::SocketAddrSpace;

#[test]
fn basic() -> ClientResult<()> {
    solana_logger::setup();

    let alice = Keypair::new();
    let validator = TestValidator::with_no_fees(alice.pubkey(), None, SocketAddrSpace::Unspecified);
    let rpc_client = RpcClient::new(validator.rpc_url());

    let slot = rpc_client.get_slot()?;

    assert!(slot < 10);

    Ok(())
}

#[test]
fn confirm_transaction() -> ClientResult<()> {
    solana_logger::setup();

    let alice = Keypair::new();
    let validator = TestValidator::with_no_fees(alice.pubkey(), None, SocketAddrSpace::Unspecified);
    let rpc_client = RpcClient::new(validator.rpc_url());

    let bob = solana_sdk::pubkey::new_rand();
    let lamports = 50;
    let (recent_blockhash, _) = rpc_client.get_recent_blockhash()?;
    let tx = system_transaction::transfer(&alice, &bob, lamports, recent_blockhash);
    let signature = rpc_client.send_transaction(&tx)?;
    let confirmed = rpc_client.confirm_transaction(&signature)?;
    assert!(confirmed);

    Ok(())
}

#[test]
fn get_signature_statuses() -> ClientResult<()> {
    solana_logger::setup();

    let alice = Keypair::new();
    let validator = TestValidator::with_no_fees(alice.pubkey(), None, SocketAddrSpace::Unspecified);
    let rpc_client = RpcClient::new(validator.rpc_url());

    let bob = solana_sdk::pubkey::new_rand();
    let lamports = 50;
    let (recent_blockhash, _) = rpc_client.get_recent_blockhash()?;
    let tx = system_transaction::transfer(&alice, &bob, lamports, recent_blockhash);
    let signature = rpc_client.send_transaction(&tx)?;

    for _ in 0..100 {
        let statuses = rpc_client.get_signature_statuses(&[signature])?;
        let status = statuses.value[0].as_ref();
        if let Some(status) = status {
            info!("{:?}", status);
        } else {
            std::thread::sleep_ms(100);
        }
    }

    panic!("no signature status");
}
