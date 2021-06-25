use anyhow::{anyhow, bail, Result};
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::mem;
use std::path::Path;
use winapi::um::{
    fileapi::{FindClose, FindFirstFileW, FindNextFileW},
    handleapi::INVALID_HANDLE_VALUE,
    minwinbase::WIN32_FIND_DATAW,
    winnt::FILE_ATTRIBUTE_DIRECTORY,
};

fn read_dir<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    if path.as_ref().is_file() {
        bail!("Path is not directory");
    }
    let path = path.as_ref().join("*");
    let path = path.to_str().ok_or(anyhow!("No path str"))?;

    let mut fd = unsafe { mem::MaybeUninit::<WIN32_FIND_DATAW>::zeroed().assume_init() };
    let handle = unsafe { FindFirstFileW(encode(&path).as_ptr(), &mut fd) };
    if handle == INVALID_HANDLE_VALUE {
        bail!("Invalid handle value");
    }
    let mut v = vec![];
    while unsafe { FindNextFileW(handle, &mut fd) } != 0 {
        if fd.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY == 0 {
            let file_name = decode(&fd.cFileName);
            v.push(file_name);
        }
    }
    unsafe { FindClose(handle) };
    Ok(v)
}

fn encode(source: &str) -> Vec<u16> {
    source.encode_utf16().chain(Some(0)).collect()
}
fn decode(source: &[u16]) -> String {
    let mut s = String::with_capacity(260);
    for c in decode_utf16(source.iter().take_while(|&c| c != &0).cloned()) {
        let c = c.unwrap_or(REPLACEMENT_CHARACTER);
        s.push(c);
    }
    s.shrink_to_fit();
    s
}
