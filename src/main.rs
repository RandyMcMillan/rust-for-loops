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

  let mut my_vec = vec![1, 2, 3, 4];
  let removed_element = my_vec.remove(0);
  println!("Removed element: {}", removed_element);
  println!("Remaining vector: {:?}", my_vec);

    let mut args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    let first_element = args.remove(0);
    println!("first_element={}",first_element);
  println!("Remaining vector: {:?}", args);
    //let mut args = env::args().skip(1);
    //println!("{}",args);

    //if !args.is_empty() {
    //    let last_arg = args.pop().unwrap();
    //    println!("Last argument: {}", last_arg);
    //} else {
    //    println!("No arguments provided.");
    //}

    // Access arguments by index (be cautious with potential out-of-bounds)
    if args.len() >= 2 {
        let first_arg = &args[1];
        println!("First argument: {}", first_arg);
    } else {
        //println!("Please provide at least one argument.");
    }

    println!("rust-for-loops");
    system_calls(args.clone());

    if args.len() >= 2 {
        let mut args = env::args().skip(1); // Skip the program name

        if let Some(last_arg) = args.next_back() {
            println!("Last argument: {}", last_arg);
        } else {
            println!("No arguments provided.");
        }
        if let Some(last_arg) = args.next_back() {
            println!("Last argument: {}", last_arg);
        } else {
            println!("No arguments provided.");
        }
        if let Some(last_arg) = args.next_back() {
            println!("Last argument: {}", last_arg);
        } else {
            println!("No arguments provided.");
        }
    }
}
