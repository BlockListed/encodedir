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

use std::fs::read_dir;

pub fn get_files(p: &str, filetypes: &Vec<String>) -> Vec<String> {
    let mut paths: Vec<String> = vec![];
    let dirdata: Vec<Result<std::fs::DirEntry, std::io::Error>> = read_dir(p).unwrap().collect();

    for i in dirdata {
        let tmp = i.unwrap().path();
        let fi = tmp.as_path();
        let strpath: String = String::from(fi.to_str().unwrap());
        if fi.is_dir() {
            paths.append(&mut get_files(strpath.as_str(), filetypes));
            continue;
        }
        let extension: String =
            String::from(fi.extension().unwrap().to_str().unwrap());
        if extension.is_empty() {
            continue;
        }

        if filetypes.iter().any(|v| v == &extension) {
            paths.push(strpath);
        }
        continue;
    }

    paths
}
