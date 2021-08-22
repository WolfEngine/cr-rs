#[cxx::bridge]
pub mod cr {

    #[derive(Debug)]
    pub enum failure {
        CR_NONE,              // No error
        CR_SEGFAULT,          // SIGSEGV / EXCEPTION_ACCESS_VIOLATION
        CR_ILLEGAL,           // illegal instruction (SIGILL) / EXCEPTION_ILLEGAL_INSTRUCTION
        CR_ABORT,             // abort (SIGBRT)
        CR_MISALIGN,          // bus error (SIGBUS) / EXCEPTION_DATATYPE_MISALIGNMENT
        CR_BOUNDS,            // EXCEPTION_ARRAY_BOUNDS_EXCEEDED
        CR_STACKOVERFLOW,     // EXCEPTION_STACK_OVERFLOW
        CR_STATE_INVALIDATED, // one or more global data section changed and does
        // not safely match basically a failure of
        // cr_plugin_validate_sections
        CR_BAD_IMAGE,       // The binary is not valid - compiler is still writing it
        CR_INITIAL_FAILURE, // Plugin version 1 crashed, cannot rollback
        CR_OTHER,           // Unknown or other signal,
        CR_USER = 0x100,
    }

    #[derive(Debug)]
    pub struct cr_plugin {
        pub p: *mut void_ptr,        // opaque pointer for internal cr data;
        pub userdata: *mut void_ptr, // may be used by the user to pass information between reloads;
        pub version: u32, // incremetal number for each succeded reload, starting at 1 for the
        // first load. **The version will change during a crash handling process**;
        pub failure: failure, // used by the crash protection system, will hold the last failure error
        // code that caused a rollback. See `cr_failure` for more info on possible values;
        pub next_version: u32, // `next_version` for use by the next reload.
        pub last_working_version: u32,
    }

    unsafe extern "C++" {

        include!("cr-rs/src/cxx/cr.hpp");

        //type
        pub type void_ptr;
        pub type cr_plugin;

        pub fn plugin_new() -> UniquePtr<cr_plugin>;
        pub fn plugin_open(pCtx: UniquePtr<cr_plugin>, pFullPath: String) -> bool;
        pub fn plugin_update(pCtx: UniquePtr<cr_plugin>, pReloadCheck: bool) -> i32;
        pub fn plugin_close() -> ();
        // pub fn set_temporary_path(fullpath: String) -> ();
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
    let ctx = unsafe { cr::new_plugin() };
    println!("{:?}", ctx);
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
