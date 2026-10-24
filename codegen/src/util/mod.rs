pub mod ts;
pub mod zip;

use std::fs;
use std::path::{Path, PathBuf};

use crate::error::Result;
use crate::gen::GenFile;

/// 把生成的文件写到目标目录（会自动创建父目录）。
///
/// 返回每个文件最终落盘的真实路径。
pub fn write_all(files: &[GenFile], root: &str) -> Result<Vec<PathBuf>> {
    let root = Path::new(root);
    let mut written = Vec::with_capacity(files.len());
    for f in files {
        let target = join(root, &f.path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&target, &f.content)?;
        written.push(target);
    }
    Ok(written)
}

/// 拼接路径并统一分隔符，`GenFile.path` 用 `/`，Windows 上要转成 `\`。
fn join<'a>(root: &Path, rel: &'a str) -> PathBuf {
    let mut p = root.to_path_buf();
    for part in rel.split('/') {
        if !part.is_empty() {
            p.push(part);
        }
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_flattens_separators() {
        assert_eq!(
            join(Path::new("d:/out"), "backend/src/modules/demo/mod.rs"),
            PathBuf::from("d:/out/backend/src/modules/demo/mod.rs")
        );
    }
}
