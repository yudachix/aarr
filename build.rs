use image::codecs::gif::GifDecoder;
use image::AnimationDecoder;
use serde::Serialize;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

/// https://github.com/0x61nas/aarty/blob/master/src/ascii_processor.rs
/// https://github.com/0x61nas/aarty/blob/master/src/args.rs
/// https://github.com/0x61nas/aarty/blob/master/LICENSE.txt
mod ascii_processor {
    use image::{Rgba, RgbaImage};
    use std::io::{self, BufWriter, Write};

    const CHARS: [char; 15] = [
        ' ', '.', ',', '-', '~', '!', ';', ':', '=', '*', '&', '%', '$', '@', '#',
    ];

    const SCALE: u32 = 4;

    pub fn generate_ascii(image: &RgbaImage) -> io::Result<BufWriter<Vec<u8>>> {
        let (width, height) = image.dimensions();
        let mut buf = BufWriter::new(Vec::new());

        for y in 0..height {
            for x in 0..width {
                if y % (SCALE * 2) == 0 && x % SCALE == 0 {
                    let element = get_char(image.get_pixel(x, y));

                    buf.write_all(element.as_bytes())?;
                }
            }

            if y % (SCALE * 2) == 0 {
                buf.write_all("\n".as_bytes())?;
            }
        }

        Ok(buf)
    }

    fn get_char(pixel: &Rgba<u8>) -> String {
        let intent = if pixel[3] == 0 {
            0
        } else {
            pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3
        };

        String::from(CHARS[(intent / (32 + 7 - (7 + (CHARS.len() - 7)) as u8)) as usize])
    }
}

#[derive(Serialize)]
struct Frame {
    delay: u32,
    content: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets/logo.gif")?;
    let decoder = GifDecoder::new(file)?;
    let mut frames: Vec<Frame> = Vec::new();

    for frame in decoder.into_frames() {
        let frame = frame?;
        let output = ascii_processor::generate_ascii(frame.buffer())?;
        let (numer, _) = frame.delay().numer_denom_ms();

        frames.push(Frame {
            delay: numer,
            content: String::from_utf8(output.into_inner()?)?,
        });
    }

    if !Path::new("dist").exists() {
        create_dir_all("dist")?;
    }

    let mut file = File::create("dist/frames.json")?;

    write!(file, "{}", serde_json::to_string(&frames).unwrap())?;

    Ok(())
}
