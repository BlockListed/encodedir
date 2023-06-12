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

use std::borrow::Cow;
use std::ffi::{OsStr, OsString};
use std::fs::create_dir;
use std::path::{Path, PathBuf};
use std::process::Command;

fn replace_single_in_vec(data: &mut [Cow<OsStr>], pattern: &str, replace: &str) {
    data
        .iter_mut()
        // Skip allocation, if it's not needed
        .filter(|v| {
            *v == OsStr::new("{}")
        })
        .for_each(|v| {
            let s = v.to_str().unwrap();
            let os_string: OsString = s.replace(pattern, replace).into();
            *v = os_string.into();
        });
}

fn create_name(filename: impl AsRef<Path>) -> PathBuf {
    let path = filename.as_ref();
    let mut name = PathBuf::from("./encoded/");
    let mut filename = OsString::from(path.file_name().unwrap());
    filename.push(".mkv");
    name.push(filename);
    name
}

fn create_encoded() {
    if !(Path::new("./encoded").is_dir()) {
        create_dir("./encoded").unwrap();
    }
}

fn slice_of_str_to_vec_of_cow<'a>(input: &'a [&str]) -> Vec<Cow<'a, OsStr>> {
    input
            .iter()
            .map(|v| Cow::Borrowed(OsStr::new(*v)))
            .collect()
}

pub fn encode(files: Vec<impl AsRef<Path>>, cmd_args: &[&str]) {
    create_encoded();

    for i in files {
        let mut args: Vec<Cow<OsStr>> = slice_of_str_to_vec_of_cow(cmd_args);
        replace_single_in_vec(&mut args, "{}", i.as_ref().to_str().unwrap());
        args.push(create_name(&i).into_os_string().into());
        println!(
            "Ffmpeg command: ffmpeg {}",
            args.join(OsStr::new(" ")).to_str().unwrap()
        );

        let _command = Command::new("/usr/bin/ffmpeg")
            .args(args)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .output()
            .expect("Failed to execute ffmpeg");
    }
}