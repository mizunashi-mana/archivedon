use std::path::{Path, PathBuf};

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use once_cell::sync::Lazy;
use regex::Regex;

pub fn to_url_safe(base: &Path, domain: &str, url_path: &str, ext: &str) -> PathBuf {
    let mut path = base.join(component_to_url_safe(domain));
    let mut last_component_opt = None;
    for component in url_path.split("/") {
        if component.is_empty() {
            continue;
        }
        if let Some(last_component) = last_component_opt {
            path = path.join(component_to_url_safe(last_component))
        }
        last_component_opt = Some(component)
    }

    match last_component_opt {
        None => path = path.join(format!(".{ext}")),
        Some(last_component) => {
            path = path.join(component_to_url_safe_with_safe_ext(last_component, ext))
        }
    }

    path
}

static RE_SAFE_COMPONENT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([a-zA-Z0-9-_:@]|\.[a-zA-Z0-9-_:@])[a-zA-Z0-9-_.:@]*$").unwrap());

pub fn component_to_url_safe(component: &str) -> String {
    let main = if RE_SAFE_COMPONENT.is_match(component) {
        component.to_string()
    } else {
        format!("..{}", URL_SAFE_NO_PAD.encode(component))
    };

    format!("_{main}")
}

pub fn component_to_url_safe_with_safe_ext(component: &str, ext: &str) -> String {
    let main = component_to_url_safe(component);
    format!("{main}.{ext}")
}
