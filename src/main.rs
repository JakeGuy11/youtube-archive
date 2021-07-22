extern crate home;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::io::Write;

enum FailReason
{
    DirectoryCreationFailure,
    FileWritingFailure
}

fn add_to_queue(req_name: String, req_id: String, target_dir: PathBuf) -> Result<(), FailReason>
{
    // First create the directories if they don't exist
    let dir_res = std::fs::create_dir_all(target_dir.as_path());
    if let Err(_) = dir_res { return Err(FailReason::DirectoryCreationFailure); }
    
    // Create the path of the actual pref file
    let mut target_file = target_dir.clone();
    target_file.push("queue");

    // Create the OpenOptions for the file
    let pref_file_res = std::fs::OpenOptions::new().write(true).append(true).open(target_file.as_path());
    let mut pref_file = match pref_file_res
    {
        Ok(f) => { f }
        Err(_) => { return Err(FailReason::FileWritingFailure); }
    };

    match writeln!(pref_file, "{}", format!("{}:{}",req_name,req_id))
    {
        Ok(_) => { Ok(()) }
        Err(_) => { Err(FailReason::FileWritingFailure) }
    }
}

fn main() {

    // Declare some paths we'll need
    let mut pref_path = home::home_dir().expect("Failed to find user's home directory!");
    pref_path.push(".config");
    pref_path.push("youtube-archive");
    let mut dl_path = home::home_dir().expect("Failed to find user's home directory!");
    dl_path.push("videos");
    dl_path.push("youtube-archive");

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
