extern crate rscam;

#[macro_use]
extern crate glium;

mod image_window;
mod image;
mod kernel;


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
fn yuv444_to_rgb888(source: &[u8; 3]) -> RgbPixel
{
    let c = source[0] as i32 - 16;
    let d = source[1] as i32 - 128;
    let e = source[2] as i32 - 128;

    let r = clamp((298*c + 409*e + 128) >> 8);
    let g = clamp((298*c - 100*d - 208 * e + 128) >> 8);
    let b = clamp((298*c + 516*d + 128) >> 8);

    RgbPixel::new(r, g, b)
}

/**
  Takes 4 bytes of yuv422, converts them to rgb and puts the result in destination
*/
fn yuv422_to_rgb888(source: &[u8]) -> (RgbPixel, RgbPixel)
{
    let y1 = source[0];
    let u = source[1];
    let y2 = source[2];
    let v = source[3];

    (yuv444_to_rgb888(&[y1, u, v]), yuv444_to_rgb888(&[y2, u, v]))
}


fn yuv422_image_to_rgb(source: &rscam::Frame) -> image::Image<RgbPixel>
{
    let mut image = image::Image::<RgbPixel>::new(source.resolution);

    for i in 0..(image.resolution.0 * image.resolution.1 / 2)
    {
        let source_offset = (i * 4) as usize;
        let destination_offset = (i * 2) as usize;

        let rgb = yuv422_to_rgb888(&source[source_offset..source_offset+4]);

        image.set_pixel_by_index(destination_offset, rgb.0);
        image.set_pixel_by_index(destination_offset+1, rgb.1);

    }

    image
}

fn rgb_to_greyscale(image: &image::Image<RgbPixel>) -> image::Image<GrayscalePixel>
{
    let mut result = image::Image::<GrayscalePixel>::new(image.resolution);

    for i in 0..result.len()
    {
        let rgb = image.get_pixel_by_index(i);

        let value = ((rgb.data[0] as u16 + rgb.data[1] as u16 + rgb.data[2] as u16) / 3) as u8;
        result.data[i] = GrayscalePixel::new(value);
    }

    result
}

fn grayscale_to_rgb(image: &image::Image<GrayscalePixel>) -> image::Image<RgbPixel>
{
    let mut result = image::Image::<RgbPixel>::new(image.resolution);

    for i in 0..image.len()
    {
        let value = image.data[i].data;

        result.set_pixel_by_index(i, RgbPixel::new(value, value, value));
    }
    result
}


fn main() 
{
    let mut camera = rscam::Camera::new("/dev/video0").unwrap();

    camera.start(&rscam::Config {
        interval: (1, 30),
        .. Default::default()
    }).unwrap();

    let window = ImageWindow::new((640, 480));
    //let grayscale_window = ImageWindow::new((640, 480));
    //let smoothed_window = ImageWindow::new((640, 480));
    let edge_window = ImageWindow::new((640, 480));

    let gauss_kernel = {
        let data = vec!(
                vec!(1., 4.,  6.,  4.,  1.),
                vec!(4., 16., 24., 16., 4.),
                vec!(6., 24., 36., 24., 6.),
                vec!(4., 16., 24., 16., 4.),
                vec!(1., 4.,  6.,  4.,  1.),
            ).into_iter()
            .map(|v| v.into_iter().map(|x| x / 256.).collect())
            .collect();

        kernel::Kernel::new(3, data)
    };

    let vertical_edge_kernel = {
        let data = vec!(
                vec!(-1., -0., 1.),
                vec!(-2.,  0., 2.),
                vec!(-1., -0., 1.),
            );

        kernel::Kernel::new(2, data)
    };
    let horizontal_edge_kernel = {
        let data = vec!(
                vec!(-1., -2.,-1.),
                vec!( 0.,  0., 0.),
                vec!( 1.,  2., 1.),
            );

        kernel::Kernel::new(2, data)
    };

    loop
    {
        let image = camera.capture().unwrap();

        let rgb_image = &yuv422_image_to_rgb(&image);

        let grayscale = rgb_to_greyscale(&rgb_image);

        let smoothed = kernel::kernel_convolution(&grayscale, &gauss_kernel);

        let edged = kernel::kernel_convolution(&smoothed, &vertical_edge_kernel);

        window.draw_image(rgb_image);
        //grayscale_window.draw_image(&grayscale_to_rgb(&grayscale));
        //smoothed_window.draw_image(&grayscale_to_rgb(&smoothed));
        edge_window.draw_image(&grayscale_to_rgb(&edged));

        window.handle_events();
    }
}



