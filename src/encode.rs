use std::path::Path;
use std::fs::create_dir;
use std::process::Command;

pub fn encode(files: Vec<String>, cmd_args: &str) {
    /*
        TODO:
        * Create new directory for files
        * Encode all files to new directory
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

        let argsiter = ffmpeg_cmd.as_str().split(" ");

        let _command = Command::new("/usr/bin/ffmpeg")
            .args(ffmpeg_cmd.as_str().split(" "))
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .output()
            .expect("Failed to execute ffmpeg");
    }
}