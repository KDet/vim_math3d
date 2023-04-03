use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Fields};


#[proc_macro_derive(StructOps)]
pub fn derive_math_struct_ops(input: TokenStream) -> TokenStream {
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
    let field_names = fields.iter().map(|field| &field.ident).map(|ident| quote! { #ident }).collect::<Vec<_>>();
    let field_types = fields.iter().map(|field| &field.ty).map(|ty| quote! { #ty }).collect::<Vec<_>>();
  
    let field_setters = fields.iter().map(|field| {
        let field_name = &field.ident;
        let setter_name = format_ident!("set_{}", field_name.as_ref().unwrap());
        let unit_name = format_ident!("unit_{}", field_name.as_ref().unwrap());
        let field_type = &field.ty;
        let other_fields = fields.iter().filter(|f| f.ident != *field_name).map(|f| &f.ident).collect::<Vec<_>>();
        let other_types = fields.iter().filter(|f| f.ident != *field_name).map(|f| &f.ty );

        quote! {
            pub fn #setter_name(self, value: #field_type) -> Self {
                Self {
                    #field_name: value,
                    #(#other_fields: self.#other_fields ),*
                }
            }
            pub fn #unit_name() -> Self {
                Self {
                    #field_name: #field_type::one(),
                    #(#other_fields: #other_types::zero() ),*
                }
            }
        }
    });

    let field_tuple_constructors = fields.iter().map(|field| {
        let index = syn::Index::from(fields.iter().position(|f| f.ident == field.ident).unwrap());
        quote! { tuple.#index }
    });

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
        impl #impl_generics #ident #ty_generics #where_clause {
            pub fn new(#(#field_names: #field_types),*) -> Self { Self { #(#field_names),* } }

            pub fn zero() -> Self { Self { #(#field_names: #field_types::zero()),* } }
            pub fn one() -> Self { Self { #(#field_names: #field_types::one()),* } }
            pub fn min_value() -> Self { Self { #(#field_names: #field_types::min_value()),* } }
            pub fn max_value() -> Self { Self { #(#field_names: #field_types::max_value()),* } }

            #(#field_setters)*
        }
        
        impl #impl_generics From<(#(#field_types),*)> for #ident #ty_generics #where_clause {
            fn from(tuple: ( #(#field_types),* )) -> Self {
                Self::new( #(#field_tuple_constructors, )* )
            }
        }

        impl #impl_generics Into<( #(#field_types),* )> for #ident #ty_generics #where_clause {
            fn into(self) -> ( #(#field_types),* ) {
                ( #(self.#field_names),* )
            }
        }

        impl #impl_generics std::hash::Hash for #ident #ty_generics #where_clause {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) { #(#field_hashes)* }
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
    let field_names = fields.iter()
        .map(|field| &field.ident)
        .map(|ident| quote! { #ident })
        .collect::<Vec<_>>();
    let count = fields.len();
    let field_type = &fields[0].ty;

    let (first_field, other_fields) = field_names.split_first().expect("Expected at least one field");

    let expanded = quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            pub const NUM_COMPONENTS: usize = #count;

            pub fn from_value(value: #field_type) -> Self { Self { #(#field_names : value),* } }

            pub fn any_component_negative(&self) -> bool { self.min_component() < #field_type::zero() }
            pub fn min_component(&self) -> #field_type { self.#first_field #(.min(self.#other_fields))* }
            pub fn max_component(&self) -> #field_type { self.#first_field #(.max(self.#other_fields))* }
            pub fn sum_components(&self) -> #field_type { #(self.#field_names)+* }
            pub fn sum_sqr_components(&self) -> #field_type { #(self.#field_names * self.#field_names)+* }
            pub fn product_components(&self) -> #field_type { self.#first_field #(*self.#other_fields)* }
            pub fn get_component(&self, index: usize) -> Option<#field_type> { if index < 0 || index > #count { return None } else { Some([#(self.#field_names),*][index]) } }
            pub fn magnitude_squared(&self) -> #field_type { self.sum_sqr_components() }
            pub fn magnitude(&self) -> #field_type { self.sum_sqr_components().sqrt() }

            pub fn almost_equals(&self, other: &Self, tolerance: #field_type) -> bool { #((self.#field_names - other.#field_names).abs() < tolerance)&&*}
            pub fn almost_zero(&self, tolerance: #field_type) -> bool { #((self.#field_names.abs() < tolerance))&&* }
            pub fn is_nan(&self) -> bool { self.#first_field.is_nan() #(|| self.#other_fields.is_nan())* }
            pub fn is_infinite(&self) -> bool { self.#first_field.is_infinite() #(|| self.#other_fields.is_infinite())* }

            pub fn lerp(self, other: &Self, t: #field_type) -> Self { Self { #(#field_names: self.#field_names + (other.#field_names - self.#field_names) * t),* } }
            pub fn inverse_lerp(self, a: &Self, b: &Self) -> Self { Self { #(#field_names: (self.#field_names - a.#field_names) / (b.#field_names - a.#field_names)),* } }
            pub fn lerp_precise(self, other: &Self, t: #field_type) -> Self { Self { #(#field_names: ((#field_type::one() - t) * self.#field_names) + (other.#field_names * t) ),* } }
            pub fn clamp_lower(self, min: &Self) -> Self { self.max(min) }
            pub fn clamp_upper(self, max: &Self) -> Self { self.min(max) }
            pub fn clamp(self, min: &Self, max: &Self) -> Self { self.min(max).max(min) }
            pub fn average(self, other: &Self) -> Self { self.lerp(other, #field_type::from(0.5).unwrap()) }          
            pub fn barycentric(self, v2: &Self, v3: &Self, u: #field_type, v: #field_type) -> Self {
                let v2_sub_self = Self { #(#field_names: v2.#field_names - self.#field_names),* };
                let v3_sub_self = Self { #(#field_names: v3.#field_names - self.#field_names),* };
                self + v2_sub_self * u + v3_sub_self * v
            }

            pub fn abs(self) -> Self { Self { #(#field_names: self.#field_names.abs()),* } }
            pub fn acos(self) -> Self { Self { #(#field_names: self.#field_names.acos()),* } }
            pub fn asin(self) -> Self { Self { #(#field_names: self.#field_names.asin()),* } }
            pub fn atan(self) -> Self { Self { #(#field_names: self.#field_names.atan()),* } }
            pub fn cos(self) -> Self { Self { #(#field_names: self.#field_names.cos()),* } }
            pub fn cosh(self) -> Self { Self { #(#field_names: self.#field_names.cosh()),* } }
            pub fn exp(self) -> Self { Self { #(#field_names: self.#field_names.exp()),* } }
            pub fn ln(self) -> Self { Self { #(#field_names: self.#field_names.ln()),* } }
            pub fn log10(self) -> Self { Self { #(#field_names: self.#field_names.log10()),* } }
            pub fn sin(self) -> Self { Self { #(#field_names: self.#field_names.sin()),* } }
            pub fn sinh(self) -> Self { Self { #(#field_names: self.#field_names.sinh()),* } }
            pub fn sqrt(self) -> Self { Self { #(#field_names: self.#field_names.sqrt()),* } }
            pub fn tan(self) -> Self { Self { #(#field_names: self.#field_names.tan() ),* } }
            pub fn tanh(self) -> Self { Self { #(#field_names: self.#field_names.tanh()),* } }

            pub fn inverse(self) -> Self { Self { #(#field_names: #field_type::one() / self.#field_names),* } }
            pub fn ceil(self) -> Self { Self { #(#field_names: self.#field_names.ceil()),* } }
            pub fn floor(self) -> Self { Self { #(#field_names: self.#field_names.floor()),* } }
            pub fn round(self) -> Self { Self { #(#field_names: self.#field_names.round()),* } }
            pub fn trunc(self) -> Self { Self { #(#field_names: self.#field_names.trunc()),* } }
            pub fn sqr(self) -> Self { Self { #(#field_names: self.#field_names * self.#field_names),* } }
            pub fn cube(self) -> Self { Self { #(#field_names: self.#field_names * self.#field_names * self.#field_names),* } }
            pub fn to_radians(self) -> Self { Self { #(#field_names: self.#field_names.to_radians()),* } }
            pub fn to_degrees(self) -> Self { Self { #(#field_names: self.#field_names.to_degrees()),* } }

            pub fn distance_squared(&self, other: &Self) -> #field_type { 
                let sub = Self { #(#field_names: self.#field_names - other.#field_names),* };
                sub.length_squared() 
            }
            pub fn distance(&self, other: &Self) -> #field_type { 
                let sub = Self { #(#field_names: self.#field_names - other.#field_names),* };
                sub.length() 
            }
            pub fn length_squared(&self) -> #field_type { self.sum_sqr_components() }
            pub fn length(&self) -> #field_type { self.length_squared().sqrt() }
            pub fn normalize(self) -> Self { 
                let len = self.length();
                self / len
            }
            pub fn safe_normalize(self) -> Self { 
                let len = self.length();
                if len != #field_type::zero() { self / len } else { self }
            }
            pub fn dot(&self, other: &Self) -> #field_type { #(self.#field_names * other.#field_names)+* }
            pub fn min(self, other: &Self) -> Self {  Self { #(#field_names: self.#field_names.min(other.#field_names)),* } }
            pub fn max(self, other: &Self) -> Self {  Self { #(#field_names: self.#field_names.max(other.#field_names)),* } }
            pub fn square_root(self) -> Self { self.sqrt() }
        }

        impl #impl_generics PartialOrd for #ident #ty_generics #where_clause {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.magnitude_squared().partial_cmp(&other.magnitude_squared()) }
        }

        impl #impl_generics Add for #ident #ty_generics #where_clause {
            type Output = Self;
            fn add(self, other: Self) -> Self::Output { Self { #(#field_names: self.#field_names + other.#field_names),* } }
        }
        impl #impl_generics Add<#field_type> for #ident #ty_generics #where_clause {
            type Output = Self;
            fn add(self, other: #field_type) -> Self::Output { Self { #(#field_names: self.#field_names + other),* } }
        }

        impl #impl_generics Sub for #ident #ty_generics #where_clause {
            type Output = Self;
            fn sub(self, other: Self) -> Self::Output { Self { #(#field_names: self.#field_names - other.#field_names),* } }
        }
        impl #impl_generics Sub<#field_type> for #ident #ty_generics #where_clause {
            type Output = Self;
            fn sub(self, other: #field_type) -> Self::Output { Self { #(#field_names: self.#field_names - other),* } }
        }

        impl #impl_generics Mul for #ident #ty_generics #where_clause {
            type Output = Self;
            fn mul(self, other: Self) -> Self::Output { Self { #(#field_names: self.#field_names * other.#field_names),* } }
        }
        impl #impl_generics Mul<#field_type> for #ident #ty_generics #where_clause {
            type Output = Self;
            fn mul(self, other: #field_type) -> Self::Output { Self { #(#field_names: self.#field_names * other),* } }
        }

        impl #impl_generics Div for #ident #ty_generics #where_clause {
            type Output = Self;
            fn div(self, other: Self) -> Self::Output { Self { #(#field_names: self.#field_names / other.#field_names),* } }
        }
        impl #impl_generics Div<#field_type> for #ident #ty_generics #where_clause {
            type Output = Self;
            fn div(self, other: #field_type) -> Self::Output { Self { #(#field_names: self.#field_names / other),* } }
        }

        impl #impl_generics Neg for #ident #ty_generics #where_clause {
            type Output = Self;
            fn neg(self) -> Self::Output { Self::zero() - self }
        }
    };

    
    TokenStream::from(expanded)
}

#[proc_macro_derive(IntervalOps)]
pub fn derive_math_interval_ops(input: TokenStream) -> TokenStream {
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
    let field_names = fields.iter()
        .map(|field| &field.ident)
        .map(|ident| quote! { #ident })
        .collect::<Vec<_>>();
    let field_type = &fields[0].ty;

    let expanded = quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            pub fn is_nan(&self) -> bool { #(self.#field_names.is_nan())||* }
            pub fn is_infinite(&self) -> bool { #(self.#field_names.is_infinite())||* }
            pub fn extent(&self) -> #field_type { self.max - self.min }

            // public <#= type #> Extent => (Max - Min);
            // public <#= type #> Center => Min.Average(Max);   
            //  double MagnitudeSquared() => Extent.MagnitudeSquared();
            //  double Magnitude() => MagnitudeSquared().Sqrt();        
            //  <#= name #> Merge(<#= name #> other) => new <#= name #>(Min.Min(other.Min), Max.Max(other.Max));
            //  <#= name #> Intersection(<#= name #> other) => new <#= name #>(Min.Max(other.Min), Max.Min(other.Max));
            //  static <#= name #> operator + (<#= name #> value1, <#= name #> value2) => value1.Merge(value2);
            //  static <#= name #> operator - (<#= name #> value1, <#= name #> value2) => value1.Intersection(value2);
            //  <#= name #> Merge(<#= type #> other) => new <#= name #>(Min.Min(other), Max.Max(other));
            //  static <#= name #> operator + (<#= name #> value1, <#= type #> value2) => value1.Merge(value2);
            // public static <#= name #> Empty = Create(<#= type #>.MaxValue, <#= type #>.MinValue);

            // pub fn from_value(value: #field_type) -> Self { Self { #(#field_names_value),* } }
            // pub fn dot(&self, other: &Self) -> #field_type { (self.#first_field * other.#first_field) #(+(self.#other_fields * other.#other_fields))* }
            
            // pub fn min_component(&self) -> #field_type { self.#first_field #(.min(self.#other_fields))* }
            // pub fn max_component(&self) -> #field_type { self.#first_field #(.max(self.#other_fields))* }
            // pub fn sum_components(&self) -> #field_type { self.#first_field #(+self.#other_fields)* }
            // pub fn sum_sqr_components(&self) -> #field_type { (self.#first_field * self.#first_field) #(+(self.#other_fields * self.#other_fields))* }
            // pub fn product_components(&self) -> #field_type { self.#first_field #(*self.#other_fields)* }
            // pub fn get_component(&self, index: usize) -> Option<#field_type> {
            //     if index < 0 || index > #count { return None } else { Some([#(self.#field_names),*][index]) }
            // }
            // pub fn magnitude_squared(&self) -> #field_type { self.sum_sqr_components() }
            // pub fn magnitude(&self) -> #field_type { self.sum_sqr_components().sqrt() }
        }

        impl #impl_generics PartialOrd for #ident #ty_generics #where_clause {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.magnitude_squared().partial_cmp(&other.magnitude_squared())
            }
        }

        // impl #impl_generics Sub for #ident #ty_generics #where_clause {
        //     type Output = Self;
        //     fn sub(self, other: Self) -> Self::Output { { Self { #(#field_initializers_sub,)* } } }
        // }

        // impl #impl_generics Neg for #ident #ty_generics #where_clause {
        //     type Output = Self;
        //     fn neg(self) -> Self::Output { Self::zero() - self }
        // }
    };

    //eprintln!("expanded code:\n{}", expanded);
    TokenStream::from(expanded)
}
