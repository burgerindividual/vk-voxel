use std::sync::Arc;

use ultraviolet::IVec2;
use vulkano::{buffer::BufferUsage, device::Device};

use crate::{render::{vertex::VertexRaw, mesh::{quad::BlockQuad, chunk_render::ChunkRender}, texture::TextureAtlas}, world::{block_data::StaticBlockData, chunk::Chunk}};

use super::allocator::HeapBuffer;

pub struct ChunkVertexBuffer {
    pub block_quad_buffer: HeapBuffer<BlockQuad>,
    pub deco_buffer: HeapBuffer<VertexRaw>,
}

const BQ_BUFFER_USAGE: BufferUsage = BufferUsage::STORAGE_BUFFER.union(BufferUsage::STORAGE_TEXEL_BUFFER);
const DECO_BUFFER_USAGE: BufferUsage = BufferUsage::VERTEX_BUFFER;

impl ChunkVertexBuffer {
    pub fn new(device: Arc<Device>) -> Self {
        Self {
            block_quad_buffer: HeapBuffer::new(device.clone(), BQ_BUFFER_USAGE, 6),
            deco_buffer: HeapBuffer::new(device, DECO_BUFFER_USAGE, 1),
        }
    }

    pub fn update(&mut self) -> (bool, bool) {
        let ret = (
            self.block_quad_buffer.update(),
            self.deco_buffer.update(),
        );

        ret
    }

    pub fn insert_chunk(
        &mut self, 
        chunk_pos: IVec2, 
        chunk: &Chunk, 
        atlas: &TextureAtlas, 
        block_data: &StaticBlockData
    ) {
        if self.has_chunk(chunk_pos) {
            self.remove_chunk(chunk_pos);
        }

        let render_chunk = chunk.get_render_section(atlas, block_data);

        if render_chunk.deco_vertices.len() % 3 != 0 {
            panic!("Number of vertices in the decorations were not a multiple of 3")
        }

        self.block_quad_buffer.insert(chunk_pos, &render_chunk.block_quads);
        self.deco_buffer.insert(chunk_pos, &render_chunk.deco_vertices);
    }

    pub fn remove_chunk(&mut self, chunk_pos: IVec2) {
        self.block_quad_buffer.remove(chunk_pos);
        self.deco_buffer.remove(chunk_pos);
    }

    pub fn has_chunk(&self, chunk_pos: IVec2) -> bool {
        self.block_quad_buffer.allocations.get(&chunk_pos.into()).is_some()
    }
}