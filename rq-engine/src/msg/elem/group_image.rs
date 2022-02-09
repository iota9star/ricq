use std::fmt;

use bytes::Bytes;

use crate::pb::msg;
use crate::pb::msg::CustomFace;

#[derive(Default, Debug, Clone)]
pub struct GroupImage {
    pub image_id: String,
    pub file_id: i64,
    pub size: i32,
    pub width: i32,
    pub height: i32,
    pub md5: Bytes,
    pub url: String,
}

impl From<msg::CustomFace> for GroupImage {
    fn from(custom_face: CustomFace) -> Self {
        if custom_face.md5().is_empty() {
            return Self::default();
        }
        let url = if let Some(orig_url) = &custom_face.orig_url {
            format!("https://gchat.qpic.cn{}", orig_url)
        } else {
            format!(
                "https://gchat.qpic.cn/gchatpic_new/0/0-0-{}{}",
                calculate_image_resource_id(&custom_face.md5()[1..37], true),
                "/0?term=2"
            )
        };
        // guild image todo
        return Self {
            file_id: custom_face.file_id() as i64,
            image_id: custom_face.file_path().to_owned(),
            size: custom_face.size(),
            width: custom_face.width(),
            height: custom_face.height(),
            url,
            md5: Bytes::copy_from_slice(custom_face.md5()),
        };
    }
}

fn to_uuid(md5: &str) -> String {
    format!(
        "{}-{}-{}-{}-{}",
        &md5[0..8],
        &md5[8..12],
        &md5[12..16],
        &md5[16..20],
        &md5[20..32],
    )
}

fn calculate_image_resource_id(md5: &[u8], no_dash: bool) -> String {
    let mut r = "{".to_owned();
    let md5 = crate::hex::encode_hex(md5).to_uppercase();
    if no_dash {
        r.push_str(&md5);
    } else {
        r.push_str(&to_uuid(&md5));
    }
    r.push_str("}.png");
    r
}

impl fmt::Display for GroupImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[GroupImage: {}]", self.url)
    }
}