# timemachine

100% WIP and alpha, use at your own risk

feature "napchart" (used by two examples) uses the https://napchart.com API which is in closed alpha

# Examples
- cargo run --example auto_lights  
    takes a configuration and simulates 24 hours of light colors
- cargo run --example napchart_lights --features napchart  
    similar to auto_lights, but gets a config from https://napchart.com/3tbkt  
    grey = lights off, red = lights red, otherwise lights white
- cargo run --example napchart_lights_option --features napchart  
    similar to napchart_lights, but gets its config from https://napchart.com/cse2j  
    grey = lights off, red = lights red, blue = lights white, otherwise error
