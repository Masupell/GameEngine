use anyhow::Ok;

pub enum RenderCommand
{
    // ClearScreen { color: wgpu::Color },
    DrawObject
    {
        pipeline: wgpu::RenderPipeline,
        bind_group: wgpu::BindGroup,
        vertex_buffer: wgpu::Buffer,
        index_buffer: wgpu::Buffer,
        num_indices: u32
    }
}

struct RenderContext
{
    commands: Vec<RenderCommand>
}

impl RenderContext
{
    pub fn add(&mut self, command: RenderCommand)
    {
        self.commands.push(command);
    }

    pub fn execute(&mut self, renderer: Renderer)
    {
        // render.process_commands(&self.commands);
        self.commands.clear();
    }
}

pub struct Renderer<'a>
{
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'a>,
    camera_bind_group: wgpu::BindGroup,
    clear_color: wgpu::Color
}

impl<'a> Renderer<'a>
{  
    pub fn process_commands(&mut self, commands: &[RenderCommand], render_pipeline: &wgpu::RenderPipeline)// -> Result<(), wgpu::SurfaceError> 
    {
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor
        {
            label: Some("Render Encoder")
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor 
        {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment 
            {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations 
                {
                    load: wgpu::LoadOp::Clear(self.clear_color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        // render_pass.set_bind_group(index, bind_group, offsets);

        for command in commands
        {
            match command
            {
                RenderCommand::DrawObject { pipeline, bind_group, vertex_buffer, index_buffer, num_indices } =>
                {
                    render_pass.set_pipeline(pipeline);
                    render_pass.set_bind_group(0, bind_group, &[]);
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    render_pass.draw_indexed(0..*num_indices, 0, 0..1);
                }
                _ => {}
            }
        }
        

        //Ok(())
    }
}

/*
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor 
        {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor 
            {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment 
                {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations 
                    {
                        load: wgpu::LoadOp::Clear(wgpu::Color 
                        {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]); // Texture
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]); // Camera

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..)); // Hexagon vertices
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // Hexagon Indices
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
*/