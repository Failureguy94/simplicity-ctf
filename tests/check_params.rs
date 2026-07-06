use simplicity_ctf::artifacts::ctf::CtfProgram;
use simplicity_ctf::artifacts::ctf::derived_ctf::CtfArguments;
use simplicity_ctf::artifacts::asset_lock::AssetLockProgram;
use simplicity_ctf::artifacts::asset_lock::derived_asset_lock::AssetLockArguments;
use simplex::provider::SimplicityNetwork;

fn decode_hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

fn encode_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn nonce_to_u256_bytes(nonce: u64) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[24..32].copy_from_slice(&nonce.to_be_bytes());
    bytes
}

#[test]
fn check_mainnet_scripts() {
    let network = SimplicityNetwork::Liquid;
    
    let pubkey_hex = "6ace88f8a725d9f78cc8f0b02f9a1b128871bc4f9c4dfabdc7d48489fdfc0f4c";
    let mut pubkey_bytes = [0u8; 32];
    pubkey_bytes.copy_from_slice(&decode_hex(pubkey_hex));
    
    let auth_asset_hex = "6e49cd6ef8acd9e2fe5e59a34fbc8ab4db81c6d6aaf30f2d240d77e84cc3b739";
    let mut auth_asset_bytes = [0u8; 32];
    auth_asset_bytes.copy_from_slice(&decode_hex(auth_asset_hex));
    auth_asset_bytes.reverse();
    
    let ctf_args = CtfArguments {
        owner_pubkey: pubkey_bytes,
        auth_asset_id: auth_asset_bytes,
    };
    let ctf_program = CtfProgram::new(ctf_args);
    let ctf_script = ctf_program.get_script_pubkey(&network);
    
    println!("Computed CTF script: {:?}", encode_hex(ctf_script.as_bytes()));
    
    let al_args = AssetLockArguments { owner_pubkey: pubkey_bytes };
    for nonce in 0u64..3 {
        let mut al = AssetLockProgram::new(al_args.clone()).with_storage_capacity(1);
        let _ = al.set_storage_at(0, nonce_to_u256_bytes(nonce));
        let script = al.get_script_pubkey(&network);
        println!("Computed AssetLock script (nonce {}): {:?}", nonce, encode_hex(script.as_bytes()));
    }
}
