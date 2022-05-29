use super::{
    state::PipelineState,
    error::FrameworkError,
};

use crate::log::{info,error};

use glow::HasContext;

pub struct GPUProgram {
    id: glow::Program,
    state:*mut PipelineState,
    // Force compiler to not implement Send and Sync, because OpenGL is not thread-safe.
    // thread_mark: PhantomData<*const u8>,
    // uniform_locations: RefCell<FxHashMap<ImmutableString, Option<UniformLocation>>>,
    //pub(crate) built_in_uniform_locations: [Option<UniformLocation>; BuiltInUniform::Count as usize],
    //vertex_shader_src:String,
    //frag_shader_src:String,
}

impl Drop for GPUProgram {
    fn drop(&mut self) {
        unsafe {
            (*self.state).gl.delete_program(self.id);
        }
    }
}


impl GPUProgram{
    pub fn from_source(state:&mut PipelineState,name:&str, vertex_source:&str, fragment_source:&str)->Result<Self,FrameworkError>{
        unsafe {
            info!("⌛ creating vertex shader");
            let vertex_shader = create_shader(
                state,
                format!("{name}_{vertex_source}").as_ref(),
                glow::VERTEX_SHADER,
                vertex_source,
            )?;
            info!("⌛ creating fragment shader");
            let fragment_shader = create_shader(
                state,
                format!("{name}_{fragment_source}").as_ref(),
                glow::FRAGMENT_SHADER,
                fragment_source,
            )?;
            let program = state.gl.create_program()?;
            state.gl.attach_shader(program, vertex_shader);
            state.gl.delete_shader(vertex_shader);
            state.gl.attach_shader(program, fragment_shader);
            state.gl.delete_shader(fragment_shader);
            state.gl.link_program(program);
            let status = state.gl.get_program_link_status(program);
            let link_message = state.gl.get_program_info_log(program);

            if !status {
                error!("Failed to link {} shader: {}", name, link_message);
                Err(FrameworkError::ShaderLinkingFailed {
                    shader_name: name.to_owned(),
                    error_message: link_message,
                })
            } else {
                let msg =
                    if link_message.is_empty() || link_message.chars().all(|c| c.is_whitespace()) {
                        format!("Shader {} linked successfully!", name)
                    } else {
                        format!(
                            "Shader {} linked successfully!\nAdditional info: {}",
                            name, link_message
                        )
                    };

                info!("{:?}",msg);

                Ok(Self {
                    id: program,
                    state:state,
                    //thread_mark: PhantomData,
                    //uniform_locations: Default::default(),
                    //built_in_uniform_locations: fetch_built_in_uniform_locations(state, program),
                })
            }
        }
    }

    pub fn bind<'a>(&'a self, state: &mut PipelineState) -> GpuProgramBinding<'a> {
        state.set_program(Some(self.id));
        GpuProgramBinding {
            active_sampler: 0,
            program: self,
        }
    }

    pub fn standard(state:&mut PipelineState)->GPUProgram{
        Self::from_source(
            state,
            "standard_gpu_program",
            include_str!("shader_source/vertex_shader_source.glsl"),
            include_str!("shader_source/frag_shader_source.glsl")
            ).unwrap()
    }
}

pub struct GpuProgramBinding<'a> {
    #[allow(dead_code)]
    active_sampler: u32,
    #[allow(dead_code)]
    pub(crate) program: &'a GPUProgram,
}



unsafe fn create_shader(
    state: &mut PipelineState,
    name:&str,
    shader_type: u32,
    source: &str,
) -> Result<glow::Shader, FrameworkError> {
    info!("⌛ creating shader {shader_type}");
    let shader = state.gl.create_shader(shader_type)?;
    info!("⌛ attaching shader source");
    state.gl.shader_source(shader, source);
    info!("⌛ compile shader ");
    state.gl.compile_shader(shader);

    let status = state.gl.get_shader_compile_status(shader);
    let compilation_message = state.gl.get_shader_info_log(shader);

    if !status {
        error!("Failed to compile {} shader: {}", name, compilation_message);
        Err(FrameworkError::ShaderCompilationFailed {
            shader_name: name.to_string(),
            error_message: compilation_message,
        })
    } else {
        let msg = if compilation_message.is_empty()
            || compilation_message.chars().all(|c| c.is_whitespace())
        {
            format!("Shader {} compiled successfully!", name)
        } else {
            format!(
                "Shader {} compiled successfully!\nAdditional info: {}",
                name, compilation_message
            )
        };

        info!("{:?}",msg);

        Ok(shader)
    }
}

