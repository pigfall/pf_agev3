use proc_macro::{TokenStream};
use quote::{quote};
use syn::{parse_macro_input, ItemFn};

pub fn build(_attr: TokenStream,input: TokenStream)->TokenStream{
    let item_ast = parse_macro_input!(input as ItemFn);
    let main_fn_name = &item_ast.sig.ident;

    let output_tks = quote!{
        use std::os::raw::c_void;
        //use pf_engine::bevy::prelude::{App};

        #[cfg(target_os="windows")]
        compile_error!("unsupprot windows");

        #[cfg(target_os="android")]
        #[no_mangle]
        unsafe extern "C" fn ANativeActivity_onCreate(
            activity_raw_ptr: *mut c_void,
            saved_state_raw_ptr: *mut c_void,
            saved_state_size: usize,
            ){

            pf_engine::platform::android::game_main(
                #main_fn_name,
                activity_raw_ptr,
                saved_state_raw_ptr,
                saved_state_size,
                );
        }
        
        #item_ast
    };

    return output_tks.into();

}
