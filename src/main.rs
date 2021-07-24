mod read_cfg;
mod get_files;
mod encode;

fn main() {
    /*
        TODO:
        * Get input for path of files (or preferably do it as an argument)
            * Validate and check if path exists
        * Get directory to move encoded files to
        * Read the config
        * Use output from config and get_files functions to run encode on all get_files
    */

    let help = "
Usage:
    encodedir [Path to directory]
    
    Additional configuration can be done in ~/.config/encodedir.toml! (Generated after first run))\n";

    let config = read_cfg::get_config();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        print!("Please give 2 arguments \n{}", help);
        return;
    }

    let patharg = args[1].as_str();

    let path = std::path::Path::new(patharg);
    if ! (path.exists()) {
        print!("Path doesn't exist \n{}", help);
        return;
    }

    

    let files = get_files::get_files(patharg, &config.ftypes);

    encode::encode(files, config.command_args.as_str());
}
