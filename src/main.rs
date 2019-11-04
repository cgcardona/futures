extern crate tokio;
#[macro_use]
extern crate futures;

mod display;
mod hello_world;

use display::Display;
use hello_world::HelloWorld;

fn main() {
    let future: Display<HelloWorld> = Display(HelloWorld);
    tokio::run(future);
}
