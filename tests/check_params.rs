use simplicity_ctf::artifacts::ctf::CtfProgram;
use simplicity_ctf::artifacts::ctf::derived_ctf::CtfArguments;
use simplex::provider::SimplicityNetwork;
use simplex::simplicityhl::simplicity::bitcoin::secp256k1::{Secp256k1, SecretKey};

fn decode_hex(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

fn encode_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

#[test]
fn check_mainnet_scripts() {
    let network = SimplicityNetwork::Liquid;
    
    let auth_asset_hex = "6e49cd6ef8acd9e2fe5e59a34fbc8ab4db81c6d6aaf30f2d240d77e84cc3b739";
    let mut auth_asset_bytes = [0u8; 32];
    auth_asset_bytes.copy_from_slice(&decode_hex(auth_asset_hex));
    auth_asset_bytes.reverse();
    
    let candidates = vec![
        // contract_hash
        "f294856522edcbfc6cf6cd605d9ffa8e13f7c7d6157ed4f3c04d74f973206422",
        // asset_entropy
        "e3aaf335cf888e5a664ed25f86c30be3383d117b007d5079b1c2658ef743b1cf",
        // txid
        "aa52a138a0e193c8530e1195b201c7139de194decc0ff3bb01489adbe814095c",
        // all 1s
        "0000000000000000000000000000000000000000000000000000000000000001",
    ];
    
    let secp = Secp256k1::new();
    
    for candidate_hex in candidates {
        let mut sk_bytes = [0u8; 32];
        sk_bytes.copy_from_slice(&decode_hex(candidate_hex));
        if let Ok(sk) = SecretKey::from_slice(&sk_bytes) {
            let pk = sk.public_key(&secp).x_only_public_key().0;
            let ctf_args = CtfArguments {
                owner_pubkey: pk.serialize(),
                auth_asset_id: auth_asset_bytes,
            };
            let ctf_program = CtfProgram::new(ctf_args);
            let ctf_script = ctf_program.get_script_pubkey(&network);
            
            println!("Candidate {}:", candidate_hex);
            println!("  CTF script: {:?}", encode_hex(ctf_script.as_bytes()));
        }
    }
}
