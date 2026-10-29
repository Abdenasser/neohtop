//! Best-effort XDG application matching; never execute a desktop entry's Exec.
use std::{
    collections::{HashMap, HashSet},
    io::Read,
    path::{Path, PathBuf},
    time::Instant,
};

#[derive(Debug, Default)]
pub(super) struct DesktopIndex {
    entries: HashMap<PathBuf, String>,
    refreshed: Option<Instant>,
}
impl DesktopIndex {
    #[cfg(target_os = "linux")]
    pub(super) fn icon_for(&mut self, executable: &Path) -> Option<PathBuf> {
        if self
            .refreshed
            .is_none_or(|t| t.elapsed() > std::time::Duration::from_secs(300))
        {
            self.refresh();
        }
        let executable = executable.canonicalize().ok()?;
        let icon = self.entries.get(&executable)?;
        if Path::new(icon).is_absolute() {
            return Path::new(icon).is_file().then(|| PathBuf::from(icon));
        }
        if icon.contains('/') || icon.contains('\\') {
            return None;
        }
        // The lookup library follows XDG icon inheritance, hicolor and pixmaps.
        freedesktop_icons::lookup(icon)
            .with_size(64)
            .with_theme(&icon_theme())
            .find()
    }
    fn refresh(&mut self) {
        let home = std::env::var_os("HOME").map(PathBuf::from);
        let mut dirs = Vec::new();
        if let Some(data) = std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| home.as_ref().map(|h| h.join(".local/share")))
        {
            dirs.push(data.join("applications"));
        }
        let data_dirs = std::env::var_os("XDG_DATA_DIRS")
            .unwrap_or_else(|| "/usr/local/share:/usr/share".into());
        dirs.extend(
            std::env::split_paths(&data_dirs)
                .filter(|p| p.is_absolute())
                .map(|p| p.join("applications")),
        );
        let mut entries = HashMap::new();
        let mut seen = HashSet::new();
        let mut budget = 4096;
        for dir in dirs {
            let mut files = Vec::new();
            collect_desktop_files(&dir, 2, &mut budget, &mut files);
            files.sort();
            for path in files {
                let id = path
                    .strip_prefix(&dir)
                    .ok()
                    .map(|p| p.to_string_lossy().replace('/', "-"));
                // User entries, including Hidden=true, override the same system entry.
                if !seen.insert(id.unwrap_or_else(|| path.to_string_lossy().into_owned())) {
                    continue;
                }
                if let Some(text) = read_entry(&path) {
                    if let Some((exec, icon)) = parse_entry(&text) {
                        if let Some(executable) = resolve_exec(&exec) {
                            entries.entry(executable).or_insert(icon);
                        }
                    }
                }
            }
        }
        self.entries = entries;
        self.refreshed = Some(Instant::now());
    }
}

fn collect_desktop_files(dir: &Path, depth: u8, budget: &mut usize, files: &mut Vec<PathBuf>) {
    if *budget == 0 {
        return;
    }
    let Ok(children) = std::fs::read_dir(dir) else {
        return;
    };
    for child in children.flatten() {
        if *budget == 0 {
            break;
        }
        *budget -= 1;
        let Ok(kind) = child.file_type() else {
            continue;
        };
        let path = child.path();
        if kind.is_dir() && depth > 0 {
            collect_desktop_files(&path, depth - 1, budget, files);
        } else if (kind.is_file() || kind.is_symlink())
            && path.extension().is_some_and(|e| e == "desktop")
        {
            files.push(path);
        }
    }
}
fn read_entry(path: &Path) -> Option<String> {
    let mut text = String::new();
    std::fs::File::open(path)
        .ok()?
        .take(65537)
        .read_to_string(&mut text)
        .ok()?;
    (text.len() <= 65536).then_some(text)
}
fn parse_entry(text: &str) -> Option<(String, String)> {
    let mut main = false;
    let mut exec = None;
    let mut icon = None;
    let mut application = false;
    let mut hidden = false;
    for line in text.lines().map(str::trim) {
        if line.starts_with('[') {
            main = line == "[Desktop Entry]";
            continue;
        }
        if !main || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        match key.trim() {
            "Type" => application = value.trim() == "Application",
            "Hidden" => hidden = value.trim() == "true",
            "Exec" => exec = exec_token(value.trim()),
            "Icon" => icon = unescape(value.trim()),
            _ => {}
        }
    }
    if hidden || !application {
        return None;
    }
    let icon = icon.filter(|s| !s.is_empty() && !s.contains('\0'))?;
    Some((exec?, icon))
}
fn unescape(value: &str) -> Option<String> {
    let mut chars = value.chars();
    let mut output = String::new();
    while let Some(c) = chars.next() {
        output.push(if c == '\\' {
            match chars.next()? {
                's' => ' ',
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                '\\' => '\\',
                _ => return None,
            }
        } else {
            c
        });
    }
    Some(output)
}
fn exec_token(value: &str) -> Option<String> {
    // Parse the executable token only. Flags, field codes and actions are not run.
    let decoded = unescape(value)?;
    let mut chars = decoded.chars().peekable();
    let mut token = String::new();
    let quoted = chars.peek() == Some(&'"');
    if quoted {
        chars.next();
    }
    let mut closed = !quoted;
    while let Some(c) = chars.next() {
        if quoted && c == '"' {
            closed = true;
            break;
        }
        if !quoted && c.is_whitespace() {
            break;
        }
        if c == '\\' {
            token.push(chars.next()?);
        } else {
            token.push(c);
        }
    }
    (closed && !token.is_empty() && !token.contains('%') && !token.contains('\0')).then_some(token)
}
fn resolve_exec(exec: &str) -> Option<PathBuf> {
    // Wrappers/interpreters may launch many unrelated apps. Do not attribute their
    // shared executable to one arbitrary desktop entry.
    let name = Path::new(exec).file_name()?.to_str()?;
    if [
        "env", "sh", "bash", "dash", "zsh", "fish", "python", "python3", "node", "ruby", "java",
        "flatpak", "snap",
    ]
    .contains(&name)
    {
        return None;
    }
    if Path::new(exec).is_absolute() {
        return Path::new(exec).canonicalize().ok().filter(|p| p.is_file());
    }
    if exec.contains('/') {
        return None;
    }
    std::env::split_paths(&std::env::var_os("PATH")?)
        .map(|p| p.join(exec))
        .find_map(|p| p.canonicalize().ok().filter(|p| p.is_file()))
}
fn icon_theme() -> String {
    let root = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")));
    root.and_then(|r| read_entry(&r.join("gtk-3.0/settings.ini")))
        .and_then(|text| {
            text.lines().find_map(|line| {
                line.trim()
                    .strip_prefix("gtk-icon-theme-name=")
                    .map(|v| v.trim().trim_matches('"').to_string())
            })
        })
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "hicolor".into())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn entries_match_primary_executable_not_actions() {
        let text="[Desktop Entry]\nType=Application\nExec=\"/opt/My App/bin/app\" --open %U\nIcon=org.example.App\n[Desktop Action Other]\nExec=/wrong\nIcon=wrong";
        assert_eq!(
            parse_entry(text),
            Some(("/opt/My App/bin/app".into(), "org.example.App".into()))
        );
        assert!(
            parse_entry("[Desktop Entry]\nType=Application\nHidden=true\nExec=app\nIcon=app")
                .is_none()
        );
        assert!(parse_entry("[Desktop Entry]\nType=Link\nExec=app\nIcon=app").is_none());
    }
    #[test]
    fn malformed_exec_never_becomes_a_shell_command() {
        assert_eq!(
            exec_token("\"/opt/app tool\" %F"),
            Some("/opt/app tool".into())
        );
        assert!(exec_token("\"unterminated").is_none());
        assert!(exec_token("%f --args").is_none());
        assert!(resolve_exec("/usr/bin/env").is_none());
        assert!(resolve_exec("../app").is_none());
    }
}
