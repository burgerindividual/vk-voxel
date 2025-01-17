use std::{path::Path, ffi::OsStr, fs, sync::Arc};

use shaderc::{ShaderKind, CompileOptions};
use vulkano::{device::Device, shader::ShaderModule};

pub trait LoadFromPath {
    fn load(device: Arc<Device>, path: &str) -> Arc<Self>;
}

impl LoadFromPath for ShaderModule {
    fn load(device: Arc<Device>, path: &str) -> Arc<Self> {
        let compiler = shaderc::Compiler::new().unwrap();

        let shader_path = format!("./src/shaders/{}", path);
        let src = match fs::read_to_string(shader_path) {
            Ok(s) => s,
            Err(e) => panic!("Error reading shader file '{}': {}", path, e),
        };

        let extension = Path::new(path).extension().and_then(OsStr::to_str).unwrap();
        let shader_kind = match_shader_ext(extension);
        let mut compile_options = CompileOptions::new().unwrap();

        compile_options.set_generate_debug_info();

        let shader_binary = match compiler.compile_into_spirv(
            &src, 
            shader_kind, 
            path,
            "main",
            Some(&compile_options),
        ) {
            Ok(b) => b,
            Err(e) => panic!("Error compiling shader file '{}': {}", path, e),
        };
        
        unsafe {
            ShaderModule::from_words(
                device, 
                shader_binary.as_binary()
            ).unwrap()
        }
    }
}

fn match_shader_ext(ext: &str) -> ShaderKind {
    match ext {
        "vert" => ShaderKind::Vertex,
        "frag" => ShaderKind::Fragment,
        e => panic!("Unsupported shader extension: {}", e),
    }
}

pub struct ShaderPair {
    pub vertex: Arc<ShaderModule>,
    pub fragment: Arc<ShaderModule>,
}

impl ShaderPair {
    pub fn new(vertex: Arc<ShaderModule>, fragment: Arc<ShaderModule>) -> Self { Self { vertex, fragment } }

    pub fn load(device: Arc<Device>, path: &str) -> Self {
        Self {
            vertex: ShaderModule::load(device.clone(), &format!("{path}.vert")),
            fragment: ShaderModule::load(device.clone(), &format!("{path}.frag")),
        }
    }
}