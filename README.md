# LINUX ONLY

# Encodedir
Encode a directory using ffmpeg.

# Install
* Make sure ffmpeg is installed on your system.
* Download latest release from the releases tab.
* Unpack the archive using `tar -xf [downloaded archive goes here]`.
* Run `sh ./install.sh` from a terminal in the extracted folder.
* If your shell can't find it, that probably means the default binary location (~/.local/bin) isn't in your path.
 * If this is the case please either add the location to your path or run with `~/.local/bin/encodedir`.
## Notes
* The default config use hevc_nvenc as the encode. If your system doesn't have an nvidia gpu with nvenc, edit the config file to use an encode like hevc_vaapi or just the cpu encoder (libx265).

# Install from source
* Install a rust toolchain for your system. (Only linux is tested and I probably won't add config locations that work with windows for a while).
* Compile with `cargo build --release`.
  * (Optional) strip binary with `strip target/release/encodedir`.
* Move file to a location in your path (Ex. /usr/bin or $HOME/.local/bin (AKA ~/.local/bin)).
* Use with `encodedir [Path to directory]`.

# Usage
* Usage information is available using `encodedir --help`

# Configuration
* The config file is location at `$HOME/.config/encodedir.toml` (AKA `~/.config/encodedir.toml`).
  * Configure command args (Like switching from nvidia nvenc to vaapi or cpu encoding).
  * Configure what files encodedir sees as video files, incase you don't have your files in an MKV, MP4 or MOV container.
