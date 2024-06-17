use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use std::thread::sleep;
use rodio::{Decoder, OutputStream, source::Source};

fn main() {
    let (_steam, steam_handle) = OutputStream::try_default().unwrap();

    loop {
        let buf = BufReader::new(File::open("audio/mr-robot-whiterose-beep.flac").unwrap());
        let decode = Decoder::new(buf).unwrap();
        steam_handle.play_raw(decode.convert_samples()).unwrap();
        sleep(Duration::from_secs(60));
    }
}
