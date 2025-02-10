use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use winapi::shared::windef::HWND;
use winapi::um::errhandlingapi::{RaiseException, SetErrorMode};
use winapi::um::processthreadsapi::OpenProcessToken;
use winapi::um::shellapi::ShellExecuteW;
use winapi::um::winbase::SEM_FAILCRITICALERRORS;
use winapi::um::winnt::LPCWSTR;
use winapi::um::winuser::SW_HIDE;

fn run_as_admin() -> bool {
    let exe_path = match std::env::current_exe() {
        Ok(path) => path,
        Err(_) => return false,
    };

    let mut exe_path: Vec<u16> = exe_path.as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let operation: Vec<u16> = OsStr::new("runas\0").encode_wide().collect();

    unsafe {
        let result = ShellExecuteW(
            ptr::null_mut() as HWND,
            operation.as_ptr() as LPCWSTR,
            exe_path.as_ptr() as LPCWSTR,
            ptr::null(),
            ptr::null(),
            SW_HIDE
        );

        result as i32 > 32
    }
}

fn is_elevated() -> bool {
    unsafe {
        let mut token = ptr::null_mut();
        if OpenProcessToken(
            winapi::um::processthreadsapi::GetCurrentProcess(),
            winapi::um::winnt::TOKEN_QUERY,
            &mut token
        ) == 0 {
            return false;
        }

        let mut elevation: winapi::um::winnt::TOKEN_ELEVATION = std::mem::zeroed();
        let mut size = 0;
        let ret = winapi::um::securitybaseapi::GetTokenInformation(
            token,
            winapi::um::winnt::TokenElevation,
            &mut elevation as *mut _ as *mut _,
            size_of::<winapi::um::winnt::TOKEN_ELEVATION>() as u32,
            &mut size
        );

        winapi::um::handleapi::CloseHandle(token);
        ret != 0 && elevation.TokenIsElevated != 0
    }
}

fn main() {
    if !is_elevated() {
        let _ = run_as_admin();
        return;
    }
    unsafe {
        SetErrorMode(SEM_FAILCRITICALERRORS);

        RaiseException(0xDEADDEAD, 0, 0, ptr::null());
    }
}
