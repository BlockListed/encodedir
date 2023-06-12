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

/*
    TODO:
    *
*/

use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

pub fn get_files(p: impl AsRef<Path>, filetypes: &Vec<String>) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = vec![];
    let dirdata: Vec<_> = read_dir(p).unwrap().collect();

    for i in dirdata {
        let filepath = i.unwrap().path();
        if filepath.is_dir() {
            paths.append(&mut get_files(filepath, filetypes));
            continue;
        }
        let extension: String = match filepath.extension() {
            Some(v) => v.to_str().unwrap().to_string(),
            None => continue,
        };

        if filetypes.iter().any(|v| v == &extension) {
            paths.push(filepath);
        }
        continue;
    }

    paths
}
