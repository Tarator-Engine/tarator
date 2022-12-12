use std::{collections::HashMap, path::PathBuf, sync::Arc};

#[macro_use]
extern crate thiserror;

#[macro_use]
extern crate bitflags;

mod material;
mod mesh;
mod node;
mod primitive;
mod root;
mod scene;
mod shader;
mod store;
mod texture;
mod uniform;
mod vertex;

use cgmath::{Matrix4, Vector3};
use node::Node;
use uuid::Uuid;

trait Vec2Slice<T> {
    fn as_slice(self) -> [T; 2];
}

impl<T> Vec2Slice<T> for cgmath::Vector2<T> {
    fn as_slice(self) -> [T; 2] {
        [self.x, self.y]
    }
}

trait Vec3Slice<T> {
    fn as_slice(self) -> [T; 3];
}

impl<T> Vec3Slice<T> for cgmath::Vector3<T> {
    fn as_slice(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}

trait Vec4Slice<T> {
    fn as_slice(self) -> [T; 4];
}

impl<T> Vec4Slice<T> for cgmath::Vector4<T> {
    fn as_slice(self) -> [T; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

pub type Vec1 = cgmath::Vector1<f32>;
pub type Vec2 = cgmath::Vector2<f32>;
pub type Vec3 = cgmath::Vector3<f32>;
pub type Vec4 = cgmath::Vector4<f32>;
pub type Quat = cgmath::Quaternion<f32>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error {e}")]
    Io {
        #[from]
        e: std::io::Error,
    },
    #[error("Rust Message Pack encode Error {e}")]
    RmpE {
        #[from]
        e: rmp_serde::encode::Error,
    },
    #[error("Rust Message Pack decode Error {e}")]
    RmpD {
        #[from]
        e: rmp_serde::decode::Error,
    },
    #[error("Image Error {e}")]
    Image {
        #[from]
        e: image::ImageError,
    },
    #[error("GlTF Error {e}")]
    GlTF {
        #[from]
        e: gltf::Error,
    },
    #[error("Mutex Error {e}")]
    NodeMutex {
        #[from]
        e: std::sync::PoisonError<std::sync::MutexGuard<'static, Node>>,
    },
    #[error("The given Id does not exist")]
    NonExistentID,
    #[error("The given path does not have a file extension")]
    NoFileExtension,
    #[error("The provided image is not valid")]
    InvalidImage,
    #[error("The feature '{0}' is not yet supported")]
    NotSupported(String),
    #[error("The provided meshes do not contain position data")]
    NoPositions,
    #[error("The provided meshes do not contain normal data")]
    NoNormals,
    #[error("Failed to aquire lock on node mutex")]
    LockFailed,
    #[error("The requested material does not exist")]
    NonExistentMaterial,
    #[error("The requested shader does not exist")]
    NonExistentShader,
    #[error("The requested primitive does not exist")]
    NonExistentPrimitive,
}

pub type Result<T> = std::result::Result<T, Error>;
// pub type NodeResult<'a, T> = std::result::Result<T>;

pub struct WgpuInfo {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    surface_format: wgpu::TextureFormat,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AssetCache {
    cache: HashMap<uuid::Uuid, PathBuf>,
    orig_name: HashMap<String, Uuid>,
    last_update: chrono::DateTime<chrono::Utc>,
}

const ASSET_PATH: &'static str = "assets/";
const CACHE_NAME: &'static str = "cache.rmp";

pub type FSID = uuid::Uuid;

#[derive(Debug)]
pub struct CameraParams {
    pub position: Vector3<f32>,
    pub view_matrix: Matrix4<f32>,
    pub projection_matrix: Matrix4<f32>,
}

pub async fn update_cache(id: Uuid, location: PathBuf) -> Result<()> {
    let path = PathBuf::from(ASSET_PATH).join(CACHE_NAME);

    let mut cache = get_cache().await?;

    cache.cache.insert(id, location.clone());
    cache.orig_name.insert(
        location
            .file_name()
            .ok_or(Error::NoFileExtension)?
            .to_str()
            .unwrap()
            .to_owned(),
        id,
    );
    cache.last_update = chrono::offset::Utc::now();

    std::fs::write(path, rmp_serde::to_vec(&cache)?)?;

    Ok(())
}

pub async fn get_cache() -> Result<AssetCache> {
    let path = PathBuf::from(ASSET_PATH).join(CACHE_NAME);
    rmp_serde::from_slice(std::fs::read(path)?.as_slice()).map_err(|e| Error::RmpD { e })
}

pub fn format_model_name(model_id: uuid::Uuid) -> String {
    format!("model-{model_id}.tarm")
}

pub fn format_img_name(mat_name: String, ty: &'static str) -> String {
    format!("img-{mat_name}-{ty}.png")
}

pub async fn reset_cache() -> Result<()> {
    let path = PathBuf::from(ASSET_PATH).join(CACHE_NAME);
    let cache = AssetCache {
        cache: HashMap::new(),
        orig_name: HashMap::new(),
        last_update: chrono::offset::Utc::now(),
    };

    std::fs::write(path, rmp_serde::to_vec(&cache)?).map_err(|e| Error::Io { e })
}