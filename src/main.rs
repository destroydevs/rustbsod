use std::ptr::null_mut;
use winapi::shared::ntdef::{NTSTATUS, PULONG, ULONG};
use winapi::um::winnt::{BOOLEAN, PBOOLEAN};

#[link(name = "ntdll")]
extern "system" {

    // extern RtlAdjustPrivilege
    fn RtlAdjustPrivilege(
        Privilege: ULONG,
        Enable: BOOLEAN,
        CurrentThread: BOOLEAN,
        Enabled: PBOOLEAN,
    ) -> NTSTATUS;

    // extern NtRaiseHardError
    fn NtRaiseHardError(
        ErrorStatus: NTSTATUS,
        NumberOfParameters: ULONG,
        UnicodeStringParameterMask: ULONG,
        Parameters: PULONG,
        ValidResponseOptions: ULONG,
        Response: &mut ULONG,
    ) -> NTSTATUS;
}

fn main() {
    unsafe {
        let mut enabled: BOOLEAN = 0; // response of RtlAdjustPrivilege

        // gives the application the right to turn off the system
        RtlAdjustPrivilege(19, 1, 0, &mut enabled);

        let mut response: ULONG = 0; // response of NtRaiseHardError

        // pub const STATUS_ASSERTION_FAILURE: DWORD = 0xC0000420
        // used information https://github.com/toxidworm/SimpleBSOD/blob/main/BSOD.cpp

        NtRaiseHardError(
            0xC0000420u32 as NTSTATUS, // code of STATUS_ASSERTION_FAILURE
            0,
            0,
            null_mut(),
            6,
            &mut response,
        );
    }
}
