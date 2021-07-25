extern crate home;
extern crate tokio;
use tokio::time::{sleep, Duration};
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
        if format! ("{}",item.0) == String::from("") || format! ("{}",item.1) == String::from("") { continue; }
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
    let entries_res = std::io::BufReader::new(pref_file).lines();

    // Now split the lines
    let mut entries: Vec<(String,String)> = Vec::new();
    for current_line in entries_res
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

    // Now scan through the elements and remove the last one that matches `req_arg`
    let mut removal_index: Option<u32> = None;
    for i in 0..entries.len() { if &entries[i].0 == req_arg || &entries[i].1 == req_arg { removal_index = Some(i as u32); } }
    match removal_index
    {
        Some(i) => { entries.remove(i as usize); },
        None => { return Err(ReasonForFail::EntryNotFound); }
    }

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

fn list_queue(queue_path: &PathBuf) -> Result<(), ReasonForFail>
{
   // Read the queue from the file
   let entries = match file_to_tup_vec(":", queue_path)
   {
       Ok(vec) => vec,
       Err(e) => { return Err(e); }
   };

   // Iterate through the entries, printing them
   for entry in entries.iter()
   {
       println! ("https://www.youtube.com/channel/{} is saved under the nickname \"{}\"", entry.1, entry.0);
   }

   Ok(())
}

fn verify_paths(paths: Vec<&PathBuf>, files: Vec<&PathBuf>) -> Result<(), ReasonForFail>
{
    // Create all the paths
    for current_path in paths.iter()
    {
        match std::fs::create_dir_all(&current_path)
        {
            Err(_) => { return Err(ReasonForFail::DirectoryCreationFailure); },
            Ok(_) => {  }
        }
    }

    // Create all the files
    for current_file in files.iter()
    {
        // Check if the file exists
        let exists = current_file.as_path().exists();
        if exists { continue; }
        else
        {
            // Create the file
            match std::fs::File::create(current_file.as_path())
            {
                Err(_) => { return Err(ReasonForFail::FileWritingFailure); },
                Ok(_) => {  }
            }
        }
    }

    Ok(())
}

fn main() 
{
    // Declare some paths we'll need
    let mut pref_path = home::home_dir().expect("Failed to find user's home directory!");
    pref_path.push(".config");
    pref_path.push("youtube-archive");
    let mut pref_file = pref_path.clone();
    pref_file.push("queue");
    let mut dl_path = home::home_dir().expect("Failed to find user's home directory!");
    dl_path.push("videos");
    dl_path.push("youtube-archive");

    verify_paths(vec![&pref_path, &dl_path], vec![&pref_file]).unwrap();

    // First collect all the cli args into a vector
    let cli_args: Vec<String> = std::env::args().skip(1).collect();

    // Check for debug mode first
    let debug_flag = String::from("--debug");
    let debug_enabled = if cli_args.contains(&debug_flag) { println! ("Debug mode enabled"); true } else { false };
    
    // Check if the user wants to start the program
    let start_flags = (String::from("--start"),String::from("-s"));
    let start_scan = if cli_args.contains(&start_flags.0) || cli_args.contains(&start_flags.1) { true } else { false };

    // Go through each argument
    for i in 0..cli_args.len()
    {
        if debug_enabled { println! ("Parsing arg {}: {}", i, cli_args[i]); }
        match cli_args[i].as_str()
        {
            "-h" | "--help" => { println! ("Help message will go here"); },
            "-l" | "--list" =>
            {
                // Call the list function and check for errors
                if debug_enabled { println! ("Listing entries"); }
                match list_queue(&pref_file)
                {
                    Ok(_) => {  },
                    Err(ReasonForFail::FileNotFound) => {  },
                    Err(e) => { eprintln! ("Something really bad happened: {:?}", e); std::process::exit(1); }
                }
            }
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
                match add_to_queue(add_name, &add_id, &mut pref_file)
                {
                    Ok(_) => { if debug_enabled { println! ("Added channel id {} with nickname {}", add_id, add_name); } },
                    Err(ReasonForFail::DirectoryCreationFailure) => { eprintln! ("Could not create preference directory! Do you have permission?"); std::process::exit(1); },
                    Err(ReasonForFail::FileWritingFailure) => { eprintln! ("Could not write queue file! Do you have permission?"); std::process::exit(1); },
                    Err(e) => { eprintln! ("Something really bad happened: {:?}", e); std::process::exit(1); }
                }
            }
            "-r" | "--remove" =>
            {
                // Get the name they want to add it as
                let attempted_remove_arg = cli_args.get(i+1);

                // Make sure it's valid
                if let None = attempted_remove_arg { eprintln! ("You must specify a valid removal arg!"); std::process::exit(1); }
                let remove_arg = attempted_remove_arg.unwrap();
                if remove_arg.len() == 0 { eprintln! ("You must specify a valid removal arg!"); std::process::exit(1); }

                if debug_enabled { println! ("Attempting to remove {} from queue", remove_arg); }
                match remove_from_queue(&remove_arg, &mut pref_file)
                {
                    Ok(_) => { println! ("Successfully removed {} from the queue", remove_arg); },
                    Err(ReasonForFail::FileWritingFailure) => { eprintln! ("Failed to write to queue file! Do you have permission?"); std::process::exit(1); },
                    Err(ReasonForFail::FileNotFound) | Err(ReasonForFail::DirectoryCreationFailure) => { eprintln! ("Could not find queue file! Do you have read permissions?"); std::process::exit(1); },
                    Err(ReasonForFail::EntryNotFound) => { eprintln! ("Could not find {} in the queue! Doing nothing.", remove_arg); }
                }
            }
            _ => {  }
        }
    }
    
    if start_scan
    {
        // We want to start the program
        let rt = tokio::runtime::Runtime::new().unwrap();
        let start_fn = start(pref_file, dl_path, 5000, debug_enabled);
        rt.block_on(start_fn);
    }

}

// All start/live functions will go below here

async fn start(queue_file: PathBuf, download_path: PathBuf, delay_time: u64, debug_mode: bool)
{
    if debug_mode { println! ("Entering async start function"); }

    // Create the vector to keep track of what's running
    let active_archives: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    // Start the periodic loop
    loop
    {
        println! ("Checking channels...");

        // Get the entries for the file
        let entries = match file_to_tup_vec(":", &queue_file)
        {
            Ok(vec) => vec,
            Err(ReasonForFail::FileNotFound) => { eprintln! ("Could not find queue file!"); std::process::exit(1); },
            Err(e) => { eprintln! ("Something really bad happened: {:?}", e); std::process::exit(1); }
        };

        // Iterate through the entries
        for current_entry in entries.iter()
        {
            println! ("Checking if {} is live...", current_entry.0);
            let video_id = match std::process::Command::new("./parse_youtube_data.py").arg(current_entry.1.as_str()).output()
            {
                Ok(vid_id) => if std::str::from_utf8(&vid_id.stdout).unwrap() == "no_live\n" { println! ("{} is not live.", current_entry.0); continue; } else { std::str::from_utf8(&vid_id.stdout).unwrap().to_string().replace("\n", "") }
                Err(_) => { eprintln! ("Failed to check entry for {}!", current_entry.0); std::process::exit(1); }
            };
            
            // Check to see if the user's stream is already being archived
            if let Ok(active_vec) = active_archives.lock() { if active_vec.contains(&video_id) { println! ("{}'s stream is already being archived.", current_entry.0); continue; } }

            tokio::task::spawn_blocking({
                let vid_id = video_id.clone();
                let save_name = String::from(&current_entry.0);
                let dl_path = download_path.clone();
                let activity_vec_to_pass = Arc::clone(&active_archives);
                let debug_to_pass = debug_mode;

                move ||archive_from_id(vid_id, save_name, dl_path, activity_vec_to_pass, debug_to_pass, )
            });
        }
        sleep(Duration::from_millis(delay_time)).await;
    }
}

fn archive_from_id(video_id: String, name: String, download_path: PathBuf, active_archive_vec: Arc<Mutex<Vec<String>>>, debug_mode: bool)
{
    println! ("Starting archive for {}...", name);
    
    // Tell everyone we're archiving the stream
    loop
    {
        if let Ok(mut active_vec) = active_archive_vec.lock() { active_vec.push(video_id.clone()); break; }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }

    // Do the youtube-dl and ffmpeg thing
    std::process::Command::new("youtube-dl").arg("-f").arg("best").arg("--no-continue").arg("-o").arg(format! ("{}/{}", download_path.as_path().display(), video_id)).arg(format! ("https://www.youtube.com/watch?v={}", video_id)).output();
    println! ("youtube-dl -f best -g https://www.youtube.com/watch?v={}` -c copy {}/YY-MM-DD-{}.mp4", video_id, download_path.display(), name);

    println! ("{}'s stream has ended.", name);

    // Remove their entry from the active queue
    loop
    {
        if let Ok(mut active_vec) = active_archive_vec.lock() 
        {
            for i in 0..active_vec.len()
            {
                if active_vec[i] == video_id { active_vec.remove(i); }
            }
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

