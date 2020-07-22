use knitting_parse::Pattern;

use std::{error::Error, fs::File, io::BufWriter, path::Path};

mod image;

fn main() -> Result<(), Box<dyn Error>> {
    let in_file = File::open("test.knit")?;
    let pattern = Pattern::new(in_file)?;

    let path = Path::new("test.png");
    let out_file = File::create(path)?;

    let ref mut writer = BufWriter::new(out_file);

    let pixel_width = pattern.pattern_width() * 32;
    let pixel_height = pattern.lines().len() * 32;

    let mut encoder = png::Encoder::new(writer, pixel_width as u32, pixel_height as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;
    let image_cache = image::load_cache()?;
    let mut data: Vec<u8> = Vec::new();

    let mut side = pattern.starting_side();
    for line in pattern.lines() {
        let mut data_line: Vec<Vec<u8>> = Vec::new();

        // There's probably a better way to do this, but each pic is 32 rows of images.  We need each row concated.
        // concat each row into it's own vec, then we'll combine them.
        for _ in 0..32 {
            data_line.push(Vec::new());
        }

        for stitch in line {
            // We shouldn't hit an unknown type as we can only get back the enum from the parser.
            let stitch_images = image_cache.get(&(*stitch, side)).unwrap();

            for stitch_image in stitch_images {
                // We made the images, so they better chunk right.  32 pixels & 4 bytes per row.
                for (row, data) in stitch_image.chunks_exact(128).enumerate() {
                    data_line[row].extend(data);
                }
            }
        }

        for data_line in data_line.iter_mut() {
            data.append(data_line);
        }

        side = side.switch(pattern.in_round());
    }

    writer.write_image_data(&data)?;

    Ok(())
}
