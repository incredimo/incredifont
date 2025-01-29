use incredimo::Banner;
fn main() {
    println!();
    let banner = Banner::new("IMPOSSIBLE")
    .with_colors()
    .with_subtitle("IMPOSSIBLE IS JUST A CHALLENGE YET TO BE SOLVED")
    .with_line_length(80)
    .build()
    .unwrap();
    println!("{}", banner.render());
}


