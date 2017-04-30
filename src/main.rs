extern crate rscam;

#[macro_use]
extern crate glium;

mod image_window;
use image_window::ImageWindow;

fn main() 
{
    let mut camera = rscam::Camera::new("/dev/video0").unwrap();

    camera.start(&rscam::Config {
        interval: (1, 30),
        .. Default::default()
    }).unwrap();

    let window = ImageWindow::new((640, 480));

    loop
    {
        camera.capture().unwrap();

        window.handle_events();
    }
}
