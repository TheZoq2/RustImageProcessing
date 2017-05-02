use std::marker::PhantomData;

pub trait Pixel{
    fn raw_data(&self) -> Vec<u8>;
}



pub struct RgbPixel
{
    pub data: [u8;3]
}
impl RgbPixel
{
    pub fn new() -> RgbPixel
    {
        RgbPixel {
            data: [0,0,0]
        }
    }
}

impl Pixel for RgbPixel {
    fn raw_data(&self) -> Vec<u8>
    {
        let result = vec!();
        result.extend_from_slice(&self.data);
        result
    }
}


pub struct GrayscalePixel
{
    pub data: u8
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
}

impl<P: Pixel> Image<P>
{
    /**
      Creates a new image of a specified size by allocating the required amount of bytes
    */
    pub fn new(resolution: (u32, u32)) -> Image<P>
    {
        let mut data = Vec::new();
        data.resize(Self::get_array_length(resolution), 0);

        Image {
            data,
            resolution,
        }
    }

    pub fn len(&self) -> usize
    {
        self.data.len()
    }

    pub fn get_raw(&self) -> Vec<u8>
    {
        let result = vec!();

        for pixel in self.data
        {
            for data in pixel.raw_data()
            {
                result.push(data);
            }
        }

        result
    }

    pub fn get_pixel(x: u32, y: u32, channel: u8)
    {
        
    }
}

