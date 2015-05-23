extern crate taglib;

use std::env;

pub fn main() {
  let args: Vec<String> = env::args().collect();

  for i in 1..args.len() {
    let ref arg = args[i];
    let file =
      match taglib::File::new(arg) {
        Ok(f) => f,
        Err(e) => {
          println!("Invalid file {} (error: {:?})", arg, e);
          continue;
        }
      };

    println!("*** \"{}\" ***", arg);

    match file.tag() {
      Ok(t) => {
        println!("-- TAG --");
        println!("title   - {}", t.title());
        println!("artist  - {}", t.artist());
        println!("album   - {}", t.album());
        println!("year    - {}", t.year());
        println!("comment - {}", t.comment());
        println!("track   - {}", t.track());
        println!("genre   - {}", t.genre());
      },
      Err(e) => {
        println!("No available tags for {} (error: {:?})", arg, e);
      }
    }

    match file.audioproperties() {
      Ok(p) => {
        let secs = p.length() % 60;
        let mins = (p.length() - secs) / 60;

        println!("-- AUDIO --");
        println!("bitrate     - {}", p.bitrate());
        println!("sample rate - {}", p.samplerate());
        println!("channels    - {}", p.channels());
        println!("length      - {}m:{}s", mins, secs);
      },
      Err(e) => {
        println!("No available audio properties for {} (error: {:?})", arg, e);
      }
    }
  }
}
