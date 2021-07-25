/*
/Encodedir: Encode all video files in a directory using the systems installed ffmpeg
/Copyright (C) 2021  BlockListed
/
/This program is free software: you can redistribute it and/or modify
/it under the terms of the GNU General Public License as published by
/the Free Software Foundation, either version 3 of the License, or
/(at your option) any later version.
/
/This program is distributed in the hope that it will be useful,
/but WITHOUT ANY WARRANTY; without even the implied warranty of
/MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/GNU General Public License for more details.
/
/You should have received a copy of the GNU General Public License
/along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
mod read_cfg;
mod get_files;
mod encode;

/*
    TODO:
    * 
*/

fn main() {
    /*
        TODO:
        * GPL stuff: https://www.gnu.org/licenses/gpl-3.0.html#howto
        * Add GPL Warning:
<program>  Copyright (C) <year>  <name of author>
This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.
This is free software, and you are welcome to redistribute it
under certain conditions; type `show c' for details.
        * Add command to show appropriate parts of GPL v3
    */

    let help = " 
Encodedir  Copyright (C) 2021  BlockListed
This program comes with ABSOLUTELY NO WARRANTY; for details type `encodedir --warranty'.
This is free software, and you are welcome to redistribute it
under certain conditions; type `encodedir --distribute' for details.

Usage:
    encodedir [Path to directory] - Encode directory
    
    Additional configuration can be done in ~/.config/encodedir.toml! (Generated after first run))\n";
    let gplwarranty = "This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.";
    let gpldistribute = "This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.";

    let config = read_cfg::get_config();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        print!("Please give 1 argument\n{}", help);
        return;
    }

    let patharg = args[1].as_str();

    if patharg == "--help" {
        println!("{}", help);
        return;
    } else if patharg == "--warranty" {
        println!("{}", gplwarranty);
        return;
    } else if patharg == "--distribute" {
        println!("{}", gpldistribute);
        return;
    }

    let path = std::path::Path::new(patharg);
    if ! (path.exists()) {
        print!("Path doesn't exist \n{}", help);
        return;
    }

    

    let files = get_files::get_files(patharg, &config.ftypes);

    encode::encode(files, config.command_args.as_str());
}
