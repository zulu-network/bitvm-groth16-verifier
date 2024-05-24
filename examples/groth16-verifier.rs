use ark_bn254::{Bn254, Fr};
use ark_ec::pairing::Pairing;
use ark_ff::Field;
use ark_groth16::Groth16;
use ark_std::{end_timer, start_timer};
use bitvm::execute_script;
use bitvm::groth16::verifier::Verifier;
use prove_on_pairing::dummy_circuit::gen_groth16_dummy_circuit_proof;

fn main() {
    type E = Bn254;

    let k = 6;
    let (proof, pvk, mut pi) = gen_groth16_dummy_circuit_proof::<E>(k);
    assert!(Groth16::<E>::verify_proof(&pvk, &proof, &pi).unwrap());

    let start = start_timer!(|| "collect_script");

    // Note: Need append one ahead of pi.
    let mut public_inputs = vec![Fr::ONE];
    public_inputs.append(&mut pi);
    let script = Verifier::verify_proof(&public_inputs, &proof, &pvk.vk, &pvk);
    end_timer!(start);

    println!("groth16-verifier = {} bytes", script.len());

    let start = start_timer!(|| "execute_script");
    let exec_result = execute_script(script);
    end_timer!(start);

    assert!(exec_result.success);
}
