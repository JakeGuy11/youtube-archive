extern crate home;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::io::BufRead;
use std::io::Write;

enum ReasonForFail
{
    DirectoryCreationFailure,
    FileWritingFailure,
    FileNotFound,
    EntryNotFound
}

/*
fn tup_vec_to_file<T,U>(target_vec: Vec<(T,U)>, target_file: PathBuf) -> Result<(), ReasonForFail>
{
    // Get the file
    let dir_res = std::fs::create_dir_all(target_file);
    Ok(())
}
*/

fn remove_from_queue(req_arg: &String, target_file: &mut PathBuf) -> Result<(), ReasonForFail>
{
    // First get the contents of a file into an organized vector
    let pref_file = match std::fs::File::open(target_file)
    {
        Err(_) => { return Err(ReasonForFail::FileNotFound); },
        Ok(file) => file
    };
    let file_reader = std::io::BufReader::new(pref_file);
    let entries_res: Vec<Result<String, std::io::Error>> = file_reader.lines().collect();
    let mut entries: Vec<(String,String)> = Vec::new();
    for current_line in entries_res.iter()
    {
        let line = match current_line
        {
            Ok(ln) => ln,
            Err(_) => { eprintln! ("Failed to read line of queue - continuing without it."); continue; }
        };
        let formatted_line: (String,String) = (line.split(":").nth(0).unwrap_or("UNDEFINED").to_string(),line.split(":").last().unwrap_or("UCK9V2B22uJYu3N7eR_BT9QA").to_string());
        entries.push(formatted_line);
    }
    
    // Now scan through the elements and remove the last one that matches `req_arg`
    let mut removal_index: Option<u32> = None;
    for i in 0..entries.len() { if &entries[i].0 == req_arg || &entries[i].1 == req_arg { removal_index = Some(i as u32); } }
    match removal_index
    {
        Some(i) => { entries.remove(i as usize); },
        None => { return Err(ReasonForFail::EntryNotFound); }
    }

    // Write that back into the config file
    // TODO: Impliment the `tup_vec` functions
    for item in entries.iter()
    {

    }

    Ok(())
}

fn add_to_queue(req_name: &String, req_id: &String, target_file: &mut PathBuf) -> Result<(), ReasonForFail>
{
    // First create the directories if they don't exist
    let dir_res = std::fs::create_dir_all(target_file.as_path());
    if let Err(_) = dir_res { return Err(ReasonForFail::DirectoryCreationFailure); }
    
    // Create the OpenOptions for the file
    let pref_file_res = std::fs::OpenOptions::new().write(true).append(true).create(true).open(target_file.as_path());
    let mut pref_file = match pref_file_res
    {
        Ok(f) => { f }
        Err(_) => { return Err(ReasonForFail::FileWritingFailure); }
    };

    match writeln!(pref_file, "{}", format!("{}:{}",req_name,req_id))
    {
        Ok(_) => { Ok(()) }
        Err(_) => { Err(ReasonForFail::FileWritingFailure) }
    }
}

fn main() {

    // Declare some paths we'll need
    let mut pref_path = home::home_dir().expect("Failed to find user's home directory!");
    pref_path.push(".config");
    pref_path.push("youtube-archive");
    pref_path.push("queue");
    let mut dl_path = home::home_dir().expect("Failed to find user's home directory!");
    dl_path.push("videos");
    dl_path.push("youtube-archive");

    // First collect all the cli args into a vector
    let cli_args: Vec<String> = std::env::args().skip(1).collect();

    // Check for debug mode first
    let debug_flag = String::from("--debug");
    let debug_enabled = if cli_args.contains(&debug_flag) { println! ("Debug mode enabled"); true } else { false };

    // Get which channels to check
    let channel_vec = Arc::new(Mutex::new(vec![(String::from("Calli"),String::from("UCL_qhgtOy0dy1Agp8vkySQg")), (String::from("IRyS"),String::from("UC8rcEBzJSleTkf_-agPM20g")), (String::from("Polka"),String::from("UCK9V2B22uJYu3N7eR_BT9QA"))]));

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
                let add_id = attempted_add_id.unwrap().split("/").last().unwrap().to_string();
                if add_id.len() != 24 { eprintln! ("You must specify a valid channel id!"); std::process::exit(1); } // Check if the length of the id is an appropriate size

                // Add the entry
                if debug_enabled { println! ("User intends to add channel with nickname {} and id {}", add_name, add_id); }
                match add_to_queue(add_name, &add_id, &mut pref_path)
                {
                    Ok(_) => { if debug_enabled { println! ("Added channel id {} with nickname {}", add_id, add_name); } },
                    Err(ReasonForFail::DirectoryCreationFailure) => { eprintln! ("Could not create preference directory! Do you have permission?"); std::process::exit(1); },
                    Err(ReasonForFail::FileWritingFailure) => { eprintln! ("Could not write queue file! Do you have permission?"); std::process::exit(1); },
                    Err(_) => { eprintln! ("Something really bad happened"); std::process::exit(1); }
                }
            }
            "-r" | "--remove" =>
            {
                remove_from_queue(&String::new(), &mut pref_path);
            }
            _ => {  }
        }
    }
}
