//! Entry point for {{project-name}}

fn main() {
    if let Err(e) = run() {
        erpintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello from {}!", env!("CARGO_PKG_NAME"));
    Ok(())
}
