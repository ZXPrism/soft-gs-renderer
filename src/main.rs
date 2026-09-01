// TODO: Once done, let agent check and refine my Rust usage

use std::fs;

use image::{ExtendedColorType, ImageFormat::Png};

struct Ply {
    n_gaussians: usize,
}

struct PlyParser {
    _ply_data: Vec<u8>,
}

impl PlyParser {
    fn new(ply_path: &str) -> Self {
        let ply_data = fs::read(ply_path).unwrap();
        return PlyParser {
            _ply_data: ply_data,
        };
    }

    fn parse(&mut self) -> Result<Ply, String> {
        let mut n_gaussians: Option<usize> = None;

        // Find ply header
        // Observation: valid ply must have "end_header", so we just need to search it
        let n = self._ply_data.len();
        let end_header = b"end_header";

        let mut end_header_start_idx: Option<usize> = None;
        for i in 0..n {
            let mut found = true;
            for j in 0..end_header.len() {
                let curr = i + j;
                if curr >= n || self._ply_data[curr] != end_header[j] {
                    found = false;
                    break;
                }
            }
            if found {
                end_header_start_idx = Some(i);
                break;
            }
        }

        let Some(end_header_start_idx) = end_header_start_idx else {
            return Err("Missing \"end_header\" in ply file".into());
        };

        // Parse line by line
        let mut line_start_idx: usize = 0;
        for i in 0..end_header_start_idx {
            if self._ply_data[i] == b'\n' {
                let curr = &self._ply_data[line_start_idx..i];
                let mut split = curr.split(|&b| b == b' ');
                let head = split.next().unwrap();

                match head {
                    b"element" => {
                        n_gaussians = Some(
                            std::str::from_utf8(split.nth(1).unwrap())
                                .unwrap()
                                .parse()
                                .unwrap(),
                        );
                    }
                    _ => {
                        return Err(format!(
                            "Unknown header {}",
                            std::str::from_utf8(head).unwrap()
                        ));
                    }
                }

                line_start_idx = i + 1;
            }
        }

        let Some(n_gaussians) = n_gaussians else {
            return Err("Missing gaussian count in ply file".into());
        };

        println!("{}", n_gaussians);

        Ok(Ply {
            n_gaussians: n_gaussians,
        })
    }
}

fn main() {
    const W: usize = 640;
    const H: usize = 480;
    const N_CHANNELS: usize = 3;

    let mut pixels = vec![0_u8; W * H * N_CHANNELS];

    for y in 0..H {
        for x in 0..W {
            let offset = (y * W + x) * N_CHANNELS;
            pixels[offset] = (255 * x / (W - 1)) as u8;
            pixels[offset + 1] = (255 * y / (H - 1)) as u8;
            pixels[offset + 2] = 64;
        }
    }

    image::save_buffer_with_format(
        "output.png",
        &pixels,
        W as u32,
        H as u32,
        ExtendedColorType::Rgb8,
        Png,
    )
    .unwrap();

    let mut ply_parser = PlyParser::new("data/playroom/point_cloud.ply");
    let ply = match ply_parser.parse() {
        Ok(v) => v,
        Err(e) => {
            println!("ply parsing error: {e}");
            return;
        }
    };
}
