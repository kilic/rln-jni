use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::objects::{JClass, JString};
    use self::jni::sys::jstring;
    use self::jni::JNIEnv;
    use super::*;
    use rln::circuit::bench;
    
    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_rlnbench_RustRLN_runBench(
        env: JNIEnv,
        _: JClass,
        java_pattern: JString,
    ) -> jstring {
        let result = bench::run_rln_bn(32);
        let output = env
            .new_string(format!("{}", result.prover_time))
            .expect("Couldn't create java string!");

        output.into_inner()
    }
}