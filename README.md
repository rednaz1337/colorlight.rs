# Colorlight.rs

This is a simple Project written in Rust to mirror the computer screen to a Colorlight 5A-75B running
Niklas Fauth's firmware: https://github.com/NiklasFauth/colorlight-led-cube
There are Python scripts in that repo, which do the same thing, but this rust version is *much* more performant.
The program can be configured either with a hard-coded JSON string in `main.rs` or it can be passed as the first
parameter. 