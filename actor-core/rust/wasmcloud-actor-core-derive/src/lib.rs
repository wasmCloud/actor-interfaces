use proc_macro::TokenStream;
use quote::quote;

/// Marks the actor entry-point function to be executed by wasmcloud
///
/// # Examples
/// ```
/// #[actor::init]
/// fn init() {
///     // Register message handlers...
/// }
/// ```
#[allow(clippy::needless_doctest_main)]
#[proc_macro_attribute]
pub fn init(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(item as syn::ItemFn);
    let attrs = &input.attrs;

    let sig = &mut input.sig;
    let body = &input.block;

    if sig.asyncness.is_some() {
        return syn::Error::new_spanned(
            sig.fn_token,
            "the async keyword cannot be used within actors",
        )
        .to_compile_error()
        .into();
    }

    sig.asyncness = None;

    (quote! {
        #[doc(hidden)]
        fn default_health(_msg: wasmcloud_actor_core::HealthCheckRequest) -> wapc_guest::HandlerResult<wasmcloud_actor_core::HealthCheckResponse> {
            Ok(wasmcloud_actor_core::HealthCheckResponse::healthy())
        }

        #(#attrs)*
        #[no_mangle]
        pub fn wapc_init() {
            wasmcloud_actor_core::Handlers::register_health_request(default_health);
            #body
        }
    })
    .into()
}
