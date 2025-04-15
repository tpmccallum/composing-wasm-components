#[allow(warnings)]
mod bindings;

use bindings::exports::docs::string_processor::processor::Guest;

struct Component;

impl Guest for Component {
    fn reverse(input: String) -> String {
        input.chars().rev().collect()
    }
}

bindings::export!(Component with_types_in bindings);