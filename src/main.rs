use std::io::{self, Write};     // Inputs
use std::process::Command;      // Commands
use std::fs;                   // Read file

fn main() {
    // ==== Detect waywall package name based on distro =======================
    let _distro = fs::read_to_string("/etc/os-release").unwrap();
    let distro: &str = _distro
        .lines()
        .find(|l| l.starts_with("ID_LIKE="))
        .or_else(|| _distro.lines().find(|l| l.starts_with("ID=")))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|l| l.split(' ').last())
        .map(|s| s.trim_matches('"'))
        .unwrap_or("unknown");

    // ==== Options (defaults) ================================================
    let installation_type: i32;                     // 0: cancel    1: default  2: custom
    let mut waywall_install: i32 = 1;               // 0: cancel    1: stable   2: latest   3: skip
    let mut instances: Vec<&str> = Vec::new();      // Vector of instance names
    let mut is_nvidia: bool = false;                // For the environment variable
    let mut is_internal_gpu: bool = false;          // If needed to not check "Use Discrete GPU"
    let mut is_latest_version: bool = false;        // For all the 26.1 tech
    let mut use_generic_config: bool = true;        // Clones generic config to ~/.config/waywall
    let waywall_release_tag: &str = "0.2026.06.13"; // Waywall release tag in github releases

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
        is_nvidia = int_input == 1;


        // == is_internal_gpu =================================================
        header();
        let input = ask(r#"
=========== Are you using an internal GPU? ==========

1) Yes

2) No
    "#);
        let int_input = to_int(&input, 1, 2);
        is_internal_gpu = int_input == 1;


        // == is_latest_version ===============================================
        header();
        let input = ask(r#"
=== Are you using Waywall on the latest version? ====

1) Yes

2) No
    "#);
        let int_input = to_int(&input, 1, 2);
        is_latest_version = int_input == 1;


        // == use_generic_config ==============================================
        header();
        let input = ask(r#"
===== Do you wish to install a Generic Config? ======

1) Yes

2) No
    "#);
        let int_input = to_int(&input, 1, 2);
        use_generic_config = int_input == 1;

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
    //
    // === Waywall installation
    

}

fn waywall(itype: i32, distro: &str, waywall_tag: &str) {
    // Install waywall
    if itype == 1 {
        // Download the waywall package
        match distro {
            "arch" => run_command(&format!("curl -fsSL github.com/tesselslate/waywall/releases/download/{}/waywall-0.5-1-x86_64.pkg.tar.zst -O /tmp/waywall.pkg.tar.zst", waywall_tag)),
            "fedora" => run_command(&format!("curl -fsSL github.com/tesselslate/waywall/releases/download/{}/waywall-0.5-1.fc42.x86_64.rpm -O /tmp/waywall.rpm", waywall_tag)),
            "debian" => run_command(&format!("curl -fsSL github.com/tesselslate/waywall/releases/download/{}/waywall_0.5-1_amd64.deb -O /tmp/waywall.deb", waywall_tag)),
            _ => println!("Unknown distro type found: {}", distro),
        }
        // Install the waywall package
        match distro {
            "arch" => run_command("pacman -S /tmp/waywall.pkg.tar.zst"),
            "fedora" => run_command("dnf install /tmp/waywall.rpm"),
            "debian" => run_command("apt install /tmp/waywall/deb"),
            _ => println!("Unknown distro type found: {}", distro),
        }
    }
    else {
    
    }

    // Download generic
    println!("Downloading Gore's generic config");
    run_command("ls ~/.config/waywall 2>&1 >/dev/null || mv ~/.config/waywall ~/.config/waywall.bkp"); // Check for existing configuration and incase of it existing move it to a backup
    run_command("git clone https://github.com/arjuncgore/waywall_generic_config.git ~/.config/waywall"); // Download it
    println!("Generic config downloaded!");
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

