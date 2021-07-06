extern crate clap;
use clap::App;

fn main() {
  App::new("Beautiful UFO").version("v1.0-beta").get_matches();
}
