mod bindings;

use bindings::docs::normalizer::normalize::NormalizedCoordinates;
use bindings::exports::docs::converter::convert::{Guest, Dms, DmsCoordinates};

struct Component;

impl Guest for Component {
    fn convert_to_dms(normalized: NormalizedCoordinates) -> DmsCoordinates {
        fn to_dms(decimal: f64, is_lat: bool) -> Dms {
            let abs = decimal.abs();
            let degrees = abs.floor() as u32;
            let minutes = ((abs - degrees as f64) * 60.0).floor() as u32;
            let seconds = (abs - degrees as f64 - minutes as f64 / 60.0) * 3600.0;
            let direction = if decimal >= 0.0 {
                if is_lat { "N".to_string() } else { "E".to_string() }
            } else {
                if is_lat { "S".to_string() } else { "W".to_string() }
            };
            Dms { degrees, minutes, seconds, direction }
        }
        DmsCoordinates {
            latitude: to_dms(normalized.latitude, true),
            longitude: to_dms(normalized.longitude, false),
        }
    }
}

bindings::export!(Component with_types_in bindings);
