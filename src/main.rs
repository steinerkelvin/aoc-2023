use std::process::Command;

fn run_bin(name: &str) {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(name)
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn main() {
    let bins = ["day_1_a", "day_1_b", "day_2_a", "day_2_b", "day_3_a"];
    for bin in &bins {
        println!("Running {}", bin);
        run_bin(bin);
    }
}
