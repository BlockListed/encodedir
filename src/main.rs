use std::{io::Write, path::PathBuf};

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
mod encode;
mod get_files;
mod read_cfg;
mod statements;

fn write_str(data: &str) {
    std::io::stderr().write_all(data.as_bytes()).unwrap();
    println!();
}

fn print_help() {
    write_str(statements::HELP);
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let config = read_cfg::get_config();

    let mut args = pico_args::Arguments::from_env();

    if args.contains("--help") {
        print_help();
        return Ok(());
    }

    if args.contains("--warranty") {
        write_str(statements::WARRANTY);
        return Ok(());
    }

    if args.contains("--distribute") {
        write_str(statements::DISTRIBUTON);
        return Ok(());
    }

    let Ok(path) = args.free_from_str::<PathBuf>() else {
        print_help();
        return Err(color_eyre::Report::msg("Path argument missing!"))
    };

    if !(path.exists()) {
        print_help();
        return Err(color_eyre::Report::msg("Path doesn't exist!"));
    }

    let files = get_files::get_files(&path, &config.ftypes);

    encode::encode(
        &files,
        &config.command_args,
        &config.format
    );

    Ok(())
}
