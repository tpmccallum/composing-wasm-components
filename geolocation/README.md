# Composing Components

The WebAssembly Component Model facilitates seamless interoperability between components across different programming languages by leveraging a standardized ABI (Application Binary Interface) and WIT-defined interfaces. Unlike conventional foreign function interfaces (FFIs), which rely on language-specific bindings, the Component Model provides a universal mechanism for linking components through compatible imports and exports. Composition involves merging a primary component with its dependencies, resulting in a new .wasm file with a structured interface. This process restricts exports to the primary component while correctly resolving imports from both the primary and dependent components.


## New Project

```bash
mkdir geolocation
cd geolocation
cargo install cargo-component
cargo install wac-cli
echo "Versions"
cargo component --version
wac --version
```

```bash
cargo-component-component 0.21.1
wac-cli 0.6.1
```

```bash
mkdir wit
cd wit
mkdir coordinate_normalizer
mkdir dms_converter
mkdir coordinate_display
```

Create `coordinate_normalizer/world.wit` and populate:

```wit
package geoloc:coord-norm@0.1.0;

interface coord-norm {
  record Coordinates {
    latitude: float64,
    longitude: float64,
  }

  record NormalizedCoordinates {
    latitude: float64,
    longitude: float64,
    valid: bool,
  }

  import normalize: func(coords: Coordinates) -> NormalizedCoordinates;
}

world coordinate-normalizer {
    export coord-norm;
}
```

Create `dms_converter/world.wit` and populate:

```wit
package geoloc:dms-conv@0.1.0;

interface dms-conv {
  record DMS {
    degrees: i32,
    minutes: i32,
    seconds: float64,
    direction: string,
  }

  record DMSCoordinates {
    latitude: DMS,
    longitude: DMS,
  }

  import convert: func(normalized: geoloc:coord-norm/coord-norm.NormalizedCoordinates) -> DMSCoordinates;
}

world dms-converter {
    export dms-conv;
    import geoloc:coord-norm/coord-norm;
}
```

Create `coordinate_display/world.wit` and populate:

```wit
package geoloc:coord-disp@0.1.0;

interface coord-disp {
  import display: func(dms: geoloc:dms-conv/dms-conv.DMSCoordinates);
}

world coordinate-display {
    import geoloc:dms-conv/dms-conv;
}
```

Create the `app/world.wit` and populate:

```wit
package geoloc:app@0.1.0;

world app {
    import geoloc:coord-disp/coord-disp;
}
```

Now, we can create the Rust projects for each component (and a new app component):

```bash
cargo component new coordinate_normalizer --lib --target wit/coordinate_normalizer/world.wit
cargo component new dms_converter --lib --target wit/dms_converter/world.wit
cargo component new coordinate_display --lib --target wit/coordinate_display/world.wit
cargo component new app --bin --target wit/app/world.wit
```

Now open the `Cargo.toml` file and add the following `package.metadata.component.target.dependencies` section:

```toml
[package.metadata.component.target.dependencies]
"geoloc:dms-conv" = { path = "../dms_converter" }
"geoloc:coord-norm" = { path = "../coordinate_normalizer" }
```

```bash
cargo component new command --command
```



