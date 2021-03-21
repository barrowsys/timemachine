# timemachine

[![GitHub last commit](https://img.shields.io/github/last-commit/barrowsys/timemachine)](https://github.com/barrowsys/timemachine)
[![Crates.io](https://img.shields.io/crates/v/timemachine)](https://crates.io/crates/timemachine/)
[![Docs.rs](https://docs.rs/timemachine/badge.svg)](https://docs.rs/timemachine)

100% WIP and alpha, use at your own risk

feature "napchart" (used by two examples) uses the https://napchart.com API which is in alpha

# Usage

add dependency to your cargo.toml:
```
timemachine = "0.2.0-alpha"
```
to enable napchart support:
```
timemachine = [ version = "0.2.0-alpha", features = ["napchart"] ]
```

# Examples
- cargo run --example auto_lights  
    takes a configuration and simulates 24 hours of light colors
- cargo run --example napchart_lights --features napchart  
    similar to auto_lights, but gets a config from https://napchart.com/3tbkt  
    grey = lights off, red = lights red, otherwise lights white
- cargo run --example napchart_lights_option --features napchart  
    similar to napchart_lights, but gets its config from https://napchart.com/cse2j  
    grey = lights off, red = lights red, blue = lights white, otherwise error
