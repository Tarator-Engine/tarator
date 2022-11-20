use wgpu::Device;

use crate::{WgpuInfo, uniform::Uniform};

bitflags! {
    /// Flags matching the defines in the PBR shader
    pub struct ShaderFlags: u16 {
        // vertex shader + fragment shader
        const HAS_NORMALS           = 1;
        const HAS_TANGENTS          = 1 << 1;
        const HAS_UV                = 1 << 2;
        const HAS_COLORS            = 1 << 3;

        // fragment shader only
        const USE_IBL               = 1 << 4;
        const HAS_BASECOLORMAP      = 1 << 5;
        const HAS_NORMALMAP         = 1 << 6;
        const HAS_EMISSIVEMAP       = 1 << 7;
        const HAS_METALROUGHNESSMAP = 1 << 8;
        const HAS_OCCLUSIONMAP      = 1 << 9;
        const USE_TEX_LOD           = 1 << 10;
    }
}

impl ShaderFlags {
    pub fn as_strings(self) -> Vec<String> {
        (0..15)
            .map(|i| 1u16 << i)
            .filter(|i| self.bits & i != 0)
            .map(|i| format!("{:?}", ShaderFlags::from_bits_truncate(i)))
            .collect()
    }   
}

pub struct Shader {
    pub module: wgpu::ShaderModule,
}
impl Shader {
    pub fn from_source(shader_code: &str, defines: &[String], w_info: &WgpuInfo) -> Self {
        let shader = wgpu::ShaderModuleDescriptor {
            label: Some("pbr shader"),
            source: wgpu::ShaderSource::Wgsl(shader_code.into()),
        };

        let module = w_info.device.create_shader_module(shader);

        Self {
            module
        }
    }
}


pub struct PbrShader {
    pub shader: Shader,
    pub flags: ShaderFlags,
    pub uniforms: PbrUniforms,
}

impl PbrShader {
    pub fn new(flags: ShaderFlags, w_info: &WgpuInfo) -> Self {
        let mut shader = Shader::from_source(
            include_str!("shaders/pbr.wgsl"),
            &flags.as_strings(),
            w_info);

        Self {
            shader,
            flags
        }
    }
}

pub struct PbrUniforms {
    pub u_MPVMatrix: Uniform<[[f32; 4]; 4]>,
    pub u_ModelMatrix: Uniform<[[f32; 4]; 4]>,
    pub u_Camera

}