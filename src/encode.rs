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
use std::fs::create_dir;
use std::path::Path;
use std::process::Command;

fn no_copy_replace(string: &mut Cow<'_, str>, pattern: &str, replace: &str) {
    if string.contains(pattern) {
        *string = string.replace(pattern, replace).into();
    }
}

fn replace_in_vec(data: &mut Vec<Cow<'_, str>>, pattern: &str, replace: &str) {
    for i in data.iter_mut() {
        no_copy_replace(i, pattern, replace);
    }
}

fn create_name(filename: &str) -> Cow<'_, str> {
    let path = Path::new(filename);
    let mut name: String = String::from("./encoded/");
    name.push_str(path.file_stem().unwrap().to_str().unwrap());
    name.push_str(".mkv");
    name.into()
}

fn create_encoded() {
    if !(Path::new("./encoded").is_dir()) {
        create_dir("./encoded").unwrap();
    }
}

fn slice_of_str_to_vec_of_cow<'a>(input: &'a [&str]) -> Vec<Cow<'a, str>> {
    input.iter().map(|x| (*x).into()).collect::<Vec<_>>()
}

pub fn encode(files: Vec<String>, cmd_args: &[&str]) {
    create_encoded();

    for i in files {
        let mut args = slice_of_str_to_vec_of_cow(cmd_args);
        replace_in_vec(&mut args, "{}", &i);
        println!("{}", i);
        args.push(create_name(&i));
        println!("Ffmpeg command: ffmpeg {}", args.join(" "));

        let _command = Command::new("/usr/bin/ffmpeg")
            .args(args.iter().map(Cow::as_ref))
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .output()
            .expect("Failed to execute ffmpeg");
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use super::{no_copy_replace, replace_in_vec};

    #[test]
    fn test_replace() {
        let mut test_string: Cow<'_, str> = "{}".to_string().into();
        no_copy_replace(&mut test_string, "{}", "replaced");

        assert_eq!(test_string, "replaced");
    }

    #[test]
    fn test_replace_slice() {
        let mut test_slice: Vec<Cow<'_, str>> = ["other_string", "{}", "other_string"].iter().map(|x| (*x).into()).collect();

        replace_in_vec(&mut test_slice, "{}", "replaced");
        
        assert_eq!(test_slice[1], "replaced");
    }
}