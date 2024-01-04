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

// Send  panic messages to the console.error
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn main2(command: String, 
    program_json_str: String,
    air_public_input_json_str: String,
    proof_file: &[u8], 
    trace_file: &[u8],
    memory_file: &[u8],
    air_private_input_json_str: String) -> Vec<u8> {
    //log_bytes(binary_data);
    // Parse the command
    //web_sys::console::log_1(&JsValue::from_str(line!().to_string().as_str()));
    web_sys::console::log_1(&JsValue::from_str("010"));
    let output_str = "".to_string();
    let command: Command = match command.as_str() {
        "Verify" => Command::Verify { required_security_bits: 80 },
        "Prove" => Command::Prove {
            num_queries: 65,
            lde_blowup_factor: 2,
            proof_of_work_bits: 16,
            fri_folding_factor: 8,
            fri_max_remainder_coeffs: 16,
        },
        _ => panic!("Unknown command"),
    };
    web_sys::console::log_1(&JsValue::from_str("020"));
    let air_public_input_json: JsValue = JsValue::from_str(&air_public_input_json_str);
    web_sys::console::log_1(&JsValue::from_str("030"));
    let data_str: String = air_public_input_json.as_string().unwrap();
    let data_reader = Cursor::new(data_str);
    web_sys::console::log_1(&JsValue::from_str("040"));
    // TODO: Read prime field from program_json_str json string
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
                    execute_command(command, claim, proof_file, trace_file, memory_file, air_private_input_json_str, output_str)
                }
                Layout::Recursive => {
                    use claims::recursive::CairoVerifierClaim;
                    web_sys::console::log_1(&JsValue::from_str("050"));
                    let claim = CairoVerifierClaim::new(program, air_public_input);
                    web_sys::console::log_1(&JsValue::from_str("060"));
                    execute_command(command, claim, proof_file, trace_file, memory_file, air_private_input_json_str, output_str)
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
    proof_file: &[u8],
    trace_file: &[u8],
    memory_file: &[u8],
    air_private_input: String,
    output: String,
) -> Vec<u8> {
    match command {
        Command::Prove {
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
            web_sys::console::log_1(&JsValue::from_str("070"));
            prove(options, air_private_input, &trace_file, &memory_file, &output, claim)
        }
        Command::Verify {
            required_security_bits,
        } => verify(required_security_bits, &proof_file, claim),
    }
}

#[wasm_bindgen]
pub fn log_bytes(bytes: &[u8]) {
    let s = hex::encode(bytes);
}

fn verify<Claim: Stark<Fp = impl Field>>(
    required_security_bits: u8,
    proof_bytes: &[u8],
    claim: Claim,
) -> Vec<u8> {
    //log_bytes(proof_bytes);
    let proof = Proof::<Claim>::deserialize_compressed(&*proof_bytes).unwrap();
    //let now = window().unwrap().performance().unwrap().now();    
    claim.verify(proof, required_security_bits.into()).unwrap();
    //let elapsed = window().unwrap().performance().unwrap().now() - now;

    //println!("Proof verified in: {:?}", elapsed);
    0u8.to_be_bytes().to_vec()
}

fn prove<Fp: PrimeField, Claim: Stark<Fp = Fp, Witness = CairoWitness<Fp>>>(
    options: ProofOptions,
    private_input_file: String,
    trace_file: &[u8],
    memory_file: &[u8],
    output_path: &String,
    claim: Claim,
) -> Vec<u8> {
    //let private_input_file =
    //    File::open(private_input_path).expect("could not open private input file");
    web_sys::console::log_1(&JsValue::from_str("080"));
    let private_input: AirPrivateInput = serde_json::from_reader(private_input_file.as_bytes()).unwrap();

    //let trace_path = &private_input.trace_path;
    //let trace_file = File::open(trace_path).expect("could not open trace file");
    web_sys::console::log_1(&JsValue::from_str("090"));
    let register_states = RegisterStates::from_reader(trace_file);

    //let memory_path = &private_input.memory_path;
    //let memory_file = File::open(memory_path).expect("could not open memory file");
    web_sys::console::log_1(&JsValue::from_str("100"));
    let memory = Memory::from_reader(memory_file);

    web_sys::console::log_1(&JsValue::from_str("110"));
    let witness = CairoWitness::new(private_input, register_states, memory);

    //let now = Instant::now();
    web_sys::console::log_1(&JsValue::from_str("120"));
    let proof = pollster::block_on(claim.prove(options, witness)).unwrap();
    //println!("Proof generated in: {:?}", now.elapsed());
    web_sys::console::log_1(&JsValue::from_str("130"));
    let security_level_bits = proof.security_level_bits();
    //println!("Proof security (conjectured): {security_level_bits}bit");

    web_sys::console::log_1(&JsValue::from_str("140"));
    let mut proof_bytes = Vec::new();
    web_sys::console::log_1(&JsValue::from_str("150"));
    proof.serialize_compressed(&mut proof_bytes).unwrap();
    web_sys::console::log_1(&JsValue::from_str("160"));
    proof_bytes
    // println!("Proof size: {:?}KB", proof_bytes.len() / 1024);
    // let mut f = File::create(output_path).unwrap();
    // f.write_all(proof_bytes.as_slice()).unwrap();
    // f.flush().unwrap();
    // println!("Proof written to {}", output_path.as_path().display());
}
