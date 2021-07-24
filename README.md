# TODO
* Setup meson (ETA 1 Week)
* Create package, so you don't have to compile everything yourself (ETA 1 Week)

# Encodedir
Encode a directory using ffmpeg)

# Install
* Install a rust toolchain for your system. (Only linux is tested and I probably won't add config locations that work with windows for a while).
* Compile with `cargo build --release`
  * (Optional) strip binary with `strip target/release/encodedir`
* Move file to a location in your path (Ex. /usr/bin or $HOME/.local/bin (AKA ~/.local/bin))
* Use with encodedir [Path to directory]

# Configuration
* The config file is location at $HOME/.config/encodedir.toml (AKA ~/.config/encodedir.toml).
  * Configure command args (Like switching from nvidia nvenc to vaapi or cpu encoding)
  * Configure what files encodedir sees as video files, incase you don't have your files in a mkv, mp4 or mov container.
