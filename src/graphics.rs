extern crate glfw;
extern crate gl;
use glfw::{Action, Context, Key};
use gl::types::*;

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
"#;
const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330 core
out vec4 FragColor;

uniform vec4 rectColor;

void main()
{
    FragColor = rectColor;
} 
"#;

pub struct Window {
    width: u32,
    height: u32,
    shader_program: GLuint,
    vao: u32,
    rects: Vec<crate::layout::Rect>,
    window: glfw::Window,
    glfw: glfw::Glfw,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>
}

impl Window {
    pub fn new() -> Window {
	let width = 500;
	let height = 500;
	let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
	let (mut window, events) = glfw.create_window(width, height, "Window", glfw::WindowMode::Windowed)
            .expect("Failed to create window.");
	window.set_key_polling(true);
	window.make_current();

	// load opengl function pointers
	gl::load_with(|s| window.get_proc_address(s) as *const _);

	// unsafe graphics stuff
	let (shader_program, vao) = unsafe {
	    // compile shaders
	    let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
	    gl::ShaderSource(vertex_shader, 1, &std::ffi::CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap().as_ptr(), std::ptr::null());
	    gl::CompileShader(vertex_shader);
	    let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
	    gl::ShaderSource(fragment_shader, 1, &std::ffi::CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap().as_ptr(), std::ptr::null());
	    gl::CompileShader(fragment_shader);
	    // shader program
	    let shader_program = gl::CreateProgram();
	    gl::AttachShader(shader_program, vertex_shader);
	    gl::AttachShader(shader_program, fragment_shader);
	    gl::LinkProgram(shader_program);
	    gl::DeleteShader(vertex_shader);
	    gl::DeleteShader(fragment_shader);
	    // vao
	    let mut vao: u32 = 0;
	    gl::GenVertexArrays(1, &mut vao);
	    gl::BindVertexArray(vao);
	    
	    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<GLfloat>() as GLsizei, std::ptr::null());
	    gl::EnableVertexAttribArray(0);

	    (shader_program, vao)
	};
	// window related stuff
	// return Window struct
	return Window{width: width, height: height, shader_program: shader_program, vao: vao, rects: Vec::new(), window: window, glfw: glfw, events: events};
    }
    pub fn start(&mut self) {
	while !self.window.should_close() {
	    // clear window
	    unsafe {
		gl::ClearColor(1.0, 1.0, 1.0, 1.0);
		gl::Clear(gl::COLOR_BUFFER_BIT);

		for rect in &self.rects {
		    // rectangle positions
		    let start_x: f32 = self.get_relative_pos_x(rect.x);
		    let start_y: f32 = self.get_relative_pos_y(rect.y);
		    let end_x: f32 = self.get_relative_pos_x(rect.x+rect.width);
		    let end_y: f32 = self.get_relative_pos_y(rect.y+rect.height);
		    // vertices of rectangle
		    let vertices: [f32;12] = [
			start_x, start_y, 0.0, // top-left
			end_x, start_y, 0.0,   // top-right
			end_x, end_y, 0.0,     // bottom-right
			start_x, end_y, 0.0,   // bottom-left
		    ];
		    let indices: [u32;6] = [
			0, 1, 3,
			1, 2, 3
		    ];
		    // vbo
		    let mut vbo: u32 = 0;
		    gl::GenBuffers(1, &mut vbo);
		    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		    gl::BufferData(gl::ARRAY_BUFFER,
				   (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
				   &vertices[0] as *const f32 as *const std::os::raw::c_void,
				   gl::STATIC_DRAW);
		    // ebo
		    let mut ebo: u32 = 0;
		    gl::GenBuffers(1, &mut ebo);
		    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
		    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
				   (indices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
				   &indices[0] as *const u32 as *const std::os::raw::c_void,
				   gl::STATIC_DRAW);
		    // set color
		    let rect_color = std::ffi::CString::new("rectColor").unwrap();
		    let vertex_color_location = gl::GetUniformLocation(self.shader_program, rect_color.as_ptr());
		    gl::Uniform4f(vertex_color_location, rect.color[0], rect.color[1], rect.color[2], 1.0);
		    // draw stuff
		    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<GLfloat>() as GLsizei, std::ptr::null());
		    gl::EnableVertexAttribArray(0);
		    gl::UseProgram(self.shader_program);
		    gl::BindVertexArray(self.vao);
		    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
		}
	    }
	    self.window.swap_buffers();
	    self.glfw.poll_events();
	    for (_, event) in glfw::flush_messages(&self.events) {
		match event {
		    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
			self.window.set_should_close(true)
		    }
		    _ => {}
		}
	    }
	}
    }
    fn get_relative_pos_x(&self, pos: crate::layout::Distance) -> f32 {
	match pos {
	    crate::layout::Distance::Absolute(pixels) => (pixels/self.width as f32)*2. - 1.,
	    crate::layout::Distance::Relative(percent) => percent*2. - 1.,
	    crate::layout::Distance::Combo(pixels, percent) => (pixels/self.width as f32)*2. + percent*2. - 1.
	}
    }
    fn get_relative_pos_y(&self, pos: crate::layout::Distance) -> f32 {
	match pos {
	    crate::layout::Distance::Absolute(pixels) => (pixels/self.height as f32)*-2. + 1.,
	    crate::layout::Distance::Relative(percent) => percent*-2. + 1.,
	    crate::layout::Distance::Combo(pixels, percent) => (pixels/self.height as f32)*-2. + percent*-2. + 1.
	}
    }    
    pub fn render_document(&mut self, document: std::rc::Rc<crate::html::Node>) {
	let (mut rects, _) = crate::layout::render_node(document, crate::layout::Distance::Absolute(0.));
	self.rects.append(&mut rects);
    }
}

pub fn get_color(color: String) -> [f32;3] {
    if color.starts_with("rgb") {
	let args = color.trim_start_matches("rgb(").trim_end_matches(")").split(",").collect::<Vec<&str>>();
	[args[0].trim().parse::<f32>().unwrap()/255.,
	 args[1].trim().parse::<f32>().unwrap()/255.,
	 args[2].trim().parse::<f32>().unwrap()/255.]
    } else if color.starts_with("#") {
	let hex_color = color.trim_start_matches("#");
	let r = u32::from_str_radix(&hex_color[0..2], 16).unwrap() as f32;
        let g = u32::from_str_radix(&hex_color[2..4], 16).unwrap() as f32;
        let b = u32::from_str_radix(&hex_color[4..6], 16).unwrap() as f32;
	[r/255.,
	 g/255.,
	 b/255.]
    } else {
	for (key, value) in crate::rules::DEFAULT_COLORS {
	    if key.to_string() == color {
		return get_color(value.to_string());
	    }
	}
	[0.0, 0.0, 0.0]
    }
}
