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

use std::fs::create_dir;
use std::path::{Path, PathBuf};
use std::process::Command;

fn create_name(original_path: impl AsRef<Path>, format: &str) -> PathBuf {
    let mut path = PathBuf::from("./encoded/");
    let mut filename = original_path.as_ref().file_name().unwrap().to_owned();
    filename.push(".");
    let extension = get_format_extension(format);
    filename.push(extension);
    path.push(filename);
    path
}

fn create_encoded() {
    if !(Path::new("./encoded").is_dir()) {
        create_dir("./encoded").unwrap();
    }
}

fn get_format_extension(format: &str) -> &str {
    match format {
        "matroska" => "mkv",
        f => f,
    }
}

pub fn encode(files: &[impl AsRef<Path>], cmd_args: &[String], format: &str) {
    create_encoded();

    for i in files {
        let output = create_name(&i, format);

        let mut command = Command::new("/usr/bin/ffmpeg");

        command
            .arg("-y")
            .arg("-i")
            .arg(i.as_ref())
            .args(cmd_args)
            .arg("-f")
            .arg(format)
            .arg(output);

        println!(
            "Ffmpeg command: {} {}",
            command.get_program().to_string_lossy(),
            command.get_args().map(|s| s.to_string_lossy()).collect::<Vec<_>>().join(" "),
        );

        command
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .output()
            .expect("Failed to execute ffmpeg");
    }
}
