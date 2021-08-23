pub mod cr {
    #[derive(Debug, Clone, Copy)]
    #[repr(C)]
    pub enum Failure {
        CrNone,             // No error
        CrSegfault,         // SIGSEGV / EXCEPTION_ACCESS_VIOLATION
        CrIllegal,          // illegal instruction (SIGILL) / EXCEPTION_ILLEGAL_INSTRUCTION
        CrAbort,            // abort (SIGBRT)
        CrMisalign,         // bus error (SIGBUS) / EXCEPTION_DATATYPE_MISALIGNMENT
        CrBounds,           // EXCEPTION_ARRAY_BOUNDS_EXCEEDED
        CrStackoverflow,    // EXCEPTION_STACK_OVERFLOW
        CrStateInvalidated, // one or more global data section changed and does
        // not safely match basically a failure of
        // cr_plugin_validate_sections
        CrBadImage,       // The binary is not valid - compiler is still writing it
        CrInitialFailure, // Plugin version 1 crashed, cannot rollback
        CrOther,          // Unknown or other signal,
        CrUser = 0x100,
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(C)]
    pub struct cr_plugin {
        pub p: *mut libc::c_void,        // opaque pointer for internal cr data;
        pub userdata: *mut libc::c_void, // may be used by the user to pass information between reloads;
        pub version: u32, // incremetal number for each succeded reload, starting at 1 for the first load. **The version will change during a crash handling process**;
        pub failure: Failure, // used by the crash protection system, will hold the last failure error code that caused a rollback. See `cr_failure` for more info on possible values;
        pub next_version: u32, // for use by the next reload.
        pub last_working_version: u32, // used by the lasted reload.
    }

    extern "C" {
        pub fn cr_plugin_open(pCtx: cr_plugin, pFullPath: *const libc::c_char) -> bool;
        pub fn cr_plugin_update(pCtx: cr_plugin, pReloadCheck: bool) -> i32;
        pub fn cr_plugin_close(pCtx: cr_plugin) -> ();
    }

    pub fn plugin_new() -> cr_plugin {
        unsafe {
            let ctx: cr_plugin = std::mem::MaybeUninit::zeroed().assume_init();
            ctx
        }
    }

    pub fn plugin_open(p_ctx: cr_plugin, p_full_path: String) -> bool {
        unsafe { cr_plugin_open(p_ctx, p_full_path.as_ptr() as *const libc::c_char) }
    }

    pub fn plugin_update(p_ctx: cr_plugin, p_reload_check: bool) -> i32 {
        unsafe { cr_plugin_update(p_ctx, p_reload_check) }
    }

    pub fn plugin_close(p_ctx: cr_plugin) -> () {
        unsafe {
            cr_plugin_close(p_ctx);
        }
    }
}

// fn inner_main(_name: &str) -> io::Result<PathBuf> {
//     let mut dir = std::env::current_exe()?;
//     dir.pop();
//     dir.push(_name);
//     Ok(dir)
// }

#[test]
fn tests() -> () {

    // let plugin = unsafe { cr::plugin_new() };
    // println!("{:?}", plugin);

    // unsafe {
    //
    // };

    // let _dll_path = inner_main("basic_guest.dll").expect("Couldn't");
    // println!("{}", _dll_path.display());

    // let _temporary_path = inner_main("temp\\").expect("Couldn't");
    // println!("{}", _temporary_path.display());

    // // Check if a folder "temp" exists. If not, creates it.
    // fs::create_dir_all(_temporary_path.as_path());

    // let _is_exist_dll_folder = Path::new(_dll_path.as_path()).exists();
    // if _is_exist_dll_folder == false{
    //     println!("Warning! Could not find the file {}. Opening the plugin and loading the library may fail.", _dll_path.display());
    // }

    // let _plugin_temp_path: String = _temporary_path.as_path().display().to_string();
    // let _plugin_path: String = _dll_path.as_path().display().to_string();

    // let _open_res = unsafe { cr::plugin_open(&*_context, _plugin_path) };

    // if _open_res == false {
    //     println!("opening plugin failed");
    //     return;
    // }

    // unsafe { cr::set_temporary_path(_plugin_temp_path); }

    // at the end do not forget to cleanup the plugin context, as it needs to
    // allocate some memory to track internal and plugin states
    // unsafe {
    //     cr::plugin_close();
    // }
}
