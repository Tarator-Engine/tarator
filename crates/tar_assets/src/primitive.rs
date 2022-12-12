use std::{sync::Arc, path::Path};

use bytemuck::{Pod, Zeroable};
use cgmath::{Vector4, Vector3, Vector2, Zero, Matrix4};

use crate::{material::Material, shader::{PbrShader, ShaderFlags, MaterialInput}, scene::ImportData, Error, Result, root::Root, WgpuInfo, Vec4Slice, Vec3Slice};

use wgpu::util::DeviceExt;

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 4],
    pub tex_coord_0: [f32; 2],
    pub tex_coord_1: [f32; 2],
    pub color_0: [f32; 4],
    pub joints_0: [u32; 4],
    pub weights_0: [f32; 4],
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: [0.0; 3],
            normal: [0.0; 3],
            tangent: [0.0; 4],
            tex_coord_0: [0.0; 2],
            tex_coord_1: [0.0; 2],
            color_0: [0.0; 4],
            joints_0: [0; 4],
            weights_0: [0.0; 4],
        }
    }
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // normal
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress, // offset of position(3)
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // tangent
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 6]>() as wgpu::BufferAddress, // offset of position(3) + normal(3)
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // tex_coords_0
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 10]>() as wgpu::BufferAddress, // offset of position(3) + normal(3) + tangent(4)
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // tex_coords_1
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress, // offset of position(3) + normal(3) + tangent(4) + tex_coord_0(2)
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // color_0
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 14]>() as wgpu::BufferAddress, // offset of position(3) + normal(3) + tangent(4) + tex_coord_0(2) + tex_coord_1(2)
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // joints_0
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 18]>() as wgpu::BufferAddress, // offset of position(3) + normal(3) + tangent(4) + tex_coord_0(2) + tex_coord_1(2) + color_0(4)
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // weights_0
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 22]>() as wgpu::BufferAddress, // offset of position(3) + normal(3) + tangent(4) + tex_coord_0(2) + tex_coord_1(2) + color_0(4) + joints_0(4)
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}


#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    model: [[f32; 4]; 4],
    normal: [[f32; 3]; 3],
}
impl Instance {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Instance>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials we'll
                    // be using 2, 3, and 4, for Vertex. We'll start at slot 5 not conflict with them later
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We don't have to do this in code though.
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 10,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 11,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 12,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 19]>() as wgpu::BufferAddress,
                    shader_location: 13,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 22]>() as wgpu::BufferAddress,
                    shader_location: 14,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
pub struct Primitive {
    pub vertices: Option<wgpu::Buffer>,
    pub num_vertices: u32,

    pub indices: Option<wgpu::Buffer>,
    pub num_indices: u32,

    pub material: Arc<Material>,
    pub pbr_shader: Arc<PbrShader>,
}

impl Primitive {
    pub fn new(
        vertices: &[Vertex],
        indices: Option<Vec<u32>>,
        material: Arc<Material>,
        shader: Arc<PbrShader>,
        w_info: &WgpuInfo
    ) -> Result<Self> {
        let num_indices = indices.as_ref().map(|i| i.len()).unwrap_or(0);
        let mut prim = Primitive {
            num_vertices: vertices.len() as u32,
            num_indices: num_indices as u32,
            vertices: None,
            indices: None,
            material,
            pbr_shader: shader,
        };

        prim.setup_primitive(vertices, indices, w_info);
        Ok(prim)
    }

    pub fn from_gltf(
        g_primitive: &gltf::Primitive<'_>,
        primitive_index: usize,
        mesh_index: usize,
        root: &mut Root,
        imp: &ImportData,
        base_path: &Path,
        w_info: &WgpuInfo,
    ) -> Result<Self> {
        let buffers = &imp.buffers;
        let reader = g_primitive.reader(|buffer| Some(&buffers[buffer.index()]));
        let positions =reader
                .read_positions()
                .ok_or(Error::NoPositions)?
                .collect::<Vec<_>>();

        let mut vertices: Vec<Vertex> = positions
            .into_iter()
            .map(|position| {
                Vertex {
                    position,
                    ..Vertex::default()
                }
            }).collect();

        let mut shader_flags = ShaderFlags::empty();

        if let Some(normals) = reader.read_normals() {
            for (i, normal) in normals.enumerate() {
                vertices[i].normal = normal;
            }
            shader_flags |= ShaderFlags::HAS_NORMALS;
        }
        else {
            return Err(Error::NotSupported("normal calculation".to_owned()))
        }

        if let Some(tangents) = reader.read_tangents() {
            for (i, tangent) in tangents.enumerate() {
                vertices[i].tangent = tangent;
            }
            shader_flags |= ShaderFlags::HAS_TANGENTS;
        }

        let mut tex_coord_set = 0;
        while let Some(tex_coords) = reader.read_tex_coords(tex_coord_set) {
            if tex_coord_set > 1 {
                println!("Ignoring texture coordinate set {}, \
                    only supporting 2 sets at the moment. (mesh: {}, primitive: {})",
                    tex_coord_set, mesh_index, primitive_index);
                tex_coord_set += 1;
                continue;
            }
            for (i, tex_coord) in tex_coords.into_f32().enumerate() {
                match tex_coord_set {
                    0 => vertices[i].tex_coord_0 = tex_coord,
                    1 => vertices[i].tex_coord_1 = tex_coord,
                    _ => unreachable!()
                }
            }
            shader_flags |= ShaderFlags::HAS_UV;
            tex_coord_set += 1;
        }

        if let Some(colors) = reader.read_colors(0) {
            let colors = colors.into_rgba_f32();
            for (i, c) in colors.enumerate() {
                vertices[i].color_0 = c.into();
            }
            shader_flags |= ShaderFlags::HAS_COLORS;
        }

        if reader.read_colors(1).is_some() {
            println!("Ignoring further color attributes, only supporting COLOR_0. (mesh: {}, primitive: {})",
                mesh_index, primitive_index);
        }

        if let Some(joints) = reader.read_joints(0) {
            for (i, joint) in joints.into_u16().enumerate() {
                vertices[i].joints_0 = [joint[0]as u32, joint[1]as u32, joint[2]as u32, joint[3]as u32, ];
            }
        }
        if reader.read_joints(1).is_some() {
            println!("Ignoring further joint attributes, only supporting JOINTS_0. (mesh: {}, primitive: {})",
                mesh_index, primitive_index);
        }

        if let Some(weights) = reader.read_weights(0) {
            for (i, weights) in weights.into_f32().enumerate() {
                vertices[i].weights_0 = weights.into();
            }
        }
        if reader.read_weights(1).is_some() {
            println!("Ignoring further weight attributes, only supporting WEIGHTS_0. (mesh: {}, primitive: {})",
                mesh_index, primitive_index);
        }

        let indices = reader
            .read_indices()
            .map(|read_indices| {
                read_indices.into_u32().collect::<Vec<_>>()
            });

        let g_material = g_primitive.material();

        let mut material = None;
        if let Some(mat) = root.materials.iter().find(|m| (***m).index == g_material.index()) {
            material = Arc::clone(mat).into()
        }

        if material.is_none() { // no else due to borrow checker madness
            let mat = Arc::new(Material::from_gltf(&g_material, root, imp, base_path, w_info)?);
            root.materials.push(Arc::clone(&mat));
            material = Some(mat);
        };
        let material = material.unwrap();
        shader_flags |= material.shader_flags();

        let mut new_shader = false;
        let shader = 
            if let Some(shader) = root.shaders.get(&shader_flags) {
                Arc::clone(shader)
            }
            else {
                new_shader = true;
                Arc::new(PbrShader::new(shader_flags,
                    MaterialInput {
                        base_color_factor: material.base_color_factor.as_slice(),
                        metallic_factor: material.metallic_factor,
                        roughness_factor: material.roughness_factor,
                        normal_scale: material.normal_scale.unwrap_or(1.0),
                        occlusion_strength: material.occlusion_strength,
                        emissive_factor: material.emissive_factor.as_slice(),
                        alpha_cutoff: material.alpha_cutoff.unwrap_or(1.0),
                    }, 
                    w_info)?)
            };
        
        if new_shader {
            root.shaders.insert(shader_flags, Arc::clone(&shader));
        }

        Primitive::new(&vertices, indices, material, shader, w_info)
    }

    fn setup_primitive(&mut self, vertices: &[Vertex], indices: Option<Vec<u32>>, w_info: &WgpuInfo) {

        self.vertices = Some(
            w_info.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("vertex buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            })
        );

        if let Some(indices) = indices {
            self.indices = Some(w_info.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("index buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            }));
        }
    }

    pub fn draw(&self, render_pass: &mut wgpu::RenderPass, model_matrix: &Matrix4<f32>, mvp_matrix: &Matrix4<f32>, camera_position: &Vector3<f32>) {
        // TODO!: determine if shader+material already active to reduce work...

        // render_pass.set_pipeline(&self.pbr_shader.pipeline);
        // render_pass.set_vertex_buffer(0, self.vertices.unwrap().slice(..));
        // render_pass.set_index_buffer(self.indices.unwrap().slice(..), wgpu::IndexFormat::Uint32);
        // render_pass.set_bind_group(0, &self.pbr_shader.uniforms.bind_group.unwrap(), &[]);
        // //TODO!: texture and camera bind_groups
        // //TODO!: correct instances
        // render_pass.draw_indexed(0..self.num_indices, 0, 0..1);

    }
}