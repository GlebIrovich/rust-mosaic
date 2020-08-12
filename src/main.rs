use image::{DynamicImage, GenericImageView, Rgba};

const LEGO_BLOCK_PX_SIZE: u32 = 8;


fn main() {
    let mut img = image::open("mario.jpg").unwrap();
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


            for y in fragment_top..fragment_height {
                for x in fragment_left..fragment_width {
                    imgbuf.put_pixel(x, y, average_rect_color)
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
