use glium;

use glium::DisplayBuild;

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
    program: glium::program::Program,
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

        ImageWindow {
            resolution,
            display,

            triangles
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

    pub fn draw_image(&self, )
    {
        
    }
}
