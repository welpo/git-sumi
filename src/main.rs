fn main() {
    if let Err(e) = git_sumi::run() {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
}
