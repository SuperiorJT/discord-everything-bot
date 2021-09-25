use serde::{Deserialize, Serialize};
use twilight_model::channel::embed::{EmbedField, EmbedImage, EmbedProvider, EmbedThumbnail};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EmbedAuthor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Into<twilight_model::channel::embed::EmbedAuthor> for EmbedAuthor {
    fn into(self) -> twilight_model::channel::embed::EmbedAuthor {
        twilight_model::channel::embed::EmbedAuthor {
            icon_url: self.image,
            name: self.name,
            proxy_icon_url: None,
            url: self.url,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EmbedFooter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl Into<twilight_model::channel::embed::EmbedFooter> for EmbedFooter {
    fn into(self) -> twilight_model::channel::embed::EmbedFooter {
        twilight_model::channel::embed::EmbedFooter {
            icon_url: self.image,
            proxy_icon_url: None,
            text: self.text.unwrap_or("".to_string()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Embed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<EmbedField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<EmbedProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Into<twilight_model::channel::embed::Embed> for Embed {
    fn into(self) -> twilight_model::channel::embed::Embed {
        twilight_model::channel::embed::Embed {
            author: self.author.map(|a| a.into()),
            color: self.color.map(css_to_u32),
            description: self.description,
            fields: self.fields,
            footer: self.footer.map(|f| f.into()),
            image: self.image.map(|i| EmbedImage {
                height: None,
                proxy_url: None,
                url: Some(i),
                width: None,
            }),
            provider: self.provider,
            thumbnail: self.thumbnail.map(|t| EmbedThumbnail {
                height: None,
                proxy_url: None,
                url: Some(t),
                width: None,
            }),
            timestamp: self.timestamp,
            title: self.title,
            url: self.url,
            kind: "rich".to_string(),
            video: None,
        }
    }
}

fn css_to_u32(color: String) -> u32 {
    let bytes = csscolorparser::parse(color.as_str())
        .unwrap_or(csscolorparser::Color::from_rgb(0f64, 0f64, 0f64))
        .rgba_u8();
    (u32::from(bytes.0) << 16) + (u32::from(bytes.1) << 8) + u32::from(bytes.2)
}

#[cfg(test)]
mod tests {
    use crate::models::embed::css_to_u32;

    #[test]
    fn convert_white_to_u32() {
        let result = css_to_u32("rgba(255, 255, 255, 255)".to_string());
        println!("{}", result);
        assert!(result == 16777215);
    }
}
