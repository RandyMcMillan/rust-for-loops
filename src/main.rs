use std::env;
fn for_loop_1(args: Vec<String>) {
    if args.len() >= 2 {
        let first_arg = &args[1];
        println!("for_loop_1:First argument: {}", first_arg);
        use std::process::Command;

        let mut cmd = Command::new(format!("rust-for-loop-{}", first_arg));
        //let mut cmd = Command::new("ls");
        let output = cmd.output().expect("Failed to execute command");
        let _stdout = String::from_utf8_lossy(&output.stdout);
        let _stderr = String::from_utf8_lossy(&output.stderr);
        let _exit_code = cmd.status().expect("Failed to get exit code");
        println!("{}", _exit_code);
    } else {
        //println!("Please provide at least one argument.");
    }
}
fn system_calls(args: Vec<String>) {
    if args.len() >= 2 {
        let first_arg = &args[1];
        println!("system_calls:First argument: {}", first_arg);
        if first_arg == "1" {
            for_loop_1(args.clone());
        }
    } else {
        //println!("Please provide at least one argument.");
    }
    //for_loop_1(args);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Access arguments by index (be cautious with potential out-of-bounds)
    if args.len() >= 2 {
        let first_arg = &args[1];
        println!("First argument: {}", first_arg);
    } else {
        //println!("Please provide at least one argument.");
    }

    println!("rust-for-loops");
    system_calls(args);
}
