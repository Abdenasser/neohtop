//! On-demand, bounded system icon lookup. Never inspect arbitrary frontend paths.
#[cfg(any(target_os = "linux", test))]
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
mod linux;

use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::{Duration, Instant, SystemTime},
};

pub const MAX_BATCH: usize = 32;
const MAX_CACHE: usize = 256;
const MAX_IMAGE_BYTES: usize = 128 * 1024;

#[derive(Debug, Deserialize)]
pub struct IconRequest {
    pub pid: u32,
    pub start_time: u64,
}
#[derive(Debug, Serialize)]
pub struct IconReply {
    pub pid: u32,
    pub start_time: u64,
    pub icon: Option<String>,
}

#[derive(Debug)]
struct CachedIcon {
    modified: Option<SystemTime>,
    expires: Instant,
    used: Instant,
    icon: Option<String>,
}
#[derive(Debug, Default)]
pub struct IconCache {
    entries: HashMap<PathBuf, CachedIcon>,
    #[cfg(target_os = "linux")]
    desktop: linux::DesktopIndex,
}

/// Choose the owning app, not a nested helper bundle or the first word of a name.
#[cfg(any(target_os = "macos", test))]
fn outer_app_bundle(executable: &Path) -> Option<PathBuf> {
    executable
        .ancestors()
        .filter(|p| p.extension().is_some_and(|e| e.eq_ignore_ascii_case("app")))
        .last()
        .map(Path::to_path_buf)
}

pub fn process_target(sys: &sysinfo::System, request: &IconRequest) -> Option<PathBuf> {
    sys.process(sysinfo::Pid::from_u32(request.pid))
        .filter(|p| p.start_time() == request.start_time)
        .and_then(|p| p.exe())
        .map(Path::to_path_buf)
}

impl IconCache {
    pub fn resolve(&mut self, executable: &Path) -> Option<String> {
        #[cfg(target_os = "macos")]
        let source = outer_app_bundle(executable)?;
        #[cfg(target_os = "windows")]
        let source = executable.to_path_buf();
        #[cfg(target_os = "linux")]
        let source = self.desktop.icon_for(executable)?;
        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        return None;
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
        {
            let modified = source.metadata().ok()?.modified().ok();
            let now = Instant::now();
            if let Some(entry) = self.entries.get_mut(&source) {
                if entry.modified == modified && entry.expires > now {
                    entry.used = now;
                    return entry.icon.clone();
                }
            }
            let icon = read_icon(&source);
            if self.entries.len() >= MAX_CACHE {
                if let Some(oldest) = self
                    .entries
                    .iter()
                    .min_by_key(|(_, v)| v.used)
                    .map(|(k, _)| k.clone())
                {
                    self.entries.remove(&oldest);
                }
            }
            self.entries.insert(
                source,
                CachedIcon {
                    modified,
                    expires: now + Duration::from_secs(if icon.is_some() { 300 } else { 60 }),
                    used: now,
                    icon: icon.clone(),
                },
            );
            icon
        }
    }
}

#[cfg(target_os = "macos")]
fn read_icon(path: &Path) -> Option<String> {
    // AppKit's temporary autoreleased objects must not accumulate on worker threads.
    objc2::rc::autoreleasepool(|_| encode_native_icon(path))
}
#[cfg(target_os = "windows")]
fn read_icon(path: &Path) -> Option<String> {
    encode_native_icon(path)
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn encode_native_icon(path: &Path) -> Option<String> {
    let icon = file_icon_provider::get_file_icon(path, 64).ok()?;
    #[cfg(target_os = "macos")]
    let mut icon = icon;
    if icon.width != 64 || icon.height != 64 || icon.pixels.len() != 64 * 64 * 4 {
        return None;
    }
    // AppKit's bitmap is premultiplied; PNG requires straight alpha.
    #[cfg(target_os = "macos")]
    for pixel in icon.pixels.chunks_exact_mut(4) {
        let alpha = u32::from(pixel[3]);
        if alpha > 0 && alpha < 255 {
            for channel in &mut pixel[..3] {
                *channel = ((u32::from(*channel) * 255 + alpha / 2) / alpha).min(255) as u8;
            }
        }
    }
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut bytes, icon.width, icon.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().ok()?;
        writer.write_image_data(&icon.pixels).ok()?;
    }
    (bytes.len() <= MAX_IMAGE_BYTES)
        .then(|| format!("data:image/png;base64,{}", STANDARD.encode(bytes)))
}

#[cfg(target_os = "linux")]
fn read_icon(path: &Path) -> Option<String> {
    use std::io::Read;
    // Read a bounded icon image only. Desktop entries and commands are never executed.
    let mut bytes = Vec::new();
    std::fs::File::open(path)
        .ok()?
        .take((MAX_IMAGE_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .ok()?;
    if bytes.is_empty() || bytes.len() > MAX_IMAGE_BYTES {
        return None;
    }
    let mime = if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        "image/png"
    } else if path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("svg"))
        && std::str::from_utf8(&bytes).ok()?.contains("<svg")
    {
        "image/svg+xml"
    } else {
        return None;
    };
    Some(format!("data:{mime};base64,{}", STANDARD.encode(bytes)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn process_identity_rejects_a_reused_pid() {
        let sys = sysinfo::System::new_all();
        let pid = std::process::id();
        let current = sys.process(sysinfo::Pid::from_u32(pid)).unwrap();
        assert!(process_target(
            &sys,
            &IconRequest {
                pid,
                start_time: current.start_time()
            }
        )
        .is_some());
        assert!(process_target(
            &sys,
            &IconRequest {
                pid,
                start_time: current.start_time().saturating_add(1)
            }
        )
        .is_none());
        assert!(process_target(
            &sys,
            &IconRequest {
                pid: 0,
                start_time: 0
            }
        )
        .is_none());
    }
    #[test]
    fn nested_helpers_use_owning_bundle() {
        assert_eq!(
            outer_app_bundle(Path::new(
                "/Applications/My App.app/Contents/Frameworks/Helper.app/Contents/MacOS/Helper"
            )),
            Some(PathBuf::from("/Applications/My App.app"))
        );
        assert_eq!(outer_app_bundle(Path::new("/usr/bin/python")), None);
        assert_eq!(
            outer_app_bundle(Path::new("/opt/not.appdata/bin/tool")),
            None
        );
    }
}
