use std::io::{self, Write};     // Inputs
use std::process::Command;      // Commands

fn main() {
    // ==== Options (defaults) ================================================
    let installation_type: i32;                     // 0: cancel    1: default  2: custom
    let mut waywall_install: i32 = 1;               // 0: cancel    1: stable   2: latest   3: skip
    let mut instances: Vec<&str> = Vec::new();      // Vector of instance names
    let mut is_nvidia: bool = false;                // For the environment variable
    let mut is_internal_gpu: bool = false;          // If needed to not check "Use Discrete GPU"
    let mut is_latest_version: bool = false;        // For all the 26.1 tech
    let mut use_generic_config: bool = true;        // Clones generic config to ~/.config/waywall

    // ==== Prompts ===========================================================

    // == installation_type ===================================================
    header();
    let input = ask(r#"
======== What installation type do you want? ========

1) Default (recommended)

2) Custom 

Press Enter to cancel installation
    "#);

    installation_type = to_int(&input, 1, 2);

    if installation_type == 0 {
        // Cancel Install
        println!("Canceling Installation");
        return;
    }
    else if installation_type == 2 {
        println!("Rest of Prompts");


        // == waywall_install =================================================
        header();
        let input = ask(r#"
======= What version of Waywall do you want? ========

1) Stable (recommended)

2) Latest (built from source)

3) Skip (if waywall's already installed)

Press Enter to cancel installation
    "#);
        waywall_install = to_int(&input, 1, 3);
        if waywall_install == 0 {
            // Cancel Install
            println!("Canceling Installation");
            return;
        }

        // == instances =======================================================
        let mut instance_options: Vec<&str> = Vec::new(); // Vector of existing instances

        // ADD INSTANCES TEMP
        instance_options.push("RSG");
        instance_options.push("SSG");
        instance_options.push("Ranked");

        let mut instances_prompt: String = String::from("\n====== Which instances do you want to set up? =======\n         type each instance number (eg. 123)\n\n0) None\n\n"); 

        for (instance_number, instance_name) in instance_options.iter().enumerate() {
            instances_prompt.push_str(&format!("{}) {}\n\n", instance_number + 1, instance_name));
        }

        header();
        let input = ask(&instances_prompt);

        if to_int(&input, 0, 0) != 0 {
            for digit in input.chars().filter_map(|c| c.to_digit(10)) {
                let index = digit as usize;

                if index > 0 && index <= instance_options.len() {
                    let choice = instance_options[index - 1];
                    instances.push(choice);
                }
            }
        }


        // == is_nvidia =======================================================
        header();
        let input = ask(r#"
============ Are you using an Nvidia GPU? ===========

1) Yes

2) No
    "#);
        let int_input = to_int(&input, 1, 2);
        is_nvidia = if int_input == 1 {true} else {false};


        // == is_internal_gpu =================================================
        header();
        let input = ask(r#"
=========== Are you using an internal GPU? ==========

1) Yes

2) No
    "#);
        let int_input = to_int(&input, 1, 2);
        is_internal_gpu = if int_input == 1 {true} else {false};


        // == is_latest_version ===============================================
        header();
        let input = ask(r#"
=== Are you using Waywall on the latest version? ====

1) Yes

2) No
    "#);
        let int_input = to_int(&input, 1, 2);
        is_latest_version = if int_input == 1 {true} else {false};


        // == use_generic_config ==============================================
        header();
        let input = ask(r#"
===== Do you wish to install a Generic Config? ======

1) Yes

2) No
    "#);
        let int_input = to_int(&input, 1, 2);
        use_generic_config = if int_input == 1 {true} else {false};

    }


    // ==== Confirmation ======================================================
    header();
    println!("");
    println!("Confirm Options:");
    println!("    Waywall Version:          {}", if waywall_install == 1 {"Stable (prebuilt package)"} else if waywall_install == 2 {"Latest (git)"} else {"None"});
    println!("    Instances to Setup:       {}", instances.join(", "));
    println!("    Nvidia GPU:               {}", if is_nvidia {"True"} else {"False"});
    println!("    Internal GPU:             {}", if is_internal_gpu {"True"} else {"False"});
    println!("    Latest Version:           {}", if is_latest_version {"True"} else {"False"});
    println!("    Install Generic Config:   {}", if use_generic_config {"True"} else {"False"});


    // ==== Installation ======================================================
}

fn header() {
    clearscreen::clear().expect("Failed to clear screen");
    println!(r#"=====================================================
                Waywall Installer CLI
                       by Gore
====================================================="#);
}

fn ask(prompt: &str) -> String{
    println!("{}", prompt);

    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Blank");

    input.trim().to_string()
}

fn to_int(input: &str, min: i32, max: i32) -> i32{

    // Convert to integer
    let int_input: i32 = match input.trim().parse() {
        Ok(num) => {
            if (min == 0) && (max == 0) {
                num
            }
            else if (num >= min) && (num <= max) {
                num
            }
            else {
                0
            }
        }
        Err(_) => {
            0
        }
    };
    int_input
}

fn run_command(cmd: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Success:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error:\n{}", stderr);
    }
}

