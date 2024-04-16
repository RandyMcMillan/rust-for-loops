use std::env;
fn system_calls(mut args: Vec<String>) {
    if !args.is_empty() {
        println!("system_calls:args={:?}", args);
        if args.len() >= 1 {
            println!("args[0]={:?}", args[0]);
            use std::process::Command;
            let mut cmd = Command::new(format!("rust-for-loop-{}", args.remove(0)));
            cmd.arg("4");
            println!("args={:?}", args);
            let output = cmd.output().expect("Failed to execute command");
            let _stdout = String::from_utf8_lossy(&output.stdout);
            let _stderr = String::from_utf8_lossy(&output.stderr);
            let _exit_code = cmd.status().expect("Failed to get exit code");
            println!("{}", _exit_code);
            use std::process;
            process::exit(0);
        }
    } else {
        usage();
    }
}

fn usage() {
    println!("Usage:");
    println!("	rust-for-loops [OPTIONS] <int> <int> <int>");
    use std::process;
    process::exit(0);
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let app_name = args.remove(0);
    //println!("app_name={}", app_name);
    if !args.is_empty() {
    println!("args={:?}",args);
    } else {
        usage();
    }
    system_calls(args.clone());
}
