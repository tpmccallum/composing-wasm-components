#[allow(warnings)]
mod bindings;

use bindings::exports::docs::orchestrator::greeting::Guest;
use bindings::docs::string_processor::processor::reverse;

struct Component;

impl Guest for Component {
    fn create_greeting(name: String) -> String {
        let reversed = reverse(&name);
        format!("Hello, {}! Your name backwards is {}.", name, reversed)
    }
}

bindings::export!(Component with_types_in bindings);