/// Contains `Accessor` and other related data structures.
pub mod accessor;

/// Contains `Animation` and other related data structures.
pub mod animation;

/// Contains `Asset` metadata.
pub mod asset;

/// Contains `Buffer`, `View`, and other related data structures.
pub mod buffer;

/// Contains `Camera` and other related data structures.
pub mod camera;

/// Contains `Image` and other related data structures.
pub mod image;

/// Contains `Material` and other related data structures.
pub mod material;

/// Contains `Mesh` and other related data structures.
pub mod mesh;

/// Contains `Root`.
pub mod root;

/// Contains `Scene`, `Node`, and other related data structures.
pub mod scene;

/// Contains `Skin` and other related data structures.
pub mod skin;

/// Contains `Texture`, `Sampler`, and other related data structures.
pub mod texture;

pub use self::root::Root;

/// Names of glTF 2.0 extensions enabled by the user.
pub const ENABLED_EXTENSIONS: &'static [&'static str] = &[
    #[cfg(feature = "KHR_lights_punctual")]
    "KHR_lights_punctual",
    #[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
    "KHR_materials_pbrSpecularGlossiness",
];

/// Names of glTF 2.0 extensions supported by the library.
pub const SUPPORTED_EXTENSIONS: &'static [&'static str] = &[
    "KHR_lights_punctual",
    "KHR_materials_pbrSpecularGlossiness",
];
