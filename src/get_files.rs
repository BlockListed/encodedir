use std::fs::read_dir;

pub fn get_files(p: &str, filetypes: &Vec<String>) -> Vec<String> {
    /*
        TODO:
        -* Get files from directory
            -* If file is directory, recursively search directory
        -* Return list of file paths
    */
    let mut paths: Vec<String> = vec![];
    let dirdata: Vec<Result<std::fs::DirEntry, std::io::Error>> = read_dir(p).unwrap().collect();

    for i in dirdata.iter() {
        let fi: &std::fs::DirEntry = i.as_ref().unwrap();
        let strpath: String = String::from(fi.path().as_path().to_str().unwrap());
        if fi.path().as_path().is_dir() {
            paths.append(&mut get_files(strpath.as_str(), &filetypes));
            continue;
        }
        let extension: String = String::from(fi.path().as_path().extension().unwrap().to_str().unwrap());
        if extension == "" {
            continue;
        }

        if filetypes.iter().any(|v| v==&extension) {
            let mut path: Vec<String> = vec![strpath];
            paths.append(&mut path);
            continue;
        } else {
            continue;
        }
    }
    
    return paths;
}