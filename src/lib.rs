mod structs;
pub mod math3d_ops;
pub mod constants;
pub mod transformable;
pub mod hash;
pub mod stateless_random;

pub use structs::*;
pub use transformable::*; 
//pub use math3d_ops::*;
//pub use num_traits::{Zero, One};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a: bool = false;
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


// let to_degrees_impl = {
//     let field_initializers_to_degrees = fields.iter().map(|field| {
//         let field_name = &field.ident;
//         quote! { #field_name: self.#field_name.to_degrees() }
//     });
//     quote! {
//         impl #impl_generics #ident #ty_generics #where_clause {
//             pub fn to_degrees(self) -> Self { Self { #(#field_initializers_to_degrees,)* } }
//         }
//     }
// };