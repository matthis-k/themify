use hex::FromHex;
use image::io::Reader;
fn main() -> image::error::ImageResult<()> {
    let img_path = std::env::args().nth(1).unwrap();
    let mut img = Reader::open(img_path)?.decode()?.to_rgb8();
    let file = std::fs::read_to_string(std::env::args().nth(2).unwrap()).expect("can open file");
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
    img.save(std::env::args().nth(3).unwrap())?;
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
