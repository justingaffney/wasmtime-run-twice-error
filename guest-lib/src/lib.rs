use bindings::exports::wasi::cli::run::Guest;

#[allow(warnings)]
mod bindings;

struct Component;

impl Guest for Component {
    fn run() -> Result<(), ()> {
        println!("Hello, world! [from guest-lib]");
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);