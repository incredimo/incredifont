use std::env;
use std::io::{self, Write};
use clipboard::{ClipboardContext, ClipboardProvider};
use incredimo::*;

fn main() {
    // Get text from command line arguments or prompt user
    let text = match env::args().nth(1) {
        Some(text) => text,
        None => {
            print!("Enter text for banner: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input.trim().to_string()
        }
    };

    // Create and render banner
    match Banner::new(&text)
        .with_colors()
        .with_line_length(80)
        .build()
    {
        Ok(banner) => {
            let rendered = banner.render();
            println!("\n{}", rendered);
            
            // Copy to clipboard
            match ClipboardContext::new() {
                Ok(mut ctx) => {
                    if let Err(e) = ctx.set_contents(rendered) {
                        eprintln!("Failed to copy to clipboard: {}", e);
                    } else {
                        println!("[38;2;76;175;80m██[0m[38;2;205;220;57m██[0m[38;2;255;193;7m██[0m COPIED TO CLIPBOARD");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to access clipboard: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error creating banner: {}", e);
            std::process::exit(1);
        }
    }
}
 