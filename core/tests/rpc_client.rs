use log::trace;
use solana_client::client_error::Result as ClientResult;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::system_transaction;
use solana_core::test_validator::TestValidator;
use solana_streamer::socket::SocketAddrSpace;
use solana_transaction_status::TransactionConfirmationStatus;
use std::time::Duration;

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

    let mut state = None;

    // Test that the status of the transaction progresses
    // from none, to processed, to confirmations, to finalized.
    for _ in 0..1000 {
        let statuses = rpc_client.get_signature_statuses(&[signature])?;
        let status = statuses.value[0].as_ref();
        if let Some(status) = status {
            let cstatus = status.confirmation_status.clone().unwrap();
            match (state, cstatus) {
                (None | Some(TransactionConfirmationStatus::Processed),
                 TransactionConfirmationStatus::Processed) => {
                    state = Some(TransactionConfirmationStatus::Processed);
                    trace!("processed");
                },
                (None | Some(TransactionConfirmationStatus::Processed) | Some(TransactionConfirmationStatus::Confirmed),
                 TransactionConfirmationStatus::Confirmed) => {
                    state = Some(TransactionConfirmationStatus::Confirmed);
                    trace!("confirmed");
                },
                (None | Some(TransactionConfirmationStatus::Processed) | Some(TransactionConfirmationStatus::Confirmed),
                 TransactionConfirmationStatus::Finalized) => {
                    assert!(status.confirmations.is_none());
                    trace!("finalized");
                    return Ok(());
                },
                _ => {
                    panic!("unexpected transaction status");
                }
            }
        } else {
            assert_eq!(state, None);
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    panic!("tx not finalized in time");
}

#[test]
fn get_signature_statuses_2() -> ClientResult<()> {
    solana_logger::setup();

    let alice = Keypair::new();
    let validator = TestValidator::with_no_fees(alice.pubkey(), None, SocketAddrSpace::Unspecified);
    let rpc_client = RpcClient::new(validator.rpc_url());

    let bob = Keypair::new();
    let lamports = 50;
    let (recent_blockhash, _) = rpc_client.get_recent_blockhash()?;
    let tx = system_transaction::transfer(&alice, &bob.pubkey(), lamports, recent_blockhash);
    let signature = rpc_client.send_transaction(&tx)?;

    let status = loop {
        let statuses = rpc_client.get_signature_statuses(&[signature])?.value;
        if let Some(status) = statuses[0].clone() {
            break status;
        }
        std::thread::sleep(Duration::from_millis(100));
    };

    assert!(status.err.is_none());

    Ok(())
}
