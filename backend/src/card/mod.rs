use serde::{Deserialize, Serialize};

mod ygo_card;
pub mod response_card;

#[derive(Debug, Deserialize, Serialize)]
enum LinkMarkers {
    Top,
    Right,
    Bottom,
    Left,
    #[serde(alias="Top-Left")]  TopLeft,
    #[serde(alias="Top-Right")] TopRight,
    #[serde(alias="Bottom-Right")]  BottomRight,
    #[serde(alias="Bottom-Left")]   BottomLeft
}
#[derive(Debug, Deserialize, Serialize)]
                // deserialize FROM uppercase, serialize TO PascalCase
#[serde(rename_all(deserialize="UPPERCASE", serialize="PascalCase"))]
enum Attribute {    
    Fire,
    Water,
    Earth,
    Wind,
    Dark,
    Light,
    Divine
}

// change to path
#[derive(Clone, Debug, Deserialize, Serialize)]
struct ImgLinks {
    #[serde(alias="image_url_small")]
    small: String,
    #[serde(alias="image_url_cropped")]
    cropped: String
}