use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

fn nome_do_span(attr: TokenStream) -> syn::Result<Option<String>> {
    if attr.is_empty() {
        return Ok(None);
    }
    let nv: syn::MetaNameValue = syn::parse(attr)?;
    if !nv.path.is_ident("name") {
        return Err(syn::Error::new_spanned(
            nv.path,
            "atributo esperado: #[aop_span] ou #[aop_span(name = \"...\")]",
        ));
    }
    match nv.value {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(s),
            ..
        }) => Ok(Some(s.value())),
        outro => Err(syn::Error::new_spanned(outro, "name deve ser uma string")),
    }
}

/// Instrumenta uma função síncrona com um span `INFO` de mesmo nome.
///
/// `#[aop_span]` usa o nome da função; `#[aop_span(name = "...")]` renomeia.
#[proc_macro_attribute]
pub fn aop_span(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as ItemFn);

    let nome = match nome_do_span(attr) {
        Ok(nome) => nome,
        Err(erro) => return erro.to_compile_error().into(),
    };
    let span_name = nome.unwrap_or_else(|| func.sig.ident.to_string());

    let bloco = (*func.block).clone();
    let corpo: syn::Block = if func.sig.asyncness.is_some() {
        syn::parse_quote!({
            let __aop_span = tracing::info_span!(#span_name);
            tracing::Instrument::instrument(async move #bloco, __aop_span).await
        })
    } else {
        syn::parse_quote!({
            let __aop_span = tracing::info_span!(#span_name);
            let __aop_guard = __aop_span.enter();
            #bloco
        })
    };
    *func.block = corpo;

    quote!(#func).into()
}
