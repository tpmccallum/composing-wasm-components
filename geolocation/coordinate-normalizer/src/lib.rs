mod bindings;

use bindings::exports::docs::normalizer::normalize::Guest;
use bindings::exports::docs::normalizer::normalize::{Coordinates, NormalizedCoordinates};

struct Component;

impl Guest for Component {
    fn normalize(coords: Coordinates) -> NormalizedCoordinates {
        let mut lat = coords.latitude;
        let mut lon = coords.longitude;
        let mut valid = true;

        if lat < -90.0 { lat = -90.0; valid = false; }
        else if lat > 90.0 { lat = 90.0; valid = false; }

        if lon < -180.0 { lon = -180.0; valid = false; }
        else if lon > 180.0 { lon = 180.0; valid = false; }

        NormalizedCoordinates { latitude: lat, longitude: lon, valid }
    }
}

bindings::export!(Component with_types_in bindings);
