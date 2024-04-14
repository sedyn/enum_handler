use quote::quote;
use syn::{
    braced,
    parse::{self, Parse},
    parse_macro_input,
    punctuated::Punctuated,
    Attribute, Ident, ParenthesizedGenericArguments, Token, Type,
};

#[derive(Default)]
struct VariantOption {
    boxed: bool,
    reference: bool,
}

impl VariantOption {
    fn new(attrs: &[Attribute]) -> Self {
        let mut option = Self::default();

        for attr in attrs {
            let path = attr.meta.path();

            if path.is_ident("boxed") {
                option.boxed = true;
            }

            if path.is_ident("reference") {
                option.reference = true;
            }


        }

        option
    }
}

#[proc_macro]
pub fn enum_handler(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as HandlerSpec);
    let name = &input.name;
    let executor = &input.executor;

    let option = VariantOption::new(&input.attrs);

    let variants: Vec<_> = input
        .variants
        .iter()
        .map(|varient| {

            let mut inner = quote! { #varient };

            if option.boxed {
                inner = quote! { Box<#inner> };
            }

            quote! {
                #varient(#inner)
            }
        })
        .collect();

    let execute_arms: Vec<_> = input
        .variants
        .iter()
        .map(|varient| {
            let message = if option.reference {
                quote! { message.as_ref() }
            } else {
                quote! { message }
            };

            let execute = if input.context.is_some() {
                quote! { executor.handle(#message, ctx) }
            } else {
                quote! { executor.handle(#message) }
            };

            quote! {
                #name::#varient(message) => {
                    #execute
                }
            }
        })
        .collect();

    let from: Vec<_> = input
        .variants
        .iter()
        .map(|varient| {
            let value = if option.boxed {
                quote! { Box::new(value) }
            } else {
                quote! { value }
            };

            quote! {
                impl From<#varient> for #name {
                    fn from(value: #varient) -> Self {
                        Self::#varient(#value)
                    }
                }
            }
        })
        .collect();

    let execute = if let Some(ctx) = input.context {
        quote! { handle(self, executor: &mut #executor, ctx: #ctx) }
    } else {
        quote! { handle(self, executor: &mut #executor) }
    };

    quote! {
        enum #name {
            #(#variants),*
        }

        impl #name {
            pub fn #execute {
                match self {
                    #(#execute_arms),*
                }
            }
        }

        #(#from)*
    }
    .into()
}

struct HandlerSpec {
    executor: Type,
    context: Option<Type>,
    attrs: Vec<Attribute>,
    name: Ident,
    variants: Vec<Ident>,
}

impl Parse for HandlerSpec {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        input.parse::<Token![enum]>()?;

        let name = input.parse::<Ident>()?;

        let content;
        braced!(content in input);

        let variants = Punctuated::<Ident, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();

        input.parse::<Token![,]>()?;

        let sigature = input.parse::<ParenthesizedGenericArguments>()?;

        let mut iter = sigature.inputs.into_iter();

        let executor = iter.next().expect("Type of executor is required");
        let context = iter.next();

        Ok(HandlerSpec {
            executor,
            context,
            attrs,
            name,
            variants,
        })
    }
}
