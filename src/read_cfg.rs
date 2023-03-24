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

use std::fs::File;
use std::io::prelude::*;
use std::path;

extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub command_args: Vec<String>,
    pub ftypes: Vec<String>,
}

// Function to create a new config
fn create_config(p: &str) {
    let mut f = File::create(p).unwrap();
    let data = toml::toml! {
        command_args = [
            "-y",
            "-i",
            "{}",
            "-c:v",
            "hevc_nvenc",
            "-rc",
            "constqp",
            "-qmin",
            "10",
            "-qmax",
            "35",
            "-f",
            "mp4",
        ]

        ftypes = [
            "mp4",
            "mkv",
            "mov",
        ]
    };
    let tomldata = toml::to_string_pretty(&data).unwrap();
    writeln!(f, "{}", tomldata.as_str()).unwrap();
}

fn get_config_path() -> String {
    let mut config_path = std::env::var("HOME").unwrap();
    config_path.push_str("/.config/encodedir.toml");
    config_path
}

pub fn get_config() -> Config {
    let config_path = get_config_path();
    if !(path::Path::new(&config_path).exists()) {
        create_config(&config_path);
    }

    // This mess just parses a toml file
    let conf: Config = toml::from_str(&std::fs::read_to_string(config_path).unwrap())
        .expect("Couldn't parse config from");
    conf
}
