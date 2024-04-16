fn for_loop_1() {
    use std::process::Command;

    let mut cmd = Command::new("rust-for-loop-1");
    //let mut cmd = Command::new("ls");
    let output = cmd.output().expect("Failed to execute command");
    let _stdout = String::from_utf8_lossy(&output.stdout);
    let _stderr = String::from_utf8_lossy(&output.stderr);
    let _exit_code = cmd.status().expect("Failed to get exit code");
    println!("{}", _exit_code);
}
fn system_calls() {
    for_loop_1();
}

fn main() {
    println!("rust-for-loops");
    system_calls();
}
