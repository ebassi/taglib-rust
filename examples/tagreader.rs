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
        println!("title   - {}", t.title().unwrap_or_default());
        println!("artist  - {}", t.artist().unwrap_or_default());
        println!("album   - {}", t.album().unwrap_or_default());
        println!("year    - {}", t.year().unwrap_or_default());
        println!("comment - {}", t.comment().unwrap_or_default());
        println!("track   - {}", t.track().unwrap_or_default());
        println!("genre   - {}", t.genre().unwrap_or_default());
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
