use clap::Parser;
use qrcode::{render::svg, QrCode};
use resvg::tiny_skia::Pixmap;
use resvg::usvg::{Options, Tree, TreeParsing};
use std::fs;
use std::process::Command;

#[derive(Parser)]
#[command(
    author,
    about,
    version,
    long_about = None
)]
struct Args {
    #[arg(required = true, help = "The URL or string to generate a QR code for")]
    url: String,
    #[arg(short, long, help = "File path to save the QR code as a PNG file")]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    let code = QrCode::new(args.url.as_bytes()).unwrap();

    // generate svg string
    let qr_svg = code
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("black"))
        .light_color(svg::Color("white"))
        .build();

    // if output file is specified, save to that location
    if let Some(output_path) = args.output {
        fs::write(&output_path, qr_svg).unwrap();
        return;
    }

    // convert svg to png
    let opt = Options::default();
    let tree = Tree::from_str(&qr_svg, &opt).unwrap();
    let mut pixmap = Pixmap::new(200, 200).unwrap();
    let rtree = resvg::Tree::from_usvg(&tree);
    rtree.render(resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());

    // save png to temporary file
    let temp_path = std::env::temp_dir().join("qr_temp.png");
    pixmap.save_png(&temp_path).unwrap();

    // open the file with default system viewer
    #[cfg(target_os = "macos")]
    Command::new("open").arg(&temp_path).spawn().unwrap();

    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(["/C", "start", ""])
        .arg(&temp_path)
        .spawn()
        .unwrap();

    #[cfg(target_os = "linux")]
    Command::new("xdg-open").arg(&temp_path).spawn().unwrap();
}
