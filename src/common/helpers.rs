use crate::common::constants::{ENDPOINT_URL, USER_AGENTS};
use crate::common::types::DownloadLink;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use cipher::{BlockDecrypt, KeyInit};
use des::Des;
use rand::seq::SliceRandom;

pub async fn use_fetch<T>(
    call_name: &str,
    extra_params: Vec<(String, String)>,
    context: Option<&str>,
) -> Result<T, reqwest::Error>
where
    for<'de> T: serde::Deserialize<'de>,
{
    let client = reqwest::Client::new();
    let mut query = vec![
        ("__call".to_string(), call_name.to_string()),
        ("_format".to_string(), "json".to_string()),
        ("_marker".to_string(), "0".to_string()),
        ("api_version".to_string(), "4".to_string()),
        ("ctx".to_string(), context.unwrap_or("web6dot0").to_string()),
    ];

    query.extend(extra_params);

    let mut rng = rand::thread_rng();
    let user_agent = USER_AGENTS.choose(&mut rng).copied().unwrap_or(USER_AGENTS[0]);

    client
        .get(ENDPOINT_URL)
        .query(&query)
        .header(reqwest::header::USER_AGENT, user_agent)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json::<T>()
        .await
}

pub fn create_download_links(encrypted_media_url: &str) -> Vec<DownloadLink> {
    if encrypted_media_url.is_empty() {
        return vec![];
    }

    let encrypted_bytes = match STANDARD.decode(encrypted_media_url) {
        Ok(bytes) => bytes,
        Err(_) => return vec![],
    };

    let key_bytes = b"38346591";
    let cipher = match Des::new_from_slice(key_bytes) {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    let mut decrypted_bytes = encrypted_bytes;
    for block in decrypted_bytes.chunks_mut(8) {
        if block.len() == 8 {
            let mut arr = cipher::generic_array::GenericArray::clone_from_slice(block);
            cipher.decrypt_block(&mut arr);
            block.copy_from_slice(&arr);
        }
    }

    // Unpad PKCS5 padding
    let len = decrypted_bytes.len();
    if len > 0 {
        let padding_len = decrypted_bytes[len - 1] as usize;
        if padding_len > 0 && padding_len <= 8 {
            let mut is_valid = true;
            for i in (len - padding_len)..len {
                if decrypted_bytes[i] as usize != padding_len {
                    is_valid = false;
                    break;
                }
            }
            if is_valid {
                decrypted_bytes.truncate(len - padding_len);
            }
        }
    }

    let decrypted_str = match String::from_utf8(decrypted_bytes) {
        Ok(s) => s,
        Err(_) => return vec![],
    };

    let qualities = [
        ("_12", "12kbps"),
        ("_48", "48kbps"),
        ("_96", "96kbps"),
        ("_160", "160kbps"),
        ("_320", "320kbps"),
    ];

    qualities
        .iter()
        .map(|(suffix, bitrate)| DownloadLink {
            quality: bitrate.to_string(),
            url: decrypted_str.replace("_96", suffix),
        })
        .collect()
}

pub fn create_image_links(link: &str) -> Vec<DownloadLink> {
    if link.is_empty() {
        return vec![];
    }

    let qualities = ["50x50", "150x150", "500x500"];
    let cleaned_link = if link.starts_with("http://") {
        link.replacen("http://", "https://", 1)
    } else {
        link.to_string()
    };

    qualities
        .iter()
        .map(|quality| {
            let url = if cleaned_link.contains("150x150") {
                cleaned_link.replace("150x150", quality)
            } else if cleaned_link.contains("50x50") {
                cleaned_link.replace("50x50", quality)
            } else {
                cleaned_link.clone()
            };

            DownloadLink {
                quality: quality.to_string(),
                url,
            }
        })
        .collect()
}
