use aura_core::errors::{AuraError, Result};
use aura_core::types::{ModelManifest, QuantType};
use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

const GGUF_MAGIC: u32 = 0x46554747; // "GGUF" ASCII in little-endian

#[derive(Debug)]
pub enum GgufValue {
    Uint8(u8),
    Int8(i8),
    Uint16(u16),
    Int16(i16),
    Uint32(u32),
    Int32(i32),
    Float32(f32),
    Bool(bool),
    String(String),
    Array(Vec<GgufValue>),
    Uint64(u64),
    Int64(i64),
    Float64(f64),
}

pub struct GgufHeader {
    pub version: u32,
    pub tensor_count: u64,
    pub metadata_kv_count: u64,
    pub metadata: HashMap<String, GgufValue>,
}

fn read_gguf_string<R: Read>(reader: &mut R) -> std::io::Result<String> {
    let len = reader.read_u64::<LittleEndian>()?;
    let mut buf = vec![0u8; len as usize];
    reader.read_exact(&mut buf)?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}

fn read_gguf_value<R: Read>(reader: &mut R, value_type: u32) -> std::io::Result<GgufValue> {
    match value_type {
        0 => Ok(GgufValue::Uint8(reader.read_u8()?)),
        1 => Ok(GgufValue::Int8(reader.read_i8()?)),
        2 => Ok(GgufValue::Uint16(reader.read_u16::<LittleEndian>()?)),
        3 => Ok(GgufValue::Int16(reader.read_i16::<LittleEndian>()?)),
        4 => Ok(GgufValue::Uint32(reader.read_u32::<LittleEndian>()?)),
        5 => Ok(GgufValue::Int32(reader.read_i32::<LittleEndian>()?)),
        6 => Ok(GgufValue::Float32(reader.read_f32::<LittleEndian>()?)),
        7 => Ok(GgufValue::Bool(reader.read_u8()? != 0)),
        8 => Ok(GgufValue::String(read_gguf_string(reader)?)),
        9 => {
            let elem_type = reader.read_u32::<LittleEndian>()?;
            let len = reader.read_u64::<LittleEndian>()?;
            let mut arr = Vec::with_capacity(len as usize);
            for _ in 0..len {
                arr.push(read_gguf_value(reader, elem_type)?);
            }
            Ok(GgufValue::Array(arr))
        }
        10 => Ok(GgufValue::Uint64(reader.read_u64::<LittleEndian>()?)),
        11 => Ok(GgufValue::Int64(reader.read_i64::<LittleEndian>()?)),
        12 => Ok(GgufValue::Float64(reader.read_f64::<LittleEndian>()?)),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Unsupported GGUF value type: {}", value_type),
        )),
    }
}

pub fn parse_gguf_header(path: &Path) -> Result<GgufHeader> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let magic = reader.read_u32::<LittleEndian>()?;
    if magic != GGUF_MAGIC {
        return Err(AuraError::ModelError(format!(
            "Invalid GGUF magic header: expected 0x{:X}, got 0x{:X}",
            GGUF_MAGIC, magic
        )));
    }

    let version = reader.read_u32::<LittleEndian>()?;
    let tensor_count = reader.read_u64::<LittleEndian>()?;
    let metadata_kv_count = reader.read_u64::<LittleEndian>()?;

    let mut metadata = HashMap::new();
    for _ in 0..metadata_kv_count {
        let key = read_gguf_string(&mut reader)?;
        let val_type = reader.read_u32::<LittleEndian>()?;
        let value = read_gguf_value(&mut reader, val_type)?;
        metadata.insert(key, value);
    }

    Ok(GgufHeader {
        version,
        tensor_count,
        metadata_kv_count,
        metadata,
    })
}

pub fn parse_gguf_manifest(path: &Path) -> Result<ModelManifest> {
    let file_meta = std::fs::metadata(path)?;
    let required_file_bytes = file_meta.len();

    let header = parse_gguf_header(path)?;

    let name = header
        .metadata
        .get("general.name")
        .and_then(|v| match v {
            GgufValue::String(s) => Some(s.clone()),
            _ => None,
        })
        .unwrap_or_else(|| {
            path.file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "Unknown-Model".to_string())
        });

    let architecture_family = header
        .metadata
        .get("general.architecture")
        .and_then(|v| match v {
            GgufValue::String(s) => Some(s.clone()),
            _ => None,
        })
        .unwrap_or_else(|| "llama".to_string());

    let arch_prefix = format!("{}.", architecture_family);

    let layer_count = header
        .metadata
        .get(&format!("{}block_count", arch_prefix))
        .or_else(|| header.metadata.get(&format!("{}layer_count", arch_prefix)))
        .and_then(|v| match v {
            GgufValue::Uint32(n) => Some(*n as usize),
            GgufValue::Uint64(n) => Some(*n as usize),
            GgufValue::Int32(n) => Some(*n as usize),
            _ => None,
        })
        .unwrap_or(32);

    let context_length_max = header
        .metadata
        .get(&format!("{}context_length", arch_prefix))
        .and_then(|v| match v {
            GgufValue::Uint32(n) => Some(*n as usize),
            GgufValue::Uint64(n) => Some(*n as usize),
            GgufValue::Int32(n) => Some(*n as usize),
            _ => None,
        })
        .unwrap_or(4096);

    let attention_heads = header
        .metadata
        .get(&format!("{}attention.head_count", arch_prefix))
        .and_then(|v| match v {
            GgufValue::Uint32(n) => Some(*n as usize),
            GgufValue::Uint64(n) => Some(*n as usize),
            _ => None,
        })
        .unwrap_or(32);

    let key_value_heads = header
        .metadata
        .get(&format!("{}attention.head_count_kv", arch_prefix))
        .and_then(|v| match v {
            GgufValue::Uint32(n) => Some(*n as usize),
            GgufValue::Uint64(n) => Some(*n as usize),
            _ => None,
        })
        .unwrap_or(attention_heads);

    let head_dimension = header
        .metadata
        .get(&format!("{}attention.key_length", arch_prefix))
        .and_then(|v| match v {
            GgufValue::Uint32(n) => Some(*n as usize),
            GgufValue::Uint64(n) => Some(*n as usize),
            _ => None,
        })
        .unwrap_or(128);

    let is_moe = header
        .metadata
        .contains_key(&format!("{}expert_count", arch_prefix));

    let expert_count = header
        .metadata
        .get(&format!("{}expert_count", arch_prefix))
        .and_then(|v| match v {
            GgufValue::Uint32(n) => Some(*n as usize),
            GgufValue::Uint64(n) => Some(*n as usize),
            _ => None,
        });

    let active_experts_per_token = header
        .metadata
        .get(&format!("{}expert_used_count", arch_prefix))
        .and_then(|v| match v {
            GgufValue::Uint32(n) => Some(*n as usize),
            GgufValue::Uint64(n) => Some(*n as usize),
            _ => None,
        });

    let file_type_code = header
        .metadata
        .get("general.file_type")
        .and_then(|v| match v {
            GgufValue::Uint32(n) => Some(*n),
            _ => None,
        })
        .unwrap_or(2);

    let quantization_type = match file_type_code {
        2 => QuantType::Q4_K_M,
        3 => QuantType::Q4_K_S,
        7 => QuantType::Q3_K_S,
        1 => QuantType::Q8_0,
        0 => QuantType::FP16,
        _ => QuantType::Q4_K_M,
    };

    // Fast parameter count estimation from GGUF file size
    let total_parameters = (required_file_bytes * 8 / 4) as u64;
    let active_parameters = total_parameters;

    let file_path = path.to_string_lossy().to_string();
    let source_hash_sha256 = format!("sha256_{:x}", required_file_bytes);

    Ok(ModelManifest {
        name,
        source_hash_sha256,
        architecture_family,
        total_parameters,
        active_parameters,
        is_moe,
        expert_count,
        active_experts_per_token,
        layer_count,
        attention_heads,
        key_value_heads,
        head_dimension,
        context_length_max,
        quantization_type,
        required_file_bytes,
        file_path,
    })
}
