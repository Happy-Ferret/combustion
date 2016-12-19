use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

use backend::generic::color::Color;

pub mod defaults;
pub mod sample;

pub use self::defaults::*;

/// Map of materials used for a certain model or scene
#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialMap {
    pub materials: HashMap<String, Material>
}

impl Deref for MaterialMap {
    type Target = HashMap<String, Material>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.materials }
}

impl DerefMut for MaterialMap {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.materials }
}

/// Represents a certain material for an object in a scene.
#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    /// Texture to apply to the material
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub texture: Option<PathBuf>,
    //TODO: Maybe texture opacity?
    /// Normal map for material
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub normal_map: Option<PathBuf>,
    /// Height map for material.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub height_map: Option<PathBuf>,
    /// Texture to be used as roughness values.
    ///
    /// See `roughness` field for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub roughness_map: Option<PathBuf>,
    /// Texture to be used as metallic values.
    ///
    /// See `metallic` field for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub metallic_map: Option<PathBuf>,
    /// Roughness of material for BRDF calculations
    #[serde(default = "Material::default_roughness")]
    pub roughness: f32,
    /// Metallic-ness of the material.
    ///
    /// A value of `None` means the material is purely dialectic.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub metallic: Option<f32>,
    /// Color of material
    #[serde(default = "Material::default_color")]
    pub color: Color,
    /// Emissive materials emit light from their surface.
    ///
    /// Uses the `color` field for the emitted light color.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub emission: Option<f32>,
    /// Overall translucency for the material. 0.0 is totally transparent and 1.0 is fully opaque.
    ///
    /// This entry can be omitted or set to `None` to assume fully opaque materials.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub translucency: Option<f32>,
    /// What specific shader should be used for the material
    #[serde(default = "Material::default_shader")]
    pub shader: MaterialShader,
    /// How the object should be rendered
    #[serde(default = "Material::default_render")]
    pub render: RenderMethod,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RenderMethod {
    /// Use traditional forward rendering for this material.
    ///
    /// Suitable for non-opaque objects or with complex reflections.
    #[serde(rename = "forward")]
    Forward,
    /// Use a more efficient but less flexible deferred rendering pipeline for this material.
    ///
    /// Suitable for most opaque objects. Alpha for this material is always interpreted as `1.0`.
    #[serde(rename = "deferred")]
    Deferred,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MaterialShader {
    /// All-in-one lighting shader used in deferred or forward rendering contexts
    #[serde(rename = "uber")]
    Uber,
    #[serde(rename = "mirror")]
    Mirror,
    #[serde(rename = "metal")]
    Metal,
    #[serde(rename = "matte")]
    Matte,
    #[serde(rename = "substrate")]
    Substrate,
    #[serde(rename = "glass")]
    Glass,
}