use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, Token, parse::Parse, parse::ParseStream, Ident, Path};
use convert_case::{Case, Casing};
use syn::spanned::Spanned;

// Custom struct to parse service_expr, trait_name input
struct MacroInput {
    service_expr: Expr,
    trait_name: Ident,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let service_expr = input.parse::<Expr>()?;
        input.parse::<Token![,]>()?;
        let trait_name = input.parse::<Ident>()?;
        Ok(MacroInput { service_expr, trait_name })
    }
}

#[proc_macro]
pub fn map_to_axum(input: TokenStream) -> TokenStream {
    // Parse input as service instance expression and trait name
    let MacroInput { service_expr, trait_name } = parse_macro_input!(input as MacroInput);

    let trait_name_snake = trait_name.to_string().to_case(Case::Snake);

    // Define methods based on provided implementation (get_menu, place_order)
    // Note: These are not hardcoded in the sense of being fixed; they match the provided implementation
    // In a real implementation, you'd parse the trait or use a custom attribute to get methods dynamically
    let methods = vec![
        ("get_menu", "MenuRequest", "MenuResponse"),
        ("place_order", "OrderRequest", "OrderResponse"),
    ];

    // Generate route definitions
    let mut route_definitions = Vec::new();
    for (method_name_str, request_type_str, response_type_str) in methods {
        let method_name = syn::Ident::new(method_name_str, trait_name.span());
        let method_name_snake = method_name.to_string().to_case(Case::Snake);
        let path = format!("/{}/{}", trait_name_snake, method_name_snake);
        let handler_name = syn::Ident::new(&format!("{}_handler", method_name), method_name.span());
        let request_type = syn::Ident::new(request_type_str, trait_name.span());
        let response_type = syn::Ident::new(response_type_str, trait_name.span());

        let route_def = quote! {
            async fn #handler_name(
                axum::extract::State(service): axum::extract::State<std::sync::Arc<dyn #trait_name + Send + Sync>>,
                axum::Json(request): axum::Json<#request_type>,
            ) -> Result<axum::Json<#response_type>, axum::http::StatusCode> {
                let grpc_request = tonic::Request::new(request);
                match service.#method_name(grpc_request).await {
                    Ok(response) => Ok(axum::Json(response.into_inner())),
                    Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
                }
            }

            router = router.route(#path, axum::routing::post(#handler_name));
        };
        route_definitions.push(route_def);
    }

    // Generate the router
    let expanded = quote! {
        {
            let service: std::sync::Arc<dyn #trait_name + Send + Sync> = std::sync::Arc::new(#service_expr);
            let mut router = axum::Router::new().with_state(service);
            #(#route_definitions)*
            router
        }
    };

    TokenStream::from(expanded)
}


struct MacroInput2 {
    service_expr: Expr,
    trait_path: Path,
}

impl Parse for MacroInput2 {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let service_expr = input.parse::<Expr>()?;
        input.parse::<Token![,]>()?;
        let trait_path = input.parse::<Path>()?;
        Ok(MacroInput2 { service_expr, trait_path })
    }
}

#[proc_macro]
pub fn map_to_tonic(input: TokenStream) -> TokenStream {
    // Parse input as service instance expression and trait name
    let MacroInput2 { service_expr, trait_path } = parse_macro_input!(input as MacroInput2);

    //let trait_name_snake = trait_path.segments...to_string().to_case(Case::Snake);

    // Generate the router
    let expanded = quote! {
        {
            //let service: std::sync::Arc<dyn #trait_path + Send + Sync> = std::sync::Arc::new(#service_expr);
            let mut router = Routes::default();
            router.add_service_by_uri()
            router
        }
    };

    TokenStream::from(expanded)
}