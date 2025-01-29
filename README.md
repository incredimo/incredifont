# Incredimo

Incredimo is a Rust library for creating beautiful terminal banners.

## Features

- Create ASCII art banners with customizable colors and styles using pure ANSI escape codes.
- Supports gradient and rainbow color effects.
- Allows adding subtitles to banners.
- Configurable line length for banners.

## Example

```rust
use incredimo::Banner;

fn main() {
    let banner = Banner::new("INCREDIBLE")
        .with_colors()
        .with_subtitle("IMPOSSIBLE IS JUST A CHALLENGE YET TO BE SOLVED")
        .with_line_length(80)
        .build()
        .unwrap();
}
```
