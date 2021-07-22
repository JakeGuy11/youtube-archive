extern crate home;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::io::BufRead;
use std::io::Write;

#[derive(Debug)]
enum ReasonForFail
{
    DirectoryCreationFailure,
    FileWritingFailure,
    FileNotFound,
    EntryNotFound
}

fn tup_vec_to_file<T: std::fmt::Display, U: std::fmt::Display>(target_vec: &Vec<(T,U)>, delim: &str, target_file: &PathBuf) -> Result<(), ReasonForFail>
{
    // First create the directories if they don't exist
    let dir_res = std::fs::create_dir_all(target_file.as_path());
    if let Err(e) = dir_res { if e.kind() != std::io::ErrorKind::AlreadyExists { return Err(ReasonForFail::DirectoryCreationFailure); } }

    // Create the OpenOptions for the file
    let target_file_opts_res = std::fs::OpenOptions::new().write(true).append(true).create(true).open(target_file.as_path());
    let mut target_file_opts = match target_file_opts_res
    {
        Ok(f) => { f }
        Err(_) => { return Err(ReasonForFail::FileWritingFailure); }
    };

    // Delete the contents of the file
    match std::fs::File::create(target_file)
    {
        Ok(_) => {  },
        Err(_) => { return Err(ReasonForFail::FileWritingFailure); }
    }

    // Now iterate through the entries of the vector, appending to the vector
    for item in target_vec.iter()
    {
        match writeln! (target_file_opts, "{}{}{}", item.0, delim, item.1 )
        {
            Ok(_) => {  },
            Err(_) => { return Err(ReasonForFail::FileWritingFailure); }
        }
    }
    
    Ok(())
}

fn file_to_tup_vec(delim: &str, target_file: &PathBuf) -> Result<Vec<(String,String)>, ReasonForFail>
{
    // First get the contents of the file
    let pref_file = match std::fs::File::open(target_file)
    {
        Err(_) => { return Err(ReasonForFail::FileNotFound); },
        Ok(file) => file
    };
    let file_reader = std::io::BufReader::new(pref_file);
    let entries_res: Vec<Result<String, std::io::Error>> = file_reader.lines().collect();

    // Now split the lines
    let mut entries: Vec<(String,String)> = Vec::new();
    for current_line in entries_res.iter()
    {
        let line = match current_line
        {
            Ok(ln) => ln,
            Err(_) => { eprintln! ("Failed to read line of queue - continuing without it."); continue; }
        };
        let formatted_line: (String,String) = (line.split(delim).nth(0).unwrap_or("UNDEFINED").to_string(),line.split(delim).last().unwrap_or("UCK9V2B22uJYu3N7eR_BT9QA").to_string());
        entries.push(formatted_line);
    }

    Ok(entries)
}

fn remove_from_queue(req_arg: &String, target_file: &mut PathBuf) -> Result<(), ReasonForFail>
{
    // First get the entries from the file
    let entries_res = file_to_tup_vec(":", target_file);
    let mut entries = match entries_res
    {
        Ok(entries_vec) => entries_vec,
        Err(e) => { return Err(e); }
    };

    println! ("Found entries: {:?}", entries);

    // Now scan through the elements and remove the last one that matches `req_arg`
    let mut removal_index: Option<u32> = None;
    for i in 0..entries.len() { if &entries[i].0 == req_arg || &entries[i].1 == req_arg { removal_index = Some(i as u32); } }
    match removal_index
    {
        Some(i) => { entries.remove(i as usize); },
        None => { return Err(ReasonForFail::EntryNotFound); }
    }

    println! ("writing {:?}", entries);

    // Write that back into the config file
    match tup_vec_to_file(&entries, ":",  target_file)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

fn add_to_queue(req_name: &String, req_id: &String, target_file: &mut PathBuf) -> Result<(), ReasonForFail>
{
    let mut entries = match file_to_tup_vec(":", target_file)
    {
        Ok(vec) => vec,
        Err(e) => { return Err(e); }
    };

    entries.push((req_name.to_string(),req_id.to_string()));

    match tup_vec_to_file(&entries, ":", target_file)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

//
fn verify_paths(paths: Vec<PathBuf>) -> Result<(), ReasonForFail>
{

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
                    Err(e) => { eprintln! ("Something really bad happened: {:?}", e); std::process::exit(1); }
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
