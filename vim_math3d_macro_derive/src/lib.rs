use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Fields};


#[proc_macro_derive(Struct)]
pub fn derive_math_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(named_fields) => named_fields.named,
            _ => panic!("Struct can only be derived for structs with named fields"),
        },
        _ => panic!("Struct can only be derived for structs"),
    };

    
    let field_name_types = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! { #field_name: #field_type }
    });
    let field_names = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name }
    });
    
    let field_initializers_zero = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! { #field_name: #field_type::zero() }
    });

    let field_initializers_one = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! { #field_name: #field_type::one() }
    });

    let field_initializers_min = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! { #field_name: #field_type::min_value() }
    });

    let field_initializers_max = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! { #field_name: #field_type::max_value() }
    });

    let field_setters = fields.iter().map(|field| {
        let field_name = &field.ident;
        let setter_name = format_ident!("set_{}", field_name.as_ref().unwrap());
        let field_type = &field.ty;
        let other_fields = fields.iter().filter(|f| f.ident != *field_name).map(|f| &f.ident);
        quote! {
            pub fn #setter_name(self, value: #field_type) -> Self {
                Self {
                    #field_name: value,
                    #( #other_fields: self.#other_fields ),*
                }
            }
        }
    });
    
    let expanded = quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            pub fn new(#(#field_name_types),*) -> Self { 
                Self { 
                    #(#field_names),*
                } 
            }

            pub fn zero() -> Self {
                Self {
                    #(#field_initializers_zero,)*
                }
            }
            pub fn one() -> Self {
                Self {
                    #(#field_initializers_one,)*
                }
            }
            pub fn min_value() -> Self {
                Self {
                    #(#field_initializers_min,)*
                }
            }
            pub fn max_value() -> Self {
                Self {
                    #(#field_initializers_max,)*
                }
            }
            #(#field_setters)*
        }
    };


    TokenStream::from(expanded)
}

#[proc_macro_derive(AlmostEq)]
pub fn derive_math_almost_eq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(named_fields) => named_fields.named,
            _ => panic!("AlmostEq can only be derived for structs with named fields"),
        },
        _ => panic!("AlmostEq can only be derived for structs"),
    };

    let generic_type_param = match generics.params.first() {
        Some(syn::GenericParam::Type(ty)) => &ty.ident,
        _ => panic!("Expecting a type parameter for the generic struct"),
    };

    let almost_equals_checks = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { (self.#field_name - other.#field_name).abs() < tolerance }
    });
    
    let almost_zero_checks = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { self.#field_name.abs() < tolerance }
    });

    let expanded = quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            pub fn almost_equals(&self, other: &Self, tolerance: #generic_type_param) -> bool {
                #(#almost_equals_checks)&&*
            }
            pub fn almost_zero(&self, tolerance: #generic_type_param) -> bool {
                #(#almost_zero_checks)&&*
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(FromTuple)]
pub fn derive_math_from_tuple(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(named_fields) => named_fields.named,
            _ => panic!("FromTuple can only be derived for structs with named fields"),
        },
        _ => panic!("FromTuple can only be derived for structs"),
    };

    let from_impl = {
        let field_tuple_types_tuple = fields.iter().map(|field| {
            let field_type = &field.ty;
            quote! { #field_type }
        });
        let field_tuple_types_params = fields.iter().map(|field| {
            let field_type = &field.ty;
            quote! { #field_type }
        });
        let field_constructors = fields.iter().map(|field| {
            let index = syn::Index::from(fields.iter().position(|f| f.ident == field.ident).unwrap());
            quote! { tuple.#index }
        });
        quote! {
            impl #impl_generics From<(#(#field_tuple_types_tuple, )*)> for #ident #ty_generics #where_clause {
                fn from(tuple: ( #(#field_tuple_types_params, )* )) -> Self {
                    Self::new( #(#field_constructors, )* )
                }
            }
        }
    };

    let into_impl = {
        let field_tuple_types_tuple = fields.iter().map(|field| {
            let field_type = &field.ty;
            quote! { #field_type }
        });
        let field_tuple_types_params = fields.iter().map(|field| {
            let field_type = &field.ty;
            quote! { #field_type }
        });
        let field_tuple_values = fields.iter().map(|field| {
            let field_name = &field.ident;
            quote! { self.#field_name }
        });
        quote! {
            impl #impl_generics Into<( #(#field_tuple_types_tuple, )* )> for #ident #ty_generics #where_clause {
                fn into(self) -> ( #(#field_tuple_types_params, )* ) {
                    ( #(#field_tuple_values, )* )
                }
            }
        }
    };

    let expanded = quote! {
        #from_impl
        #into_impl
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(FloatHash)]
pub fn derive_math_float_hash(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(named_fields) => named_fields.named,
            _ => panic!("FloatHash can only be derived for structs with named fields"),
        },
        _ => panic!("FloatHash can only be derived for structs"),
    };

    let field_hashes = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
    
        match field_type {
            syn::Type::Path(type_path) if type_path.qself.is_none() => {
                let last_segment = &type_path.path.segments.last().unwrap();
                let type_ident = &last_segment.ident;

                if type_ident == "T" || type_ident == "Float" || type_ident == "f32" || type_ident == "f64" {
                    quote! {
                        let (mantissa, exponent, sign) = self.#field_name.integer_decode();
                        mantissa.hash(state);
                        exponent.hash(state);
                        sign.hash(state);
                    }
                } else {
                    quote! {
                        self.#field_name.hash(state);
                    }
                }
            }
            _ => quote! {
                self.#field_name.hash(state);
            },
        }
    });

    let expanded = quote! {
        impl #impl_generics std::hash::Hash for #ident #ty_generics #where_clause {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                #(#field_hashes)*
            }
        }
    };
    TokenStream::from(expanded)
}

// #[proc_macro_derive(Interval)]
// pub fn derive_math_interval(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let ident = input.ident;
//     let generics = input.generics;
//     let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    
//     let fields = match input.data {
//         Data::Struct(data) => match data.fields {
//             Fields::Named(named_fields) => named_fields.named,
//             _ => panic!("Struct can only be derived for structs with named fields"),
//         },
//         _ => panic!("Struct can only be derived for structs"),
//     };

//     let count = fields.iter().count();
//     let field_name_types = fields.iter().map(|field| {
//         let field_name = &field.ident;
//         let field_type = &field.ty;
//         quote! { #field_name: #field_type }
//     });
//     let field_names = fields.iter().map(|field| {
//         let field_name = &field.ident;
//         quote! { #field_name }
//     });
    
//     let field_initializers_zero = fields.iter().map(|field| {
//         let field_name = &field.ident;
//         let field_type = &field.ty;
//         quote! { #field_name: #field_type::zero() }
//     });

//     let field_initializers_one = fields.iter().map(|field| {
//         let field_name = &field.ident;
//         let field_type = &field.ty;
//         quote! { #field_name: #field_type::one() }
//     });

//     let field_initializers_min = fields.iter().map(|field| {
//         let field_name = &field.ident;
//         let field_type = &field.ty;
//         quote! { #field_name: #field_type::min_value() }
//     });

//     let field_initializers_max = fields.iter().map(|field| {
//         let field_name = &field.ident;
//         let field_type = &field.ty;
//         quote! { #field_name: #field_type::max_value() }
//     });

//     let field_setters = fields.iter().map(|field| {
//         let field_name = &field.ident;
//         let setter_name = format_ident!("set_{}", field_name.as_ref().unwrap());
//         let field_type = &field.ty;
//         let other_fields = fields.iter().filter(|f| f.ident != *field_name).map(|f| &f.ident);
//         quote! {
//             pub fn #setter_name(self, value: #field_type) -> Self {
//                 Self {
//                     #field_name: value,
//                     #( #other_fields: self.#other_fields ),*
//                 }
//             }
//         }
//     });
    
//     let expanded = quote! {
//         impl #impl_generics #ident #ty_generics #where_clause {
//             pub const NUM_COMPONENTS2: usize = #count;
//         }
//     };

//     eprintln!("expanded code:\n{}", expanded);
//     TokenStream::from(expanded)
// }

#[proc_macro_derive(UnaryOps)]
pub fn derive_math_unary_ops(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(named_fields) => named_fields.named,
            _ => panic!("UnaryOps can only be derived for structs with named fields"),
        },
        _ => panic!("UnaryOps can only be derived for structs"),
    };

    let field_initializers_abs = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.abs() }
    });
    let field_initializers_acos = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.acos() }
    });
    let field_initializers_asin = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.asin() }
    });
    let field_initializers_atan = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.atan() }
    });
    let field_initializers_cos = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.cos() }
    });
    let field_initializers_cosh = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.cosh() }
    });
    let field_initializers_exp = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.exp() }
    });
    let field_initializers_ln = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.ln() }
    });
    let field_initializers_log10 = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.log10() }
    });
    let field_initializers_sin = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.sin() }
    });
    let field_initializers_sinh = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.sinh() }
    });
    let field_initializers_sqrt = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.sqrt() }
    });
    let field_initializers_tan = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.tan() }
    });
    let field_initializers_tanh = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.tanh() }
    });
    let field_initializers_inverse = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! { #field_name: #field_type::one() / self.#field_name }
    });
    let field_initializers_ceil = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.ceil() }
    });
    let field_initializers_floor = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.floor() }
    });
    let field_initializers_round = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.round() }
    });
    let field_initializers_trunc = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.trunc() }
    });
    let field_initializers_sqr = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name * self.#field_name }
    });
    let field_initializers_cube = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name * self.#field_name * self.#field_name }
    });
    let field_initializers_to_radians = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.to_radians() }
    });
    let field_initializers_to_degrees = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name.to_degrees() }
    });

    let expanded = quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            pub fn abs(self) -> Self { Self { #(#field_initializers_abs,)* } }
            pub fn acos(self) -> Self { Self { #(#field_initializers_acos,)* } }
            pub fn asin(self) -> Self { Self { #(#field_initializers_asin,)* } }
            pub fn atan(self) -> Self { Self { #(#field_initializers_atan,)* } }
            pub fn cos(self) -> Self { Self { #(#field_initializers_cos,)* } }
            pub fn cosh(self) -> Self { Self { #(#field_initializers_cosh,)* } }
            pub fn exp(self) -> Self { Self { #(#field_initializers_exp,)* } }
            pub fn ln(self) -> Self { Self { #(#field_initializers_ln,)* } }
            pub fn log10(self) -> Self { Self { #(#field_initializers_log10,)* } }
            pub fn sin(self) -> Self { Self { #(#field_initializers_sin,)* } }
            pub fn sinh(self) -> Self { Self { #(#field_initializers_sinh,)* } }
            pub fn sqrt(self) -> Self { Self { #(#field_initializers_sqrt,)* } }
            pub fn tan(self) -> Self { Self { #(#field_initializers_tan,)* } }
            pub fn tanh(self) -> Self { Self { #(#field_initializers_tanh,)* } }

            pub fn inverse(self) -> Self { Self { #(#field_initializers_inverse,)* } }
            pub fn ceil(self) -> Self { Self { #(#field_initializers_ceil,)* } }
            pub fn floor(self) -> Self { Self { #(#field_initializers_floor,)* } }
            pub fn round(self) -> Self { Self { #(#field_initializers_round,)* } }
            pub fn trunc(self) -> Self { Self { #(#field_initializers_trunc,)* } }
            pub fn sqr(self) -> Self { Self { #(#field_initializers_sqr,)* } }
            pub fn cube(self) -> Self { Self { #(#field_initializers_cube,)* } }
            pub fn to_radians(self) -> Self { Self { #(#field_initializers_to_radians,)* } }
            pub fn to_degrees(self) -> Self { Self { #(#field_initializers_to_degrees,)* } }
        }
    };

    //eprintln!("expanded code:\n{}", expanded);
    TokenStream::from(expanded)
}

#[proc_macro_derive(VectorOps)]
pub fn derive_math_vector_ops(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(named_fields) => named_fields.named,
            _ => panic!("UnaryOps can only be derived for structs with named fields"),
        },
        _ => panic!("UnaryOps can only be derived for structs"),
    };
    let count = fields.iter().count();

    let field_type = &fields[0].ty;
    let field_names_value = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name : value }
    });

    let expanded = quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            pub const NUM_COMPONENTS: usize = #count;

            pub fn from_value(value: #field_type) -> Self { 
                Self { 
                    #(#field_names_value),*
                } 
            }

        }
    };

    eprintln!("expanded code:\n{}", expanded);
    TokenStream::from(expanded)

}