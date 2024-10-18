use std::borrow::Cow;

use proc_macro2::TokenStream;
use proc_macro2_diagnostics::Diagnostic;
use quote::quote;
use syn::ItemFn;

use crate::args::{err_on_unknown_args, pop_bool_arg};

pub(crate) struct StructOps {
    item: ItemFn,
    sleepable: bool,
}

impl StructOps {
    pub(crate) fn parse(attrs: TokenStream, item: TokenStream) -> Result<Self, Diagnostic> {
        let item = syn::parse2(item)?;
        let mut args = syn::parse2(attrs)?;
        let sleepable = pop_bool_arg(&mut args, "sleepable");
        err_on_unknown_args(&args)?;
        Ok(StructOps { item, sleepable })
    }

    pub(crate) fn expand(&self) -> TokenStream {
        let section_prefix = if self.sleepable {
            "struct_ops.s"
        } else {
            "struct_ops"
        };
        let section_name: Cow<'_, _> = format!("{}", section_prefix).into();
        let fn_vis = &self.item.vis;
        let fn_name = self.item.sig.ident.clone();
        let item = &self.item;
        quote! {
            #[no_mangle]
            #[link_section = #section_name]
            #fn_vis fn #fn_name(ctx: *mut ::core::ffi::c_void) -> i32 {
                return #fn_name(::aya_ebpf::programs::StructOpsContext::new(ctx));

                #item
            }
        }
    }
}
