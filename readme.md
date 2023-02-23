# Simple Rust Weather Application
I wanted to learn a little rust so I created a quick application that gives you the current weather at your location in a simple digestable string.

## Building
`cargo build`

This will produce a binary in ./debug/target/ named `r_weather`.

## Running
`r_weather [YOUR_API_KEY]`

## Notes
I made a simple function in fish named `weather` to call this function with my API key. Bash and Zsh should be similar
`alias weather="r_weather [YOUR_API_KEY]"; funcsave weather`
