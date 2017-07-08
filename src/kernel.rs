use image;
use image::{RgbPixel, GrayscalePixel};

extern crate time;

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
        let x_actual = x -1 + self.radius as i32;
        let y_actual = y -1 + self.radius as i32;

        self.data[y_actual as usize][x_actual as usize]
    }

    pub fn get_radius(&self) -> usize
    {
        self.radius
    }
}



pub fn kernel_convolution(
        source: &image::Image<GrayscalePixel>,
        kernel: &Kernel
    ) -> image::Image<GrayscalePixel>
{
    let mut result = image::Image::<GrayscalePixel>::new(source.resolution);


    // Go through all the target pixels
    for x in 0..source.resolution.0
    {
        for y in 0..source.resolution.1
        {
            let mut kernel_sum = 0.;
            let mut kernel_amount = 0;

            let radius = kernel.get_radius() as i32;
            for x_offset in -(radius-1)..radius
            {
                for y_offset in -(radius-1)..radius
                {
                    let x = x as i32 + x_offset;
                    let y = y as i32 + y_offset;

                    match source.maybe_get_pixel(x, y)
                    {
                        Some(val) => {
                            kernel_sum += val.data as f32 * kernel.get_value(x_offset, y_offset);
                            kernel_amount += 1;
                        }
                        None => {}
                    }
                }
            }

            result.set_pixel(x, y, GrayscalePixel::new((kernel_sum / kernel_amount as f32) as u8));
        }
    }


    result
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
