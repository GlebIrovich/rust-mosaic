use image::{DynamicImage, GenericImageView, Rgba, Rgb, Pixel};
use num_traits::pow::Pow;

const LEGO_BLOCK_PX_SIZE: u32 = 8;

struct ColorBrick<'a> {
    id: u32,
    name: &'a str,
    color: Rgb<u8>
}

const DARK_STONE_GREY: ColorBrick = ColorBrick {
    id: 4210719,
    name: "Dark stone grey",
    color: Rgb([99, 95, 97])
};

const BLACK: ColorBrick = ColorBrick {
    id: 302426,
    name: "Black",
    color: Rgb([27, 42, 52])
};

const EARTH_BLUE: ColorBrick = ColorBrick {
    id: 4184108,
    name: "Earth Blue",
    color: Rgb([32, 58, 86])
};

const BRIGHT_YELLOW: ColorBrick = ColorBrick {
    id: 302424,
    name: "Bright Yellow",
    color: Rgb([245, 205, 47])
};

const MEDIUM_STONE_GREY: ColorBrick = ColorBrick {
    id: 4211399,
    name: "Medium Stone Grey",
    color: Rgb([163, 162,164])
};

const REDDISH_BROWN: ColorBrick = ColorBrick {
    id: 4221744,
    name: "Reddish Brown",
    color: Rgb([105,64,39])
};

const BRIGHT_BLUE: ColorBrick = ColorBrick {
    id: 302423,
    name: "Bright Blue",
    color: Rgb([13, 105, 	171])
};

const BRIGHT_RED: ColorBrick = ColorBrick {
    id: 302421,
    name: "Bright Red",
    color: Rgb([196,40,27])
};

const WHITE: ColorBrick = ColorBrick {
    id: 302401,
    name: "White",
    color: Rgb([242,243,242])
};

const BRICK_YELLOW: ColorBrick = ColorBrick {
    id: 4159553,
    name: "Brick Yellow",
    color: Rgb([215,197,153])
};

const BRIGHT_ORANGE: ColorBrick = ColorBrick {
    id: 4524929,
    name: "Bright Orange",
    color: Rgb([218,133,64])
};

const DARK_GREEN: ColorBrick = ColorBrick {
    id: 302428,
    name: "Dark Green",
    color: Rgb([40, 127, 70])
};


const BRICKS: [ColorBrick; 12] = [
    DARK_STONE_GREY,
    BLACK,
    EARTH_BLUE,
    BRIGHT_YELLOW,
    MEDIUM_STONE_GREY,
    REDDISH_BROWN,
    BRIGHT_BLUE,
    BRIGHT_RED,
    WHITE,
    BRICK_YELLOW,
    BRIGHT_ORANGE,
    DARK_GREEN
];

fn main() {
    let mut img = image::open("image.jpg").unwrap();
    let (width, height) = img.dimensions();
    println!("Image has width {}, height {}", width, height);
    println!("Bounds: {:?}", img.bounds());
    println!("{:?}", average_color(&img, &img.bounds()));

    let mut new_width = width;
    let mut new_height = height;

    if width % LEGO_BLOCK_PX_SIZE != 0 {
        new_width = width - width % LEGO_BLOCK_PX_SIZE
    }

    if height % LEGO_BLOCK_PX_SIZE != 0 {
        new_height = height - height % LEGO_BLOCK_PX_SIZE
    }

    let mut cropped_img = img.crop(0, 0, new_width, new_height);
    flatten_colors(& mut cropped_img);
}

fn flatten_colors(img: &mut DynamicImage) {
    let (width, height) = img.dimensions();
    let mut imgbuf = image::ImageBuffer::new(width, height);
    println!("Flattening colors");
    for horizontal in 0..(width / LEGO_BLOCK_PX_SIZE) {
        for vertical in 0..(height / LEGO_BLOCK_PX_SIZE) {

            let fragment_top = vertical * LEGO_BLOCK_PX_SIZE;
            let fragment_height = fragment_top + LEGO_BLOCK_PX_SIZE;
            let fragment_left = horizontal * LEGO_BLOCK_PX_SIZE;
            let fragment_width = fragment_left + LEGO_BLOCK_PX_SIZE;

            let bounds = (fragment_left, fragment_top, fragment_width, fragment_height);
            let average_rect_color = average_color(&img, &bounds);

            let lego_color = BRICKS.iter().fold(&BRICKS[0], |acc, brick| {
                let dist_acc = euclidean_distance(&average_rect_color.to_rgb(), &acc.color);
                let dist_current = euclidean_distance(&average_rect_color.to_rgb(), &brick.color);
                if dist_current < dist_acc {
                    return brick
                }
                acc
            });


            for y in fragment_top..fragment_height {
                for x in fragment_left..fragment_width {
                    imgbuf.put_pixel(x, y, lego_color.color)
                }
            }


            println!("Horizontal: {}. Vertical: {}. Color: {:?}", horizontal, vertical, average_rect_color);
        }
    }
    imgbuf.save("test.jpg").unwrap();
}

fn average_color(img: &DynamicImage, rect: &(u32, u32, u32, u32)) -> Rgba<u8> {
    let (left, top, width, height) = rect;
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    let mut a = 0.0;
    let mut count = 0.0;
    for y in *top..*height {
        for x in *left..*width {
            let pixel = img.get_pixel(x, y);
            r += f64::from(pixel[0]);
            g += f64::from(pixel[1]);
            b += f64::from(pixel[2]);
            a += f64::from(pixel[3]);
            count += 1.0;
        }
    }
    let r = (r / count).round() as u8;
    let g = (g / count).round() as u8;
    let b = (b / count).round() as u8;
    let a = (a / count).round() as u8;

    Rgba([r, g, b, a])
}

fn euclidean_distance(color1: &Rgb<u8>, color2: &Rgb<u8>) -> f32 {
    // d(p,q) = sqrt((px - qx)^2 + (py - qy)^2 + (pz - qz)^2)
    let (r1, g1, b1) = (f32::from(color1[0]), f32::from(color1[1]), f32::from(color1[2]));
    let (r2, g2, b2) = (f32::from(color2[0]), f32::from(color2[1]), f32::from(color2[2]));

    let delta_r = (r1 - r2).pow(2);
    let delta_g = (g1 - g2).pow(2);
    let delta_b = (b1 - b2).pow(2);

    f32::sqrt(delta_r + delta_g + delta_b)
}
