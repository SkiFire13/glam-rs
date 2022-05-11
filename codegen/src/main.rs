use tera::{Context, Tera};
use std::{process, io};

fn main() {
    let tera = match Tera::new("templates/**/*.rs") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            process::exit(1);
        }
    };

    let context = Context::new();
    tera.render_to("swizzles/traits.rs", &context, &mut io::stdout().lock()).unwrap();
}
