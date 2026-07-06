use simplicity_ctf::artifacts::ctf::CtfProgram;
use simplicity_ctf::artifacts::ctf::derived_ctf::{CtfArguments, CtfWitness};
use simplicity_ctf::artifacts::asset_lock::AssetLockProgram;
use simplicity_ctf::artifacts::asset_lock::derived_asset_lock::{AssetLockArguments, AssetLockWitness};
use simplex::transaction::{FinalTransaction, PartialInput, PartialOutput, ProgramInput, RequiredSignature};
use simplex::transaction::partial_input::IssuanceInput;
use simplex::simplicityhl::simplicity::bitcoin::secp256k1::{Secp256k1, SecretKey};

#[test]
fn test_pk() {
    let secp = Secp256k1::new();
    let sk = SecretKey::from_slice(&[
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ]).unwrap();
    let pk = sk.public_key(&secp);
    println!("PrivKey=1 PubKey={:?}", pk.serialize());
}
