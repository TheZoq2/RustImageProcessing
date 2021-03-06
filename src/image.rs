use std::marker::PhantomData;

pub trait Pixel : Clone {
    fn raw_data(&self) -> Vec<u8>;
}



#[derive(Clone)]
pub struct RgbPixel
{
    pub data: [u8;3]
}
impl RgbPixel
{
    pub fn new(r: u8, g: u8, b: u8) -> RgbPixel
    {
        RgbPixel {
            data: [r,g,b]
        }
    }
}

impl Pixel for RgbPixel {
    fn raw_data(&self) -> Vec<u8>
    {
        let mut result = vec!();
        result.extend_from_slice(&self.data);
        result
    }
}


#[derive(Clone)]
pub struct GrayscalePixel
{
    pub data: u8
}

impl GrayscalePixel
{
    pub fn new(data: u8) -> GrayscalePixel
    {
        GrayscalePixel {
            data
        }
    }
}

impl Pixel for GrayscalePixel
{
    fn raw_data(&self) -> Vec<u8>
    {
        vec!(self.data)
    }
}




/**
  Main image class. Images are represented as continuous vectors in memory
*/
pub struct Image<P: Pixel>
{
    pub data: Vec<P>,
    pub resolution: (u32, u32),
    pixel_amount: usize
}

impl<P: Pixel> Image<P>
{
    fn get_pixel_amount(resolution: (u32, u32)) -> usize
    {
        (resolution.0 * resolution.1) as usize
    }
    pub fn len(&self) -> usize
    {
        self.data.len()
    }

    pub fn get_raw(&self) -> Vec<u8>
    {
        let mut result = vec!();

        for pixel in &self.data
        {
            for data in pixel.raw_data()
            {
                result.push(data);
            }
        }

        result
    }

    fn index_from_coords(&self, x: i32, y: i32) -> usize
    {
        (x + y * self.resolution.0 as i32) as usize
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> P
    {
        self.maybe_get_pixel(x as i32, y as i32).unwrap()
    }

    pub fn maybe_get_pixel(&self, x: i32, y: i32) -> Option<P>
    {
        if x > self.resolution.0 as i32
        {
            return None
        }

        let index = self.index_from_coords(x, y);

        if index < self.pixel_amount
        {
            Some(self.data[index].clone())
        }
        else
        {
            None
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: P)
    {
        let index = self.index_from_coords(x as i32, y as i32);
        self.data[index] = pixel;
    }

    pub fn get_pixel_by_index(&self, index: usize) -> P
    {
        self.data[index].clone()
    }
    pub fn set_pixel_by_index(&mut self, index: usize, pixel: P)
    {
        self.data[index] = pixel
    }
}



macro_rules! implement_image_new {
    ($type:ident, $intial_value:expr) => {
        impl Image<$type>
        {
            pub fn new(resolution: (u32, u32)) -> Image<$type>
            {
                let mut data = Vec::new();
                data.resize(Self::get_pixel_amount(resolution), $intial_value);

                let pixel_amount = data.len();

                Image {
                    data,
                    resolution,
                    pixel_amount
                }
            }
        }
    }
}

implement_image_new!(RgbPixel, RgbPixel{data: [0,0,0]});
implement_image_new!(GrayscalePixel, GrayscalePixel{data: 0});
