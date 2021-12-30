use crate::colours::Colour::{Black, Blue, Cyan, Green, Purple, Red, Yellow};
use crate::colours::Style;
use crate::Style::Plain;
use std::env;
use std::env::ArgsOs;
use std::ffi::{OsStr, OsString};
use std::fmt::Formatter;
use std::fs;
use std::fs::{DirEntry, FileType, Metadata};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::os::unix::prelude;
use std::path::{Path, PathBuf};
use std::process::exit;

mod colours;

fn main() {
    let args: Vec<OsString> = env::args_os().skip(1).collect();
    if args.len() == 0 {
        unreachable!()
    }
    for arg in args.iter() {
        let arg_ref: &OsStr = arg.as_ref();
        list(Path::new(arg_ref));
    }
}
struct FileHolder<'a> {
    name: &'a OsStr,
    path: &'a PathBuf,
    meta: &'a Metadata,
}

impl<'a> std::fmt::Display for FileHolder<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            format!(
                "{}{}{}{}{}{}{}{}{}{}",
                type_char(&self.meta.file_type()),
                bit(self.meta.permissions().mode(), 0o100, b'r', Yellow.bold()),
                bit(self.meta.permissions().mode(), 0o200, b'w', Red.bold()),
                bit(self.meta.permissions().mode(), 0o400, b'x', Green.bold()),
                bit(self.meta.permissions().mode(), 0o010, b'r', Yellow.bold()),
                bit(self.meta.permissions().mode(), 0o020, b'w', Red.bold()),
                bit(self.meta.permissions().mode(), 0o040, b'x', Green.bold()),
                bit(self.meta.permissions().mode(), 0o001, b'r', Yellow.bold()),
                bit(self.meta.permissions().mode(), 0o002, b'w', Red.bold()),
                bit(self.meta.permissions().mode(), 0o004, b'x', Green.bold()),
            ),
            {
                let str_size =
                    format_bytes(self.meta.size(), 1024, &["B", "KiB", "MiB", "GiB", "TiB"]);
                if self.meta.is_dir() {
                    Green.normal()
                } else {
                    Green.bold()
                }
                .paint(str_size.as_bytes())
            },
            self.name.to_str().unwrap()
        )
    }
}
trait Column {
    fn display(&self, metadata: &Metadata, filename: String) -> String;
}
impl Column for std::fs::Permissions {
    fn display(&self, metadata: &Metadata, filename: String) -> String {
        // file_colour(metadata,filename.as_bytes()).paint(filename.as_bytes())
        format!(
            "{}{}{}{}{}{}{}{}{}{}",
            type_char(&metadata.file_type()),
            bit(self.mode(), 0o100, b'r', Yellow.bold()),
            bit(self.mode(), 0o300, b'w', Red.bold()),
            bit(self.mode(), 0o700, b'x', Green.bold()),
            bit(self.mode(), 0o010, b'r', Yellow.bold()),
            bit(self.mode(), 0o030, b'w', Red.bold()),
            bit(self.mode(), 0o070, b'x', Green.bold()),
            bit(self.mode(), 0o001, b'r', Yellow.bold()),
            bit(self.mode(), 0o003, b'w', Red.bold()),
            bit(self.mode(), 0o007, b'x', Green.bold()),
        )
    }
}
struct FileSize;
impl Column for FileSize {
    fn display(&self, metadata: &Metadata, filename: String) -> String {
        let str_size = format_bytes(metadata.size(), 1024, &["B  ", "KiB", "MiB", "GiB", "TiB"]);
        return if metadata.is_dir() {
            Green.normal()
        } else {
            Green.bold()
        }
        .paint(str_size.as_bytes());
    }
}
pub fn format_bytes(mut size: u64, kilo: u64, prefixes: &[&str]) -> String {
    let mut prefix = 0;
    while size > kilo {
        size /= kilo.clone();
        prefix += 1;
    }
    format!("{:.4} {}", size, prefixes[prefix])
}

pub fn list(path: &Path) {
    let dir = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("读取文件失败{:?}", e);
            exit(1);
        }
    };
    let mut files: Vec<_> = dir.map(|e| e.unwrap()).collect();
    files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    files.iter().for_each(|e: &DirEntry| {
        let mut buf = e.path();
        let meta = match fs::metadata(buf) {
            Ok(meta) => meta,
            Err(e) => {
                eprintln!("无法获取到文件信息 {:?}", e);
                exit(1);
            }
        };
        buf = e.path();
        let holder = FileHolder {
            name: &buf.file_name().unwrap(),
            path: &buf,
            meta: &meta,
        };
        println!("{}", holder);
    });
}

fn file_colour(metadata: &Metadata, bytes: &[u8]) -> Style {
    if metadata.is_dir() {
        Blue.normal()
    } else if metadata.permissions().mode() & 0o111 == 0o111 {
        Green.normal()
    } else if bytes.ends_with(b"~") {
        Black.bold()
    } else {
        Plain
    }
}

fn bit(permission: u32, bit: u32, other: u8, style: Style) -> String {
    if permission & bit == bit {
        style.paint(&[other])
    } else {
        Cyan.paint(b"-")
    }
}
fn type_char(file_type: &FileType) -> String {
    use std::os::unix::fs::FileTypeExt;
    if file_type.is_dir() {
        Blue.paint(&[b'd'])
    } else if file_type.is_file() {
        ".".to_string()
    } else if file_type.is_symlink() {
        Cyan.paint(&[b'l'])
    } else if file_type.is_block_device() {
        Purple.paint(&[b's'])
    } else if file_type.is_char_device() {
        Yellow.paint(&[b'|'])
    } else if file_type.is_fifo() {
        "f".to_string()
    } else {
        "?".to_string()
    }
}
