use dirs::home_dir;
use hex::FromHex;
use image::io::Reader;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "themify",
    about = "A cli tool to apply a color palette to an image"
)]
struct Opt {
    /// Path to the input file
    #[structopt(short)]
    input: String,
    /// Path to the output file
    #[structopt(short)]
    output: String,
    /// name of the color palette in $HOME/.config/themify/palettes
    #[structopt(short)]
    palette: String,
}

fn main() -> image::error::ImageResult<()> {
    let opt = Opt::from_args();
    let mut img = Reader::open(opt.input)?.decode()?.to_rgb8();
    let mut path = home_dir().unwrap();
    path.push(".config/themify/palettes/");
    path.push(&opt.palette);
    let path = path.to_string_lossy().to_owned().to_string();
    println!("{}", path);
    let file = std::fs::read_to_string(path).expect("can open file");
    let palette: Vec<_> = file
        .split("\n")
        .into_iter()
        .filter_map(|str| {
            if let Ok(arr) = <[u8; 3]>::from_hex(str.strip_prefix("#")?) {
                Some(arr)
            } else {
                None
            }
        })
        .map(|arr| image::Rgb(arr))
        .collect();

    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let pixel = sel_min_dist(img.get_pixel(x, y), &palette);
            img.put_pixel(x, y, pixel);
        }
    }
    img.save(opt.output)?;
    Ok(())
}

fn sel_min_dist(pix: &image::Rgb<u8>, pal: &Vec<image::Rgb<u8>>) -> image::Rgb<u8> {
    pal.iter().fold(image::Rgb([0, 0, 0]), |best, &cur| {
        if col_dist(best, *pix) > col_dist(*pix, cur) {
            cur
        } else {
            best
        }
    })
}

fn col_dist(l: image::Rgb<u8>, r: image::Rgb<u8>) -> f32 {
    let x = l[0] as i32 - r[0] as i32;
    let y = l[1] as i32 - r[1] as i32;
    let z = l[2] as i32 - r[2] as i32;
    ((x * x + y * y + z * z) as f32).sqrt()
}
