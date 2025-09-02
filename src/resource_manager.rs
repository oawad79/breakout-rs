use std::{
    collections::HashMap,
    fs::read_to_string,
    sync::LazyLock,
};

use crate::shader::Shader;

static SHADERS: LazyLock<HashMap<String, Shader>> = LazyLock::new(HashMap::new);

fn load_shader_from_file(vertex_path: &str, fragment_path: &str, geometry_path: Option<&str>) {
    let vertex_code = read_to_string(vertex_path).expect("Failed to read vertex shader");
    let fragment_code = read_to_string(fragment_path).expect("Failed to read fragment shader");
    let geometry_code = if let Some(geom_path) = geometry_path {
        Some(read_to_string(geom_path).expect("Failed to read geometry shader"))
    } else {
        None
    };

    let shader = Shader::new();
}
