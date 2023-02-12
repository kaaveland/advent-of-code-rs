use anyhow::{Context, Result};

#[derive(Eq, PartialEq, Debug, Clone)]
struct Image<const N: usize> {
    layers: Vec<[u8; N]>,
}

fn parse_image<const N: usize>(input: &str) -> Image<N> {
    let mut layers = vec![];
    input.lines().for_each(|line| {
        let line = line.as_bytes();
        for _ in 0..line.len() / N {
            layers.push([0; N]);
        }
        for (idx, pixel) in line.iter().enumerate() {
            let layer = idx / N;
            let idx = idx % N;
            layers[layer][idx] = pixel - b'0';
        }
    });
    Image { layers }
}

pub fn part_1(input: &str) -> Result<String> {
    const DIM: usize = 25 * 6;
    let img: Image<DIM> = parse_image(input);
    img.layers
        .iter()
        .min_by_key(|layer| layer.iter().filter(|&&ch| ch == 0).count())
        .map(|layer| {
            layer.iter().filter(|&&ch| ch == 1).count()
                * layer.iter().filter(|&&ch| ch == 2).count()
        })
        .context("No layers")
        .map(|n| format!("{n}"))
}

fn decode<const N: usize>(image: &Image<N>) -> [u8; N] {
    let mut out = [0; N];
    for idx in 0..N {
        out[idx] = image
            .layers
            .iter()
            .map(|layer| layer[idx])
            .find(|&ch| ch != 2)
            .unwrap_or(0);
    }
    out
}

pub fn part_2(input: &str) -> Result<String> {
    const HEIGHT: usize = 6;
    const WIDTH: usize = 25;
    const DIM: usize = HEIGHT * WIDTH;
    let img: Image<DIM> = parse_image(input);
    let pixels = decode(&img);
    const WHITE: char = ' ';
    const BLACK: char = '#';
    let mut out = String::new();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel = pixels[y * WIDTH + x];
            out.push(if pixel == 1 { BLACK } else { WHITE });
        }
        if y != HEIGHT - 1 {
            out.push('\n');
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let img: Image<6> = parse_image("123456789012");
        assert_eq!(
            img,
            Image {
                layers: vec![[1, 2, 3, 4, 5, 6], [7, 8, 9, 0, 1, 2]]
            }
        );
        let img: Image<4> = parse_image("0222112222120000");
        assert_eq!(decode(&img), [0, 1, 1, 0]);
    }
}
