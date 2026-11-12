extern crate zap;

use zap::App;

fn main() {
    match App::run() {
        Ok(_) => {},
        Err(e) => println!("{e}"),
    }
}
