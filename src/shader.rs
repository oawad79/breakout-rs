struct Shader {}

impl Shader {
    pub fn new(
        gl: &Context,
        vertex_source: String,
        fragment_source: String,
        geometry_source: Option<String>,
    ) -> Self {
        let vertex_shader = gl.create_shader(glow::VERTEX_SHADER);

        Self {}
    }
}
