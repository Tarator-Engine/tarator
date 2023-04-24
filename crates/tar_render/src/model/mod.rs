use wgpu::util::DeviceExt;

pub mod material;
pub mod texture;

pub struct Model {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: Option<wgpu::Buffer>,
    pub num_vertices: u32,
    pub num_indices: Option<u32>,
    pub material: material::Material,
}

impl Model {
    pub fn from_stored(
        stored: tar_res::model::Model,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target_format: wgpu::TextureFormat,
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(stored.vertices.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let (index_buffer, num_indices) = if let Some(i) = stored.indices {
            (
                Some(
                    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Index Buffer"),
                        contents: bytemuck::cast_slice(i.as_slice()),
                        usage: wgpu::BufferUsages::INDEX,
                    }),
                ),
                Some(i.len() as u32),
            )
        } else {
            (None, None)
        };

        let num_vertices = stored.vertices.len() as u32;

        let material =
            material::Material::from_stored(stored.material, device, queue, target_format);

        Self {
            vertex_buffer,
            index_buffer,
            num_vertices,
            num_indices,
            material,
        }
    }

    pub fn render<'rps>(&'rps self, render_pass: &mut wgpu::RenderPass<'rps>) {
        render_pass.set_pipeline(&self.material.pipeline);
        self.material.bind_group.set(render_pass);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        if let Some(i_buff) = &self.index_buffer {
            render_pass.set_index_buffer(i_buff.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.num_indices.unwrap(), 0, 0..1);
        } else {
            render_pass.draw(0..self.num_vertices, 0..1);
        }
    }
}