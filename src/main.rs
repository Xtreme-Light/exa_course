use crate::colours::Colour::{Black, Blue, Cyan, Green, Purple, Red, Yellow};
use crate::colours::Style;
use crate::Style::Plain;
use std::env;
use std::env::ArgsOs;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::fs::{FileType, Metadata};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::prelude;
use std::path::Path;
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
    files.iter().for_each(|e| {
        let file_name_os = e.file_name();
        let file_name: &OsStr = file_name_os.as_ref();
        // 获取文件类型
        let meta = match fs::metadata(e.path()) {
            Ok(meta) => meta,
            Err(e) => {
                eprintln!("无法获取到文件信息 {:?}", e);
                exit(1);
            }
        };
        // 权限部分和文件名字部分，分成两部分着色
        let colour = file_colour(&meta, file_name);
        println!(
            "{} {}",
            perm_str(&meta),
            colour.paint(file_name.to_str().unwrap().to_string())
        )
    });
}

fn file_colour(metadata: &Metadata, file_name: &OsStr) -> Style {
    if metadata.is_dir() {
        Blue.normal()
    } else if metadata.permissions().mode() & 0o111 == 0o111 {
        Green.normal()
    } else if file_name.as_bytes().ends_with(b"~") {
        Black.bold()
    } else {
        Plain
    }
}

fn perm_str(metadata: &Metadata) -> String {
    let permission = metadata.permissions().mode();
    format!(
        "{}{}{}{}{}{}{}{}{}{}",
        type_char(&metadata.file_type()),
        bit(permission, 0o100, String::from("r"), Yellow.bold()),
        bit(permission, 0o300, String::from("w"), Red.bold()),
        bit(permission, 0o700, String::from("x"), Green.bold()),
        bit(permission, 0o010, String::from("r"), Yellow.bold()),
        bit(permission, 0o030, String::from("w"), Red.bold()),
        bit(permission, 0o070, String::from("x"), Green.bold()),
        bit(permission, 0o001, String::from("r"), Yellow.bold()),
        bit(permission, 0o003, String::from("w"), Red.bold()),
        bit(permission, 0o007, String::from("x"), Green.bold()),
    )
}
fn bit(permission: u32, bit: u32, other: String, style: Style) -> String {
    if permission & bit == bit {
        style.paint(other)
    } else {
        Cyan.paint(String::from(r"-"))
    }
}
fn type_char(file_type: &FileType) -> String {
    use std::os::unix::fs::FileTypeExt;
    if file_type.is_dir() {
        Blue.paint(String::from("d"))
    } else if file_type.is_file() {
        String::from(".")
    } else if file_type.is_symlink() {
        Cyan.paint(String::from("l"))
    } else if file_type.is_block_device() {
        Purple.paint(String::from("s"))
    } else if file_type.is_char_device() {
        Yellow.paint(String::from("|"))
    } else if file_type.is_fifo() {
        String::from("f")
    } else {
        String::from("?")
    }
}
