//! Incredifont - A Rust library for creating beautiful terminal banners
//! 
///! ██ ██████     ████ ██████   ██████ ██████   ██ ████████   ██████
///! ██ ██    ██ ██     ██    ██ ██     ██    ██ ██ ██  ██  ██ ██    ██
///! ██ ██    ██ ██     ██████   ████   ██    ██ ██ ██  ██  ██ ██    ██
///! ██ ██    ██ ██████ ██    ██ ██████ ██████   ██ ██  ██  ██   ██████
///! ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
///! Incredifont - A Rust library for creating beautiful terminal banners
///! 
///! This library provides functionality to create ASCII art banners with
///! customizable colors and styles using pure ANSI escape codes.
///! 
///! # Example
///! ```rust
///! use incredifont::Banner;
///! 
///! fn main() {
///!     let banner = Banner::new("HELLO WORLD")
///!         .with_colors()
///!         .with_subtitle("This is a subtitle")
///!         .with_line_length(80)
///!         .unwrap();
///!     println!("{}", banner.render());
///! }
///! ```
use std::fmt;
use std::error::Error;
/// Contains the ASCII art representation for each character (4 lines height)
const FONT_MAP: &[(&str, &[&str])] = &[
    ("A", &[
        "██████   ",
        "██    ██ ",
        "████████ ",
        "██    ██ "
    ]),
    ("B", &[
        "██████   ",
        "██    ██ ",
        "██████   ",
        "████████ "
    ]),
    ("C", &[
        "  ████ ",
        "██     ",
        "██     ",
        "██████ "
    ]),
    ("D", &[
        "██████   ",
        "██    ██ ",
        "██    ██ ",
        "██████   "
    ]),
    ("E", &[
        "██████ ",
        "██     ",
        "████   ",
        "██████ "
    ]),
    ("F", &[
        "  ████ ",
        "██     ",
        "██████ ",
        "██     "
    ]),
    ("G", &[
        "  ████ ",
        "██     ",
        "██  ██ ",
        "██████ "
    ]),
    ("H", &[
        "██   ██ ",
        "██   ██ ",
        "███████ ",
        "██   ██ "
    ]),
    ("I", &[
        "██ ",
        "██ ",
        "██ ",
        "██ "
    ]),
    ("J", &[
        "    ██ ",
        "    ██ ",
        "██  ██ ",
        "██████ "
    ]),
    ("K", &[
        "██  ██ ",
        "████   ",
        "██  ██ ",
        "██  ██ "
    ]),
    ("L", &[
        "██     ",
        "██     ",
        "██     ",
        "██████ "
    ]),
    ("M", &[
        "████████   ",
        "██  ██  ██ ",
        "██  ██  ██ ",
        "██  ██  ██ "
    ]),
    ("N", &[
        "██████   ",
        "██    ██ ",
        "██    ██ ",
        "██    ██ "
    ]),
    ("O", &[
        "██████   ",
        "██    ██ ",
        "██    ██ ",
        "  ██████ "
    ]),
    ("P", &[
        "  ██████ ",
        "██    ██ ",
        "██████   ",
        "██       "
    ]),
    ("Q", &[
        "██████  ",
        "██    ██ ",
        "██  ████ ",
        "████  ██ "
    ]),
    ("R", &[
        "██████   ",
        "██    ██ ",
        "██████   ",
        "██    ██ "
    ]),
    ("S", &[
        "██████ ",
        "██     ",
        "    ██ ",
        "██████ "
    ]),
    ("T", &[
        "██████ ",
        "  ██   ",
        "  ██   ",
        "  ██   "
    ]),
    ("U", &[
        "██    ██ ",
        "██    ██ ",
        "██    ██ ",
        "  ██████ "
    ]),
    ("V", &[
        "██    ██ ",
        "██    ██ ",
        "██  ██   ",
        "████     "
    ]),
    ("W", &[
        "██  ██  ██ ",
        "██  ██  ██ ",
        "██  ██  ██ ",
        "  ████████ "
    ]),
    ("X", &[
        "██   ██ ",
        " ████   ",
        " ████   ",
        "██   ██ "
    ]),
    ("Y", &[
        "██   ██ ",
        " ████   ",
        "  ██    ",
        "  ██    "
    ]),
    ("Z", &[
        "██████  ",
        "   ██   ",
        " ██     ",
        "██████  "
    ]),
    (" ", &[
        "   ",
        "   ",
        "   ",
        "   "
    ]),
];

const BLOCK: &str = "██";

#[derive(Debug)]
pub enum IncrediError {
    InvalidConfig(String),
}

impl fmt::Display for IncrediError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IncrediError::InvalidConfig(msg) => write!(f, "Invalid banner configuration: {}", msg),
        }
    }
}

impl Error for IncrediError {}

#[derive(Debug, Clone)]
struct RgbColor {
    r: u8,
    g: u8,
    b: u8,
}

impl RgbColor {
    fn new(r: u8, g: u8, b: u8) -> Self {
        RgbColor { r, g, b }
    }

    fn to_ansi(&self) -> String {
        format!("\x1b[38;2;{};{};{}m██\x1b[0m", self.r, self.g, self.b)
    }
}

const GRADIENT_COLORS: &[(u8, u8, u8)] = &[
    (230, 230, 230),  // Dimmest white
    (230, 230, 230),
    (230, 230, 230),
    (230, 230, 230),
    (230, 230, 230),
    (230, 230, 230),  // Brightest white
];

const RAINBOW_COLORS: &[(u8, u8, u8)] = &[
    (63, 81, 181),    // 3F51B5
    (33, 150, 243),   // 2196F3
    (3, 169, 244),    // 03A9F4
    (0, 150, 136),    // 009688
    (76, 175, 80),    // 4CAF50
    (205, 220, 57),   // CDDC39
    (255, 193, 7),    // FFC107
    (255, 152, 0),    // FF9800
    (255, 87, 34),    // FF5722
    (244, 67, 54),    // F44336
];

#[derive(Debug)]
pub struct UnvalidatedBanner {
    text: String,
    colors: bool,
    subtitle: Option<String>,
    line_length: Option<usize>,
}

#[derive(Debug)]
pub struct Banner {
    text: String,
    colors: bool,
    subtitle: Option<String>,
    line_length: usize,
}

impl UnvalidatedBanner {
    pub fn build(self) -> Result<Banner, IncrediError> {
        if self.text.is_empty() {
            return Err(IncrediError::InvalidConfig("Text cannot be empty".into()));
        }

        for c in self.text.chars() {
            if !c.is_ascii_alphabetic() && c != ' ' {
                return Err(IncrediError::InvalidConfig(format!("Invalid character in text: {}", c)));
            }
        }

        Ok(Banner {
            text: self.text.to_uppercase(),
            colors: self.colors,
            subtitle: self.subtitle,
            line_length: self.line_length.unwrap_or(80),
        })
    }

    pub fn with_colors(mut self) -> Self {
        self.colors = true;
        self
    }

    pub fn with_subtitle<S: Into<String>>(mut self, subtitle: S) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn with_line_length(mut self, length: usize) -> Self {
        self.line_length = Some(length);
        self
    }
}

impl Banner {
    pub fn new<S: Into<String>>(text: S) -> UnvalidatedBanner {
        UnvalidatedBanner {
            text: text.into(),
            colors: false,
            subtitle: None,
            line_length: None,
        }
    }

    fn count_blocks(text: &str) -> usize {
        text.matches(BLOCK).count()
    }

    pub fn render(&self) -> String {
        let mut result = String::new();
        let chars: Vec<char> = self.text.chars().collect();
        
        // Get the ASCII art for each character
        let lines: Vec<Vec<&str>> = (0..4).map(|i| {
            chars.iter()
                .map(|c| get_char_line(&c.to_string(), i))
                .collect()
        }).collect();

        // Generate color arrays
        let gradient_colors: Vec<RgbColor> = GRADIENT_COLORS.iter()
            .map(|(r, g, b)| RgbColor::new(*r, *g, *b))
            .collect();

        let rainbow_colors: Vec<RgbColor> = RAINBOW_COLORS.iter()
            .map(|(r, g, b)| RgbColor::new(*r, *g, *b))
            .collect();

        // Process each line
        for line in &lines {
            let line_text = line.join("");
            
            if self.colors {
                let total_blocks = Self::count_blocks(&line_text);
                let mut block_count = 0;
                let mut chars = line_text.chars().peekable();

                while let Some(c) = chars.next() {
                    if c == '█' && chars.peek() == Some(&'█') {
                        // Found a block "██"
                        let color = if block_count >= total_blocks - rainbow_colors.len() {
                            // Last blocks get rainbow colors
                            let color_index = block_count - (total_blocks - rainbow_colors.len());
                            &rainbow_colors[color_index]
                        } else if block_count < gradient_colors.len() {
                            // First blocks get gradient colors
                            &gradient_colors[block_count]
                        } else {
                            // Middle blocks get brightest gradient color
                            gradient_colors.last().unwrap()
                        };
                        result.push_str(&color.to_ansi());
                        block_count += 1;
                        chars.next(); // Skip second █
                    } else {
                        result.push(c);
                    }
                }
            } else {
                result.push_str(&line_text);
            }
            result.push('\n');
        }

        // Add separator line
        result.push_str(&"━".repeat(self.line_length));
        result.push('\n');

        // Add subtitle if present
        if let Some(subtitle) = &self.subtitle {
            result.push_str(subtitle);
            result.push('\n');
        }

        result
    }
}

fn get_char_line(c: &str, line: usize) -> &'static str {
    FONT_MAP
        .iter()
        .find(|(ch, _)| *ch == c)
        .map(|(_, lines)| lines[line])
        .unwrap_or(FONT_MAP.last().unwrap().1[line])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_banner_creation() {
        let banner = Banner::new("TEST")
            .with_colors()
            .with_subtitle("test")
            .with_line_length(80)
            .build()
            .unwrap();
        assert!(banner.render().contains("██████"));
    }

    #[test]
    fn test_empty_input() {
        let result = Banner::new("")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_character() {
        let result = Banner::new("Test!")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_colors() {
        let banner = Banner::new("TEST")
            .with_colors()
            .build()
            .unwrap();
        assert!(banner.render().contains("\x1b[38;2;"));
    }
}

 
