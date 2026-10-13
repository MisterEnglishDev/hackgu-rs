//! Binary glTF 2.0 (.glb) serializer for .hack//G.U. models, materials, and stages.

use std::io::Cursor;
use image::{ImageBuffer, Rgba};

pub struct GlbPrimitive {
    pub position_accessor: usize,
    pub normal_accessor: Option<usize>,
    pub uv_accessor: Option<usize>,
    pub index_accessor: usize,
    pub material_index: Option<usize>,
}

pub struct GlbBuilder {
    json_scenes: Vec<String>,
    json_nodes: Vec<String>,
    json_meshes: Vec<String>,
    json_materials: Vec<String>,
    json_textures: Vec<String>,
    json_images: Vec<String>,
    json_accessors: Vec<String>,
    json_buffer_views: Vec<String>,
    bin_buffer: Vec<u8>,
}

impl GlbBuilder {
    pub fn new() -> Self {
        Self {
            json_scenes: Vec::new(),
            json_nodes: Vec::new(),
            json_meshes: Vec::new(),
            json_materials: Vec::new(),
            json_textures: Vec::new(),
            json_images: Vec::new(),
            json_accessors: Vec::new(),
            json_buffer_views: Vec::new(),
            bin_buffer: Vec::new(),
        }
    }

    /// Add raw bytes into the binary buffer aligned to a 4-byte boundary. Returns bufferView index.
    pub fn add_buffer_view(&mut self, data: &[u8], target: Option<u32>) -> usize {
        // Pad buffer to 4-byte alignment
        while self.bin_buffer.len() % 4 != 0 {
            self.bin_buffer.push(0);
        }
        let byte_offset = self.bin_buffer.len();
        let byte_length = data.len();
        self.bin_buffer.extend_from_slice(data);

        let target_str = match target {
            Some(t) => format!(",\"target\":{}", t),
            None => String::new(),
        };

        let view_idx = self.json_buffer_views.len();
        self.json_buffer_views.push(format!(
            "{{\"buffer\":0,\"byteOffset\":{},\"byteLength\":{}{}}}",
            byte_offset, byte_length, target_str
        ));
        view_idx
    }

    /// Add an accessor referencing a buffer view.
    pub fn add_accessor(
        &mut self,
        buffer_view: usize,
        component_type: u32,
        count: usize,
        type_str: &str,
        min: Option<&[f32]>,
        max: Option<&[f32]>,
    ) -> usize {
        let min_str = match min {
            Some(m) => format!(",\"min\":[{}]", m.iter().map(|v| format!("{:.6}", v)).collect::<Vec<_>>().join(",")),
            None => String::new(),
        };
        let max_str = match max {
            Some(m) => format!(",\"max\":[{}]", m.iter().map(|v| format!("{:.6}", v)).collect::<Vec<_>>().join(",")),
            None => String::new(),
        };

        let acc_idx = self.json_accessors.len();
        self.json_accessors.push(format!(
            "{{\"bufferView\":{},\"componentType\":{},\"count\":{},\"type\":\"{}\"{}{}}}",
            buffer_view, component_type, count, type_str, min_str, max_str
        ));
        acc_idx
    }

    /// Add a PNG texture encoded in-memory from RGBA8 pixels.
    pub fn add_image_rgba(&mut self, width: u32, height: u32, rgba: &[u8]) -> Result<usize, String> {
        let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(width, height, rgba.to_vec())
            .ok_or_else(|| "Failed to construct RGBA image buffer".to_string())?;

        let mut png_bytes = Vec::new();
        let mut cursor = Cursor::new(&mut png_bytes);
        img.write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|e| format!("PNG encoding error: {}", e))?;

        let bv_idx = self.add_buffer_view(&png_bytes, None);
        let img_idx = self.json_images.len();
        self.json_images.push(format!(
            "{{\"bufferView\":{},\"mimeType\":\"image/png\"}}",
            bv_idx
        ));

        // Create texture referencing this image
        let tex_idx = self.json_textures.len();
        self.json_textures.push(format!("{{\"source\":{}}}", img_idx));
        Ok(tex_idx)
    }

    /// Add a PBR material referencing a texture.
    pub fn add_material(&mut self, name: &str, texture_idx: Option<usize>) -> usize {
        let mat_idx = self.json_materials.len();
        let tex_prop = match texture_idx {
            Some(t) => format!(",\"baseColorTexture\":{{\"index\":{}}}", t),
            None => String::new(),
        };

        self.json_materials.push(format!(
            "{{\"name\":\"{}\",\"pbrMetallicRoughness\":{{\"roughnessFactor\":0.7,\"metallicFactor\":0.1{}}},\"doubleSided\":true}}",
            name, tex_prop
        ));
        mat_idx
    }

    /// Add a mesh composed of one or more primitives.
    pub fn add_mesh(&mut self, name: &str, primitives: &[GlbPrimitive]) -> usize {
        let mut prim_json = Vec::new();
        for p in primitives {
            let mut attrs = format!("\"POSITION\":{}", p.position_accessor);
            if let Some(n) = p.normal_accessor {
                attrs.push_str(&format!(",\"NORMAL\":{}", n));
            }
            if let Some(uv) = p.uv_accessor {
                attrs.push_str(&format!(",\"TEXCOORD_0\":{}", uv));
            }

            let mat_str = match p.material_index {
                Some(m) => format!(",\"material\":{}", m),
                None => String::new(),
            };

            prim_json.push(format!(
                "{{\"attributes\":{{{}}},\"indices\":{}{}}}",
                attrs, p.index_accessor, mat_str
            ));
        }

        let mesh_idx = self.json_meshes.len();
        self.json_meshes.push(format!(
            "{{\"name\":\"{}\",\"primitives\":[{}]}}",
            name,
            prim_json.join(",")
        ));
        mesh_idx
    }

    /// Add a root node containing a mesh.
    pub fn add_node_mesh(&mut self, name: &str, mesh_idx: usize) -> usize {
        let node_idx = self.json_nodes.len();
        self.json_nodes.push(format!(
            "{{\"name\":\"{}\",\"mesh\":{}}}",
            name, mesh_idx
        ));
        node_idx
    }

    /// Finalize and package into the binary .glb container.
    pub fn build(mut self, root_nodes: &[usize]) -> Vec<u8> {
        let nodes_str = root_nodes.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        self.json_scenes.push(format!("{{\"nodes\":[{}]}}", nodes_str));

        let buffer_entry = format!("{{\"byteLength\":{}}}", self.bin_buffer.len());

        let mut json_parts = Vec::new();
        json_parts.push("\"asset\":{\"version\":\"2.0\",\"generator\":\"hackgu-export\"}".to_string());
        json_parts.push("\"scene\":0".to_string());
        json_parts.push(format!("\"scenes\":[{}]", self.json_scenes.join(",")));

        if !self.json_nodes.is_empty() {
            json_parts.push(format!("\"nodes\":[{}]", self.json_nodes.join(",")));
        }
        if !self.json_meshes.is_empty() {
            json_parts.push(format!("\"meshes\":[{}]", self.json_meshes.join(",")));
        }
        if !self.json_materials.is_empty() {
            json_parts.push(format!("\"materials\":[{}]", self.json_materials.join(",")));
        }
        if !self.json_textures.is_empty() {
            json_parts.push(format!("\"textures\":[{}]", self.json_textures.join(",")));
        }
        if !self.json_images.is_empty() {
            json_parts.push(format!("\"images\":[{}]", self.json_images.join(",")));
        }
        if !self.json_accessors.is_empty() {
            json_parts.push(format!("\"accessors\":[{}]", self.json_accessors.join(",")));
        }
        if !self.json_buffer_views.is_empty() {
            json_parts.push(format!("\"bufferViews\":[{}]", self.json_buffer_views.join(",")));
        }
        json_parts.push(format!("\"buffers\":[{}]", buffer_entry));

        let json_str = format!("{{{}}}", json_parts.join(","));
        let mut json_bytes = json_str.into_bytes();

        // JSON chunk must be padded with spaces (0x20) to a 4-byte boundary
        while json_bytes.len() % 4 != 0 {
            json_bytes.push(0x20);
        }

        // BIN chunk must be padded with zeros (0x00) to a 4-byte boundary
        while self.bin_buffer.len() % 4 != 0 {
            self.bin_buffer.push(0x00);
        }

        let json_chunk_len = json_bytes.len() as u32;
        let bin_chunk_len = self.bin_buffer.len() as u32;

        let total_file_len = 12 + (8 + json_chunk_len) + if bin_chunk_len > 0 { 8 + bin_chunk_len } else { 0 };

        let mut glb = Vec::with_capacity(total_file_len as usize);

        // 12-byte glTF Header
        glb.extend_from_slice(b"glTF");
        glb.extend_from_slice(&2u32.to_le_bytes()); // version 2
        glb.extend_from_slice(&total_file_len.to_le_bytes());

        // Chunk 0: JSON
        glb.extend_from_slice(&json_chunk_len.to_le_bytes());
        glb.extend_from_slice(b"JSON");
        glb.extend_from_slice(&json_bytes);

        // Chunk 1: BIN
        if bin_chunk_len > 0 {
            glb.extend_from_slice(&bin_chunk_len.to_le_bytes());
            glb.extend_from_slice(b"BIN\0");
            glb.extend_from_slice(&self.bin_buffer);
        }

        glb
    }
}
