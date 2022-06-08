use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let original_sv = &args[2];
    // let original_sv = &args[2];

    println!("Version {}",original_sv);
}