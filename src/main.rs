use std::sync::{Arc, Mutex};

fn main() {
    // First collect all the cli args into a vector
    let cli_args: Vec<String> = std::env::args().skip(1).collect();

    // Check for debug mode first
    let debug_flag = String::from("--debug");
    let debug_enabled = if cli_args.contains(&debug_flag) { println! ("Debug mode enabled"); true } else { false };

    // Get which channels to check
    let channel_vec = Arc::new(Mutex::new(vec![String::from("UCL_qhgtOy0dy1Agp8vkySQg"), String::from("UC8rcEBzJSleTkf_-agPM20g"), String::from("UCK9V2B22uJYu3N7eR_BT9QA")]));

    // Go through each argument
    for i in 0..cli_args.len()
    {
        if debug_enabled { println! ("Parsing arg {}: {}", i, cli_args[i]); }
        match cli_args[i].as_str()
        {
            "-h" | "--help" => { println! ("Help message will go here"); },
            "-a" | "--add" => 
            {
                // Get the name they want to add it as
                let attempted_add_name = cli_args.get(i+1);

                // Do some checks to make sure it's valid
                if let None = attempted_add_name { eprintln! ("You must specify a valid nickname!"); std::process::exit(1); }
                let add_name = attempted_add_name.unwrap();
                if add_name.len() == 0 { eprintln! ("You must specify a valid nickname!"); std::process::exit(1); }

                // Get the id they want to add
                let attempted_add_id = cli_args.get(i+2);

                // Do some checks to make sure it's valid
                if let None = attempted_add_id { eprintln! ("You must specify a valid channel id!"); std::process::exit(1); } // Check if there is a next arg
                let add_id = attempted_add_id.unwrap().split("/").last().unwrap();
                if add_id.len() != 24 { eprintln! ("You must specify a valid channel id!"); std::process::exit(1); } // Check if the length of the id is an appropriate size

                // Add the entry
                if debug_enabled { println! ("User intends to add channel with nickname {} and id {}", add_name, add_id); }

            }
            _ => { if debug_enabled { println! ("Arg not recognized - skipping"); } }
        }
    }
}
