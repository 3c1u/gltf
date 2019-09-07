use gltf_derive::Validate;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// The root object of a glTF 2.0 asset.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Root {
    #[serde(default, flatten)]
    map: HashMap<String, Value>,
}

#[derive(Debug)]
pub enum ExtensionError {
    NotFound,
    JsonError(serde_json::Error),
}

impl From<serde_json::Error> for ExtensionError {
    fn from(e: serde_json::Error) -> Self {
        Self::JsonError(e)
    }
}

impl Root {
    pub fn extension<T>(&self, name: &str) -> Result<T, ExtensionError>
    where
        T: DeserializeOwned,
    {
        serde_json::from_value(self.map.get(name).ok_or(ExtensionError::NotFound)?.clone())
            .map_err(|e| e.into())
    }
    pub fn set_extension<T>(&mut self, name: &str, t: T) -> Option<()>
    where
        T: Serialize,
    {
        self.map
            .insert(name.to_owned(), serde_json::to_value(t).ok()?);
        Some(())
    }
}

#[cfg(feature = "KHR_lights_punctual")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct KhrLightsPunctual {
    /// Lights at this node.
    pub lights: Vec<crate::extensions::scene::khr_lights_punctual::Light>,
}
