use crate::logger as eveon_logger;
use cgmath::Matrix;
use gl::types::*;

use std::{
    collections::HashMap,
    ffi::{c_void, CString},
    fs::File,
    io::Read,
    mem, process, ptr,
};

pub struct Vao {
    id: GLuint,
}

impl Vao {
    pub fn create() -> Self {
        let mut id = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        Self { id }
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }
}

pub struct BufferObject {
    id: GLuint,
    r#type: GLenum,
    usage: GLenum,
}

impl BufferObject {
    pub fn create(r#type: GLenum, usage: GLenum) -> Self {
        let mut id = 0;

        unsafe {
            gl::GenBuffers(0, &mut id);
        }

        Self { id, r#type, usage }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(self.r#type, self.id) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(self.r#type, 0) }
    }

    pub fn save_f32_data(&self, data: &[f32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &data[0] as *const f32 as *const c_void,
                self.usage,
            )
        }
    }

    pub fn save_i32_data(&self, data: &[i32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &data[0] as *const i32 as *const c_void,
                self.usage,
            )
        }
    }
}

pub struct VertexAttr {
    index: GLuint,
}

impl VertexAttr {
    pub fn create(
        index: u32,
        size: i32,
        r#type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) -> Self {
        unsafe {
            gl::VertexAttribPointer(index, size, r#type, normalized, stride, pointer);
        }

        Self { index }
    }

    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }

    pub fn disable(&self) {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
    }
}

pub struct ShaderProgram {
    program: u32,
    uniform_ids: HashMap<String, GLint>,
}

#[allow(temporary_cstring_as_ptr)]
impl ShaderProgram {
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> Self {
        let mut vertex_shader_file = File::open(vertex_shader_path).unwrap_or_else(|_| {
            eveon_logger::error!("failed to open file {}", vertex_shader_path);
            process::exit(1)
        });

        let mut fragment_shader_file = File::open(fragment_shader_path).unwrap_or_else(|_| {
            eveon_logger::error!("failed to open file {}", vertex_shader_path);
            process::exit(1)
        });

        let mut vertex_source = String::new();
        let mut fragment_source = String::new();

        vertex_shader_file
            .read_to_string(&mut vertex_source)
            .expect("Failed to read vertex source");

        fragment_shader_file
            .read_to_string(&mut fragment_source)
            .expect("Failed to read fragment source");

        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let vertex_c_str = CString::new(vertex_source.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &vertex_c_str.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let fragment_c_str = CString::new(fragment_source.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &fragment_c_str.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);

            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Self {
                program,
                uniform_ids: HashMap::new(),
            }
        }
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.program) }
    }

    pub fn unbind(&self) {
        unsafe { gl::UseProgram(0) }
    }

    pub fn create_uniform(&mut self, name: &str) {
        let uni_location =
            unsafe { gl::GetUniformLocation(self.program, CString::new(name).unwrap().as_ptr()) };

        if uni_location < 0 {
            eveon_logger::error!("can not locate uniform {}", name);
            process::exit(1);
        }

        self.uniform_ids.insert(name.to_string(), uni_location);
    }

    pub fn set_uniform_matrix4(&self, name: &str, matrix: &cgmath::Matrix4<f32>) {
        unsafe { gl::UniformMatrix4fv(self.uniform_ids[name], 1, gl::FALSE, matrix.as_ptr()) }
    }
}
