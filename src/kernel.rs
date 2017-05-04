use image;
use image::{RgbPixel, GrayscalePixel};

pub struct Kernel
{
    data: Vec<Vec<f32>>,
    radius: usize
}

impl Kernel
{
    pub fn new(radius: usize, data: Vec<Vec<f32>>) -> Kernel
    {
        Kernel {
            data,
            radius
        }
    }

    pub fn get_value(&self, x: i32, y: i32) -> f32
    {
        let x = x + self.radius as i32 / 2;
        let y = y + self.radius as i32 / 2;

        self.data[y as usize][x as usize]
    }
}



pub fn kernel_convolution(
        source: image::Image<GrayscalePixel>,
        kernel: &[&[f32]]
    ) -> image::Image<GrayscalePixel>
{
    //Go through all the target pixels
    for x in 0..source.resolution.0
    {
        for y in 0..source.resolution.1
        {
        }
    }

    unimplemented!();
}



#[cfg(test)]
mod kernel_tests
{
    use super::*;

    #[test]
    fn simple_coordinate_test()
    {
        let data = vec!(
                vec!(1. ,2. ,3.),
                vec!(4. ,5. ,6.),
                vec!(7. ,8. ,9.)
            );

        let kernel = Kernel::new(2, data);

        assert_eq!(kernel.get_value(0, 0), 5.);
        assert_eq!(kernel.get_value(0, 1), 8.);
        assert_eq!(kernel.get_value(1, 0), 6.);
        assert_eq!(kernel.get_value(-1, -1), 1.);
    }
}
