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

use std::path::Path;
use std::fs::create_dir;
use std::process::Command;

pub fn encode(files: Vec<String>, cmd_args: &str) {
    /*
        TODO:
        *
    */
    if ! (Path::new("./encoded").is_dir()) {
        create_dir("./encoded").unwrap();
    }

    for i in files.iter() {
        let path = Path::new(i.as_str());
        let mut name: String = String::from("./encoded/");
        name.push_str(path.file_stem().unwrap().to_str().unwrap());
        name.push_str(".mp4");
        let mut ffmpeg_cmd = cmd_args.replace("{}", i.as_str());
        ffmpeg_cmd.push_str(name.as_str());
        println!("Ffmpeg command: ffmpeg {}", ffmpeg_cmd);

        let _command = Command::new("/usr/bin/ffmpeg")
            .args(ffmpeg_cmd.as_str().split(" "))
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .output()
            .expect("Failed to execute ffmpeg");
    }
}