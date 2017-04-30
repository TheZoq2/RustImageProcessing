extern crate rscam;

use glium;
use glium::texture::RawImage2d;

use glium::DisplayBuild;
use glium::Surface;

use std::vec::Vec;

const VERTEX_SHADER: &str = r#"
    #version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;





#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f32, f32),
    pub tex_coords: (f32, f32),
}
implement_vertex!(Vertex, position, tex_coords);


pub struct ImageWindow
{
    resolution: (u32, u32),
    display: glium::Display,

    triangles: glium::VertexBuffer<Vertex>,
    program: glium::Program,
}

impl ImageWindow
{
    fn init_triangles() -> [Vertex; 4]
    {
        [
            Vertex{ position: (-1., -1.), tex_coords: (0., 0.) },
            Vertex{ position: ( 1., -1.), tex_coords: (1., 0.) },
            Vertex{ position: ( 1.,  1.), tex_coords: (1., 1.) },
            Vertex{ position: (-1.,  1.), tex_coords: (0., 1.) },
        ]
    }

    pub fn new(resolution: (u32, u32)) -> ImageWindow
    {
        let display = glium::glutin::WindowBuilder::new()
            .with_dimensions(resolution.0, resolution.1)
            .build_glium()
            .unwrap();

        let triangles = 
            glium::VertexBuffer::new(&display, &Self::init_triangles()).unwrap();

        let program = glium::Program::from_source(
                &display,
                VERTEX_SHADER,
                FRAGMENT_SHADER,
                None
            ).unwrap();

        ImageWindow {
            resolution,
            display,

            triangles,
            program
        }
    }

    pub fn handle_events(&self)
    {
        for ev in self.display.poll_events() {
            match ev {
                _ => {}
            }
        }
    }

    pub fn draw_image(&self, image: &rscam::Frame)
    {
        let mut pixel_vec = vec!();
        pixel_vec.extend_from_slice(image as &[u8]);

        let image = RawImage2d::from_raw_rgba(pixel_vec, image.resolution);

        let texture = glium::texture::SrgbTexture2d::new(&self.display, image).unwrap();

        let uniforms = uniform! {
            texture: texture.sampled()
        };

        let mut target = self.display.draw();

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        target.draw(&self.triangles,
                &indices,
                &self.program,
                &uniforms,
                &Default::default()
            ).unwrap();

        target.finish().unwrap();
    }
}
