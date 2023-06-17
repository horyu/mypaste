#![cfg(windows)]
use clipboard_win::{formats, get_clipboard, SysResult};

fn main() -> SysResult<()> {
    let data: String = get_clipboard(formats::Unicode)?;
    print!("{data}");
    Ok(())
}
