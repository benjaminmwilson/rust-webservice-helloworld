#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, MediaType, Options};

fn main() {
    let mut server = Nickel::new();

    Nickel::keep_alive_timeout( & mut server, None);

    /* server.options = Options::default()
        .thread_count(Some(300)); */

    server.get("**", middleware!("{ \"message\": \"Hello World\"}"));
    /* server.get("**", middleware! { |_, mut res|
        res.set(MediaType::Json);

        "{ \"message\": \"Hello World\"}"
    }); */
    server.listen("0.0.0.0:9001").unwrap();
}