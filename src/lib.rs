use quote::quote;
use syn::{
    parse::{self, Parse},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, ParenthesizedGenericArguments, Token, Type,
};

#[proc_macro]
pub fn enum_handler(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as EnumVariants);
    let name = &input.name;
    let executor = &input.executor;

    let variants: Vec<_> = input
        .variants
        .iter()
        .map(|varient| {
            quote! {
                #varient(#varient)
            }
        })
        .collect();

    let execute_arms: Vec<_> = input
        .variants
        .iter()
        .map(|varient| {
            let execute = if input.context.is_some() {
                quote! { executor.execute(message, ctx) }
            } else {
                quote! { executor.execute(message) }
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
            quote! {
                impl From<#varient> for #name {
                    fn from(value: #varient) -> Self {
                        Self::#varient(value)
                    }
                }
            }
        })
        .collect();

    let execute = if let Some(ctx) = input.context {
        quote! { execute(self, executor: &mut #executor, ctx: #ctx) }
    } else {
        quote! { execute(self, executor: &mut #executor) }
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

struct EnumVariants {
    executor: Type,
    context: Option<Type>,
    name: Ident,
    variants: Vec<Ident>,
}

impl Parse for EnumVariants {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        input.parse::<Token![,]>()?;

        let sigature = input.parse::<ParenthesizedGenericArguments>()?;
        input.parse::<Token![,]>()?;

        let mut iter = sigature.inputs.into_iter();

        let executor = iter.next().expect("Type of executor is required");
        let context = iter.next();

        let parser = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        let variants = parser.into_iter().collect();

        Ok(EnumVariants {
            executor,
            context,
            name,
            variants,
        })
    }
}
