fn usage() {
    println!("Usage:");
    println!("	rust-for-loops [OPTIONS] [SUB-COMMANDS]");
    println!("");
    println!("	[OPTIONS]");
    println!("");
    println!("	-h --help");
    println!("");
    println!("	[SUB-COMMANDS]");
    println!("");
    println!("	0 1 2 3 4 5 6 7 8 9 10 git");
    use std::process;
    process::exit(0);
}
fn main() {
    usage();
}
