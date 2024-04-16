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
    if !args.is_empty() {
        println!("system_calls:args={:?}", args);
        //let last_arg = args.pop().unwrap();
        //println!("Last argument: {}", last_arg);

        if args.len() == 1 {
            println!("args[0]={:?}",args[0]);
            use std::process;
            process::exit(0);
        }
        if args.len() == 2 {
            println!("args[0]={:?}",args[0]);
            println!("args[1]={:?}",args[1]);
            use std::process;
            process::exit(0);
        }
        if args.len() == 3 {
            println!("args[0]={:?}",args[0]);
            println!("args[1]={:?}",args[1]);
            println!("args[2]={:?}",args[2]);
            use std::process;
            process::exit(0);
        }
        if args.len() == 4 {
            println!("args[0]={:?}",args[0]);
            println!("args[1]={:?}",args[1]);
            println!("args[2]={:?}",args[2]);
            println!("args[3]={:?}",args[3]);
            use std::process;
            process::exit(0);
        }
    } else {
        println!("system_calls:No arguments provided.");
        usage();
    }
    if args.len() >= 2 {
        let first_arg = &args[1];
        println!("system_calls:First argument: {}", first_arg);
        if first_arg == "1" {
            for_loop_1(args.clone());
        }
        use std::process;
        process::exit(0);
    } else {
        usage();
    }
    //for_loop_1(args);
}

fn usage() {
    println!("Usage:");
    println!("	rust-for-loops [OPTIONS] <int>");
    println!("	\n");
    println!("	rust-for-loops [OPTIONS] <int>");
    use std::process;
    process::exit(0);
}

fn main() {
    //let mut my_vec = vec![1, 2, 3, 4];
    //let removed_element = my_vec.remove(0);
    //println!("Removed element: {}", removed_element);
    //println!("Remaining vector: {:?}", my_vec);

    let mut args: Vec<String> = env::args().collect();

    //println!("{:?}", args);

    let app_name = args.remove(0);

    println!("app_name={}", app_name);

    println!("Remaining args: {:?}", args);

    //let mut args = env::args().skip(1);
    //println!("args={:?}",args);

    if !args.is_empty() {
        //let last_arg = args.pop().unwrap();
        //println!("Last argument: {}", last_arg);
    } else {
        //println!("No arguments provided.");
    }
    println!("args={:?}", args);
    system_calls(args.clone());

    // Access arguments by index (be cautious with potential out-of-bounds)
    //if args.len() >= 2 {
    //    let first_arg = &args[1];
    //    println!("First argument: {}", first_arg);
    //} else {
    //    //println!("Please provide at least one argument.");
    //}

    //println!("rust-for-loops ");
    //system_calls(args.clone());

    //if args.len() >= 2 {
    //    let mut args = env::args().skip(1); // Skip the program name

    //    if let Some(last_arg) = args.next_back() {
    //        println!("Last argument: {}", last_arg);
    //    } else {
    //        println!("No arguments provided.");
    //    }
    //    if let Some(last_arg) = args.next_back() {
    //        println!("Last argument: {}", last_arg);
    //    } else {
    //        println!("No arguments provided.");
    //    }
    //    if let Some(last_arg) = args.next_back() {
    //        println!("Last argument: {}", last_arg);
    //    } else {
    //        println!("No arguments provided.");
    //    }
    //}
}
