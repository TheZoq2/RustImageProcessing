extern crate rscam;

#[macro_use]
extern crate glium;

mod image_window;
mod image;


use image_window::ImageWindow;

use image::{RgbPixel, GrayscalePixel};

use std::vec::Vec;


fn clamp(value: i32) -> u8
{
    if value > 255
    {
        255
    }
    else if value < 0
    {
        0
    }
    else
    {
        value as u8
    }
}

/**
  Takes a yuv444 value and converts it to a rgb value

  https://en.wikipedia.org/wiki/YUV
*/
fn yuv444_to_rgb888(source: &[u8; 3], destination: &mut [u8])
{
    let c = source[0] as i32 - 16;
    let d = source[1] as i32 - 128;
    let e = source[2] as i32 - 128;

    let r = clamp((298*c + 409*e + 128) >> 8);
    let g = clamp((298*c - 100*d - 208 * e + 128) >> 8);
    let b = clamp((298*c + 516*d + 128) >> 8);

    destination[0]=r;
    destination[1]=g;
    destination[2]=b;
}

/**
  Takes 4 bytes of yuv422, converts them to rgb and puts the result in destination
*/
fn yuv422_to_rgb888(source: &[u8], dest: &mut [u8])
{
    let y1 = source[0];
    let u = source[1];
    let y2 = source[2];
    let v = source[3];

    yuv444_to_rgb888(&[y1,u,v], &mut dest[0..3]);
    yuv444_to_rgb888(&[y2,u,v], &mut dest[3..6]);
}


fn yuv422_image_to_rgb(source: &rscam::Frame) -> image::Image<RgbPixel>
{
    let mut image = image::Image::new(source.resolution);

    for i in 0..(image.resolution.0 * image.resolution.1 / 2)
    {
        let source_offset = (i * 4) as usize;
        let destination_offset = (i * 6) as usize;

        yuv422_to_rgb888(
                &source[source_offset..source_offset+4],
                &mut image.data[destination_offset..destination_offset + 6],
            );
    }

    image
}

fn rgb_to_greyscale(image: &image::Image<RgbPixel>) -> image::Image<GrayscalePixel>
{
    let mut result = image::Image::new(image.resolution);

    for i in 0..result.len()
    {
        let rgb = &image.data[(i*3)..(i*3) + 3];

        result.data[i] = ((rgb[0] as u16 + rgb[1] as u16 + rgb[2] as u16) / 3) as u8;
    }

    result
}

fn grayscale_to_rgb(image: &image::Image<GrayscalePixel>) -> image::Image<RgbPixel>
{
    let mut result = image::Image::new(image.resolution);

    for i in 0..image.len()
    {
        result.data[i*3] = image.data[i];
        result.data[i*3 + 1] = image.data[i];
        result.data[i*3 + 2] = image.data[i]
    }
    result
}


fn kernel_convolution<P1: image::Pixel, P2: image::Pixel>(
        source: image::Image<P1>,
        kernel: &[&[u8]]
    ) -> image::Image<P2>
{
    //Go through all the pixels
}

fn main() 
{
    let mut camera = rscam::Camera::new("/dev/video0").unwrap();

    camera.start(&rscam::Config {
        interval: (1, 30),
        .. Default::default()
    }).unwrap();

    let window = ImageWindow::new((640, 480));
    let grayscale_window = ImageWindow::new((640, 480));

    loop
    {
        let image = camera.capture().unwrap();

        let rgb_image = &yuv422_image_to_rgb(&image);

        let grayscale = rgb_to_greyscale(&rgb_image);

        window.draw_image(rgb_image);
        grayscale_window.draw_image(&grayscale_to_rgb(&grayscale));

        window.handle_events();
    }
}
