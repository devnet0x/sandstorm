use wasm_bindgen::prelude::*;
use std::io::Cursor;
use web_sys::console;
use serde_json::Value;
use serde_json::from_str;
use hex;

use ark_ff::Field;
use ark_ff::PrimeField;
use ark_serialize::CanonicalDeserialize;
use ark_serialize::CanonicalSerialize;
use binary::AirPrivateInput;
use binary::AirPublicInput;
use binary::CompiledProgram;
use binary::Layout;
use binary::Memory;
use binary::RegisterStates;
use layouts::CairoWitness;
use ministark::stark::Stark;
use ministark::Proof;
use ministark::ProofOptions;
use ministark_gpu::fields::p3618502788666131213697322783095070105623107215331596699973092056135872020481;
use sandstorm::claims;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;
use structopt::StructOpt;


/// Modulus of Starkware's 252-bit prime field used for Cairo
const STARKWARE_PRIME_HEX_STR: &str =
    "0x800000000000011000000000000000000000000000000000000000000000001";

/// Modulus of 64-bit goldilocks field
#[cfg(feature = "experimental_claims")]
const GOLDILOCKS_PRIME_HEX_STR: &str = "0xffffffff00000001";


#[derive(StructOpt, Debug)]
enum Command {
    Prove {
        #[structopt(long, parse(from_os_str))]
        output: PathBuf,
        #[structopt(long, parse(from_os_str))]
        air_private_input: PathBuf,
        // TODO: add validation to the proof options
        #[structopt(long, default_value = "65")]
        num_queries: u8,
        #[structopt(long, default_value = "2")]
        lde_blowup_factor: u8,
        #[structopt(long, default_value = "16")]
        proof_of_work_bits: u8,
        #[structopt(long, default_value = "8")]
        fri_folding_factor: u8,
        #[structopt(long, default_value = "16")]
        fri_max_remainder_coeffs: u8,
    },
    Verify {
        #[structopt(long, default_value = "80")]
        required_security_bits: u8,
    },
}

fn convert_string_to_value(json_str: &str) -> Result<Value, serde_json::Error> {
    let value: Value = from_str(json_str)?;
    Ok(value)
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn main2(program_json_str: String,air_public_input_json_str: String,binary_data: &[u8]) {
    //log_bytes(binary_data);
    let proof: &[u8] = binary_data;
    let command: Command = Command::Verify {
        required_security_bits: 80
    };
    let air_public_input_json: JsValue = JsValue::from_str(&air_public_input_json_str);

    let data_str: String = air_public_input_json.as_string().unwrap();
    let data_reader = Cursor::new(data_str);

    let prime: String = "0x800000000000011000000000000000000000000000000000000000000000001".to_string();
    match prime.to_lowercase().as_str() {
        STARKWARE_PRIME_HEX_STR => {
            use p3618502788666131213697322783095070105623107215331596699973092056135872020481::ark::Fp;
            let program: CompiledProgram<Fp> = serde_json::from_value(convert_string_to_value(&program_json_str).unwrap()).unwrap();
            let air_public_input: AirPublicInput<Fp> =
                serde_json::from_reader(data_reader).unwrap();
            match air_public_input.layout {
                Layout::Starknet => {
                    use claims::starknet::EthVerifierClaim;
                    let claim = EthVerifierClaim::new(program, air_public_input);
                    execute_command(command, claim, proof);
                }
                Layout::Recursive => {
                    use claims::recursive::CairoVerifierClaim;
                    let claim = CairoVerifierClaim::new(program, air_public_input);
                    execute_command(command, claim, proof);
                }
                _ => unimplemented!(),
            }
        }
        #[cfg(feature = "experimental_claims")]
        GOLDILOCKS_PRIME_HEX_STR => {
            use ministark::hash::Sha256HashFn;
            use ministark::merkle::MatrixMerkleTreeImpl;
            use ministark::random::PublicCoinImpl;
            use ministark_gpu::fields::p18446744069414584321;
            use p18446744069414584321::ark::Fp;
            use p18446744069414584321::ark::Fq3;
            use sandstorm::CairoClaim;
            let program: CompiledProgram<Fp> = serde_json::from_value(convert_string_to_value(&program_json_str).unwrap()).unwrap();
            let air_public_input: AirPublicInput<Fp> =
                serde_json::from_reader(air_public_input_file).unwrap();
            match air_public_input.layout {
                Layout::Plain => {
                    type A = layouts::plain::AirConfig<Fp, Fq3>;
                    type T = layouts::plain::ExecutionTrace<Fp, Fq3>;
                    type M = MatrixMerkleTreeImpl<Sha256HashFn>;
                    type P = PublicCoinImpl<Fq3, Sha256HashFn>;
                    type C = CairoClaim<Fp, A, T, M, P>;
                    let claim = C::new(program, air_public_input);
                    execute_command(command, claim);
                }
                Layout::Starknet => {
                    unimplemented!("'starknet' layout does not support Goldilocks field")
                }
                Layout::Recursive => {
                    unimplemented!("'recursive' layout does not support Goldilocks field")
                }
                layout => unimplemented!("layout {layout} is not supported yet"),
            }
        }
        prime => unimplemented!("prime field p={prime} is not supported yet. Consider enabling the \"experimental_claims\" feature."),
    }
}

fn execute_command<Fp: PrimeField, Claim: Stark<Fp = Fp, Witness = CairoWitness<Fp>>>(
    command: Command,
    claim: Claim,
    proof: &[u8],
) {
    match command {
        Command::Prove {
            output,
            air_private_input,
            num_queries,
            lde_blowup_factor,
            proof_of_work_bits,
            fri_folding_factor,
            fri_max_remainder_coeffs,
        } => {
            let options = ProofOptions::new(
                num_queries,
                lde_blowup_factor,
                proof_of_work_bits,
                fri_folding_factor,
                fri_max_remainder_coeffs,
            );
            prove(options, &air_private_input, &output, claim)
        }
        Command::Verify {
            required_security_bits,
        } => verify(required_security_bits, &proof, claim),
    }
}

#[wasm_bindgen]
pub fn log_bytes(bytes: &[u8]) {
    let s = hex::encode(bytes);
}

fn verify<Claim: Stark<Fp = impl Field>>(
    required_security_bits: u8,
    proof_path: &[u8],
    claim: Claim,
) {
    let proof_bytes = proof_path;
    //log_bytes(proof_bytes);
    let proof = Proof::<Claim>::deserialize_compressed(&*proof_bytes).unwrap();
    //let now = window().unwrap().performance().unwrap().now();    
    claim.verify(proof, required_security_bits.into()).unwrap();
    //let elapsed = window().unwrap().performance().unwrap().now() - now;

    //println!("Proof verified in: {:?}", elapsed);
}

fn prove<Fp: PrimeField, Claim: Stark<Fp = Fp, Witness = CairoWitness<Fp>>>(
    options: ProofOptions,
    private_input_path: &PathBuf,
    output_path: &PathBuf,
    claim: Claim,
) {
    let private_input_file =
        File::open(private_input_path).expect("could not open private input file");
    let private_input: AirPrivateInput = serde_json::from_reader(private_input_file).unwrap();

    let trace_path = &private_input.trace_path;
    let trace_file = File::open(trace_path).expect("could not open trace file");
    let register_states = RegisterStates::from_reader(trace_file);

    let memory_path = &private_input.memory_path;
    let memory_file = File::open(memory_path).expect("could not open memory file");
    let memory = Memory::from_reader(memory_file);

    let witness = CairoWitness::new(private_input, register_states, memory);

    let now = Instant::now();
    let proof = pollster::block_on(claim.prove(options, witness)).unwrap();
    println!("Proof generated in: {:?}", now.elapsed());
    let security_level_bits = proof.security_level_bits();
    println!("Proof security (conjectured): {security_level_bits}bit");

    let mut proof_bytes = Vec::new();
    proof.serialize_compressed(&mut proof_bytes).unwrap();
    println!("Proof size: {:?}KB", proof_bytes.len() / 1024);
    let mut f = File::create(output_path).unwrap();
    f.write_all(proof_bytes.as_slice()).unwrap();
    f.flush().unwrap();
    println!("Proof written to {}", output_path.as_path().display());
}
