
fn main() {
    // First collect all the cli args into a vector
    let cli_args: Vec<String> = std::env::args().skip(1).collect();

    // Check for debug mode first
    let debug_flag = String::from("--debug");
    let DEBUG = if cli_args.contains(&debug_flag) { println! ("Debug mode enabled"); true } else { false };

    // Go through each argument
    for i in 0..cli_args.len()
    {
    }
}
