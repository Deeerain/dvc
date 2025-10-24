use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Margin {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttom: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Left,
    Top,
    Right,
    Buttom,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioDevice {
    id: String,
    label: Option<String>,
}
