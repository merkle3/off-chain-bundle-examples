use ethers::contract::MultiAbigen;

fn main() {
    // generate abi
    let gen = MultiAbigen::from_json_files("../contracts/out")
        .expect("Failed to generate Abis!");

    // compile and write rust modules
    let build = gen.build().expect("Failed to build bindings!");
    build.write_to_module("../crates/bindings", false)
        .expect("Failed to write bindings!");
    
    println!("Built blind-backrun contracts!");
}
