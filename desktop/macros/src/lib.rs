use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{DeriveInput, Result, parse_macro_input};

/// Procedural macro to generate preview functionality for GUI components.
///
/// Usage:
/// ```rust
/// #[derive(Default)]
/// #[Preview]
/// struct MyComponent {
///     // fields...
/// }
/// ```
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Preview(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_item = parse_macro_input!(input as DeriveInput);

    match generate_preview_impl(&input_item) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn generate_preview_impl(input: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    let component_name = &input.ident;
    // Generate unique identifiers to avoid conflicts
    let title_struct = Ident::new(
        &format!("__PreviewTitle_{}", component_name),
        Span::call_site(),
    );
    let run_fn = Ident::new(
        &format!("__preview_run_{}", component_name),
        Span::call_site(),
    );
    let config_static = Ident::new(
        &format!("__PREVIEW_CONFIG_{}", component_name),
        Span::call_site(),
    );

    let expanded = quote! {
        #input

        struct #title_struct;

        impl iced::application::Title<#component_name> for #title_struct {
            fn title(&self, _comp: &#component_name) -> String {
                format!("Previewing {}", std::any::type_name::<#component_name>())
            }
        }

        #[cfg(feature = "preview")]
        fn #run_fn() -> iced::Result {
            fn update_wrapper(state: &mut #component_name, message: <#component_name as crate::gui::Component>::Message) -> iced::Task<<#component_name as crate::gui::Component>::Message> {
                use crate::gui::Component;
                state.update(message)
            }

            fn view_wrapper(state: &#component_name) -> iced::Element<<#component_name as crate::gui::Component>::Message> {
                use crate::gui::Component;
                state.view()
            }

            iced::application(#title_struct, update_wrapper, view_wrapper).run()
        }

        #[cfg(feature = "preview")]
        #[linkme::distributed_slice(crate::gui::PREVIEW_TARGETS)]
        static #config_static: (&str, fn() -> iced::Result) = (
            concat!(module_path!(), "::", stringify!(#component_name)),
            #run_fn
        );

        #[cfg(feature = "preview")]
        #[test]
        fn preview() -> anyhow::Result<()> {
            crate::gui::run_preview(std::any::type_name::<#component_name>())
        }
    };

    Ok(expanded)
}
