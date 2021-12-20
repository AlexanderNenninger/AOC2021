use std::fmt;
use std::str;

#[allow(unused)]
const INPUT: &str = "./input/day20.txt";
#[allow(unused)]
const INPUT_TEST: &str = "./input/day20_test.txt";

const WIDTH: usize = 5;
const HEIGTH: usize = 5;
const KERNEL_SIZE: usize = 3;
const PADDING: usize = KERNEL_SIZE / 2;

const DATASEP: &str = "\r\n\r\n";

type Kernel1d = [Pixel; KERNEL_SIZE];
type Kernel2d = [Kernel1d; KERNEL_SIZE];

fn pxs_to_num(pxs: Kernel2d) -> usize {
    ((pxs[0][2] as usize) << 8)
        + ((pxs[0][1] as usize) << 7)
        + ((pxs[0][0] as usize) << 6)
        + ((pxs[1][2] as usize) << 5)
        + ((pxs[1][1] as usize) << 4)
        + ((pxs[1][0] as usize) << 3)
        + ((pxs[2][2] as usize) << 2)
        + ((pxs[2][1] as usize) << 1)
        + ((pxs[2][0] as usize) << 0)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Pixel {
    B = 0,
    W = 1,
}

impl Pixel {
    fn to_str(&self) -> &str {
        match self {
            Self::B => ".",
            Self::W => "#",
        }
    }

    fn from_uint(n: usize) -> Result<Self, &'static str> {
        match n {
            0 => Ok(Self::B),
            1 => Ok(Self::W),
            _ => Err("Can't convert number > 1 to Pxel"),
        }
    }
}

impl str::FromStr for Pixel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::B),
            "#" => Ok(Self::W),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::B => write!(f, "."),
            Self::W => write!(f, "#"),
        }
    }
}

type ImageLine = [Pixel; PADDING + WIDTH + PADDING];
type ImageData = [ImageLine; PADDING + HEIGTH + PADDING];

#[derive(Debug, Clone, Copy)]
struct Image {
    padding_value: Pixel,
    data: ImageData,
}

impl Image {
    fn convolve_with(&self, f: &dyn Fn(usize) -> Pixel) -> Self {
        let mut out = self.clone();
        for i in PADDING..PADDING + HEIGTH {
            for j in PADDING..PADDING + WIDTH {
                let ker = [
                    [
                        self.data[i - 1][j - 1],
                        self.data[i - 1][j],
                        self.data[i - 1][j + 1],
                    ],
                    [
                        self.data[i + 0][j - 1],
                        self.data[i + 0][j],
                        self.data[i + 0][j + 1],
                    ],
                    [
                        self.data[i + 1][j - 1],
                        self.data[i + 1][j],
                        self.data[i + 1][j + 1],
                    ],
                ];
                out.data[i][j] = f(pxs_to_num(ker))
            }
        }
        out
    }
}

impl str::FromStr for Image {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data: ImageData = [[Pixel::B; WIDTH + 2 * PADDING]; HEIGTH + 2 * PADDING];
        let mut px_stack: Vec<char> = s.chars().filter(|c| *c == '.' || *c == '#').collect();
        for i in PADDING..PADDING + HEIGTH {
            for j in PADDING..PADDING + WIDTH {
                let mut buf = [0u8; 4];
                let entry = px_stack.pop().ok_or(())?.encode_utf8(&mut buf);
                data[i][j] = Pixel::from_str(entry)?;
            }
        }
        Ok(Image {
            data,
            padding_value: Pixel::B,
        })
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in PADDING..PADDING + HEIGTH {
            for j in PADDING..PADDING + WIDTH {
                write!(f, "{}", self.data[i][j])?;
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

struct Program {
    mapping: Vec<Pixel>,
    image: Image,
}

impl str::FromStr for Program {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mapping_str, img_str) = s.split_once(DATASEP).ok_or(())?;

        let image = img_str.parse::<Image>()?;

        println!(
            "{:?}",
            mapping_str
                .char_indices()
                .filter(|(idx, c)| *c != '#' && *c != '.')
                .collect::<Vec<(usize, char)>>()
        );

        let mapping = mapping_str
            .chars()
            .map(|c| c.to_string())
            .map(|s| s.parse::<Pixel>())
            .collect::<Result<Vec<Pixel>, ()>>()?;

        Ok(Program { mapping, image })
    }
}

impl Program {
    fn apply(mut self) -> Self {
        self.image = self.image.convolve_with(&|n| self.mapping[n]);
        self
    }
}

mod test {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use std::fs;

    #[test]
    fn test_pixel_from_str() {
        let b = ".";
        let w = "#";
        let err = "asd";

        assert_eq!(b.parse::<Pixel>(), Ok(Pixel::B));
        assert_eq!(w.parse::<Pixel>(), Ok(Pixel::W));
        assert_eq!(err.parse::<Pixel>(), Err(()));
    }

    #[test]
    fn test_image_from_str_to_string() {
        let data = fs::read_to_string(INPUT_TEST).unwrap();
        let img_str = data.split_once(DATASEP).unwrap();
        let image = img_str.1.parse::<Image>().expect("Could not parse image");
        println!("Image: \n{}", image);
    }

    #[test]
    fn test_program_from_str() {
        let data = fs::read_to_string(INPUT_TEST).unwrap();
        let mut program = data.parse::<Program>().unwrap();
        println!(
            "Mapping: \n{:?} \nImage: \n{}",
            program.mapping, program.image
        );
        program = program.apply();
        println!(
            "Mapping: \n{:?} \nImage: \n{}",
            program.mapping, program.image
        );
        program = program.apply();
        println!(
            "Mapping: \n{:?} \nImage: \n{}",
            program.mapping, program.image
        );
    }
}
