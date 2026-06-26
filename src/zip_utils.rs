//! ZIP archive creation and extraction using the `zip` crate with `zlib-rs`
//! as the deflate backend (via `flate2`'s `zlib-rs` feature).
//!
//! Thin wrapper providing `zip_dir` and `unzip_to_dir` conveniences.

use std::fs::{self, File};
use std::io::{self, Cursor, Write};
use std::path::Path;

pub use zip::result::ZipError;

/// Create a ZIP file at `dst` containing all files under `src_dir`.
pub fn zip_dir(src_dir: &Path, dst: &Path) -> Result<(), ZipError> {
    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    let file = File::create(dst)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default();

    let src_abs = src_dir.canonicalize()?;
    let entries: Vec<_> = walkdir::WalkDir::new(&src_abs)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .collect();

    for entry in &entries {
        let rel = entry.strip_prefix(&src_abs).unwrap();
        let name = rel.to_str().unwrap().replace('\\', "/");

        // Skip the root directory itself
        if name.is_empty() {
            continue;
        }

        if entry.is_dir() {
            zip.add_directory(&name, options)?;
        } else {
            zip.start_file(&name, options)?;
            let data = fs::read(entry)?;
            zip.write_all(&data)?;
        }
    }
    zip.finish()?;
    Ok(())
}

/// Extract ZIP file from in-memory bytes to `dst_dir`.
pub fn unzip_to_dir(data: &[u8], dst_dir: &Path) -> Result<(), ZipError> {
    use zip::ZipArchive;

    let cursor = Cursor::new(data);
    let mut archive = ZipArchive::new(cursor)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(p) => dst_dir.join(p),
            None => continue,
        };

        if file.is_dir() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

// ── tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let tmp = tempfile::TempDir::new().unwrap();
        let src = tmp.path().join("src");
        fs::create_dir(&src).unwrap();
        fs::write(src.join("hello.txt"), b"hello world").unwrap();

        let zip_path = tmp.path().join("test.zip");
        zip_dir(&src, &zip_path).unwrap();

        let data = fs::read(&zip_path).unwrap();
        let out = tmp.path().join("out");
        unzip_to_dir(&data, &out).unwrap();

        let extracted = fs::read_to_string(out.join("hello.txt")).unwrap();
        assert_eq!(extracted, "hello world");
    }

    #[test]
    fn roundtrip_nested() {
        let tmp = tempfile::TempDir::new().unwrap();
        let src = tmp.path().join("src");
        let sub = src.join("a").join("b");
        fs::create_dir_all(&sub).unwrap();
        fs::write(src.join("root.txt"), b"root").unwrap();
        fs::write(sub.join("deep.txt"), b"deep").unwrap();

        let zip_path = tmp.path().join("test.zip");
        zip_dir(&src, &zip_path).unwrap();

        let data = fs::read(&zip_path).unwrap();
        let out = tmp.path().join("out");
        unzip_to_dir(&data, &out).unwrap();

        assert_eq!(fs::read_to_string(out.join("root.txt")).unwrap(), "root");
        assert_eq!(
            fs::read_to_string(out.join("a/b/deep.txt")).unwrap(),
            "deep"
        );
    }
}
