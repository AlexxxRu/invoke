use proc_macro::TokenStream;
use quote::format_ident;

use crate::invoke_ctx::InvokeCtx;

pub fn generate_meta(ctx: &InvokeCtx) -> TokenStream {
    let name_ident = format_ident!("{}", ctx.name);

    let fns: Vec<_> = ctx.immutable.iter().map(|v| &v.name_ident).collect();
    let fns_names: Vec<_> = ctx
        .immutable
        .iter()
        .map(|v| format!("{}::{}", ctx.name, v.name))
        .collect();
    let idx: Vec<_> = ctx.immutable.iter().map(|v| v.id).collect();

    quote::quote! {
        impl ::invoke::InvokeMeta for #name_ident {
            fn get_method_id_raw_ptr(func_ptr: *const std::ffi::c_void) -> Option<u16> {
                #(
                    if std::ptr::eq(func_ptr, &Self::#fns as *const _ as *const std::ffi::c_void) {
                        return Some(#idx);
                    }
                )*

                None
            }

            fn get_method_id(name: &str) -> Option<u16> {
                #[allow(unreachable_code)]
                match name {
                    #(#fns_names => Some(#idx),)*
                    _ => None
                }
            }
        }
    }
    .into()
}
