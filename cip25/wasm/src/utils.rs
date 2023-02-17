use crate::*;

use wasm_bindgen::prelude::JsError;

use cml_core_wasm::metadata::Metadata;

#[wasm_bindgen]
impl CIP25Metadata {
    /// Create a Metadata containing only the CIP25 schema
    pub fn to_metadata(&self) -> Result<Metadata, JsError> {
        self.0.to_metadata().map(Metadata::from).map_err(Into::into)
    }

    /// Read the CIP25 schema from a Metadata. Ignores all other data besides CIP25
    /// Can fail if the Metadata does not conform to CIP25
    pub fn from_metadata(metadata: &Metadata) -> Result<CIP25Metadata, JsError> {
        core::CIP25Metadata::from_metadata(metadata.as_ref())
            .map(Self)
            .map_err(Into::into)
    }

    /// Add to an existing metadata (could be empty) the full CIP25 metadata
    pub fn add_to_metadata(&self, metadata: &mut Metadata) -> Result<(), JsError> {
        self.0
            .add_to_metadata(metadata.as_mut())
            .map_err(Into::into)
    }
}

#[wasm_bindgen]
impl String64 {
    pub fn new(s: &str) -> Result<String64, JsError> {
        core::String64::new_str(s).map(Self).map_err(Into::into)
    }

    pub fn to_str(&self) -> String {
        self.0.to_str().to_owned()
    }

    pub fn get_str(&self) -> String {
        self.0.get().clone()
    }
}

#[wasm_bindgen]

impl ChunkableString {
    pub fn from_string(str: &str) -> Self {
        Self(core::ChunkableString::from(str))
    }

    pub fn to_string(&self) -> String {
        String::from(&self.0)
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MiniMetadataDetails(pub(crate) core::utils::MiniMetadataDetails);

#[wasm_bindgen]
impl MiniMetadataDetails {
    pub fn new() -> Self {
        MiniMetadataDetails(core::utils::MiniMetadataDetails {
            name: None,
            image: None
        })
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<MiniMetadataDetails, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn set_name(&mut self, name: &String64) {
        self.0.name = Some(name.clone().into())
    }

    pub fn name(&self) -> Option<String64> {
        self.0.name.clone().map(String64)
    }

    pub fn set_image(&mut self, image: &ChunkableString) {
        self.0.image = Some(image.clone().into())
    }

    pub fn image(&self) -> Option<ChunkableString> {
        self.0.image.clone().map(ChunkableString)
    }
}

impl From<core::utils::MiniMetadataDetails> for MiniMetadataDetails {
    fn from(native: core::utils::MiniMetadataDetails) -> Self {
        Self(native)
    }
}

impl From<MiniMetadataDetails> for core::utils::MiniMetadataDetails {
    fn from(wasm: MiniMetadataDetails) -> Self {
        wasm.0
    }
}
