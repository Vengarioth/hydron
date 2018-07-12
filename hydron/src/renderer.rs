use gl;
use gl::types::*;

use std::ffi::CString;
use std::ptr;
use std::str;
use std::mem;
use std::os::raw::c_void;
use hydron_ui::rendering::Command;
use hydron_ui::Size;

static VS_SRC: &'static str = r#"
    #version 330 core
    layout (location = 0) in vec2 position;
    void main() {
       gl_Position = vec4(position.xy, 0.0, 1.0);
    }
"#;

static FS_SRC: &'static str = r#"
    #version 330 core
    uniform vec4 color;
    out vec4 fragColor;
    void main() {
       fragColor = color;
    }
"#;
pub struct Renderer {
    shaderProgram: u32,
    VBO: u32,
    VAO: u32,
    EBO: u32,
}

impl Renderer {
    pub fn new() -> Renderer {

        let (shaderProgram, VBO, VAO, EBO) = unsafe {
            // build and compile our shader program
            // ------------------------------------
            // vertex shader
            let vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(VS_SRC.as_bytes()).unwrap();
            gl::ShaderSource(vertexShader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertexShader);

            // check for shader compile errors
            let mut success = gl::FALSE as GLint;
            let mut infoLog = Vec::with_capacity(512);
            infoLog.set_len(512 - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderiv(vertexShader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(vertexShader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&infoLog).unwrap());
            }

            // fragment shader
            let fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(FS_SRC.as_bytes()).unwrap();
            gl::ShaderSource(fragmentShader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragmentShader);
            // check for shader compile errors
            gl::GetShaderiv(fragmentShader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(fragmentShader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&infoLog).unwrap());
            }

            // link shaders
            let shaderProgram = gl::CreateProgram();
            gl::AttachShader(shaderProgram, vertexShader);
            gl::AttachShader(shaderProgram, fragmentShader);
            gl::LinkProgram(shaderProgram);
            // check for linking errors
            gl::GetProgramiv(shaderProgram, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shaderProgram, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&infoLog).unwrap());
            }
            gl::DeleteShader(vertexShader);
            gl::DeleteShader(fragmentShader);

            // set up vertex data (and buffer(s)) and configure vertex attributes
            // ------------------------------------------------------------------
            // add a new set of vertices to form a second triangle (a total of 6 vertices); the vertex attribute configuration remains the same (still one 3-float position vector per vertex)
            let vertices: [f32; 8] = [
                // first triangle
                -1.0, -1.0,  // left
                1.0, -1.0,  // right
                1.0,  1.0,  // top
                -1.0,  1.0,  // top
            ];

            let indices = [
                0, 1, 2,
                0, 2, 3,
            ];

            let (mut VBO, mut VAO, mut EBO) = (0, 0, 0);
            gl::GenVertexArrays(1, &mut VAO);
            gl::GenBuffers(1, &mut VBO);
            gl::GenBuffers(1, &mut EBO);

            gl::BindVertexArray(VAO);

            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::BufferData(gl::ARRAY_BUFFER,
                        (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &vertices[0] as *const f32 as *const c_void,
                        gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                        (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &indices[0] as *const i32 as *const c_void,
                        gl::STATIC_DRAW);

            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 2 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindVertexArray(0);

            (shaderProgram, VBO, VAO, EBO)
        };

        Renderer {
            shaderProgram,
            VBO,
            VAO,
            EBO,
        }
    }

    pub fn render(&mut self, size: Size, commands: Vec<Command>) {
        if size.width < 1 || size.height < 1 {
            return;
        }

        for command in commands {
            match command {
                Command::DrawRect(box_rect, box_color) => {
                    unsafe {
                        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

                        gl::UseProgram(self.shaderProgram);
                        gl::BindVertexArray(self.VAO);

                        let color_string = CString::new("color").unwrap();
                        let color_location = gl::GetUniformLocation(self.shaderProgram, color_string.as_ptr());
                        
                        let x = (box_rect.x as f32 / size.width as f32) * 2.0;
                        let y = (box_rect.y as f32 / size.height as f32) * 2.0;
                        let width = (box_rect.width as f32 / size.width as f32) * 2.0;
                        let height = (box_rect.height as f32 / size.height as f32) * 2.0;

                        let vertices: [f32; 8] = [
                            -1.0 + x,         1.0 - y - height,
                            -1.0 + x + width, 1.0 - y - height,
                            -1.0 + x + width, 1.0 - y,
                            -1.0 + x,         1.0 - y,
                        ];

                        gl::BindBuffer(gl::ARRAY_BUFFER, self.VBO);
                        gl::BufferData(gl::ARRAY_BUFFER,
                        (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &vertices[0] as *const f32 as *const c_void,
                        gl::STATIC_DRAW);

                        gl::Uniform4f(
                            color_location,
                            box_color.r as f32,
                            box_color.g as f32,
                            box_color.b as f32,
                            box_color.a as f32
                        );
                        
                        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
                        gl::BindVertexArray(0);
                    }
                }
            }
        }
    }
}
