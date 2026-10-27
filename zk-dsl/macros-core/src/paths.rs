use proc_macro2::TokenStream;
use quote::quote;

pub fn proof_input() -> TokenStream {
    quote!(::spr_program::conversion::ProofInput)
}

pub fn placeholder() -> TokenStream {
    quote!(::spr_program::conversion::Placeholder)
}

pub fn instantiate_padded() -> TokenStream {
    quote!(::spr_program::conversion::instantiate_padded)
}

pub fn placeholders() -> TokenStream {
    quote!(::spr_program::conversion::placeholders)
}

pub fn from_circuit() -> TokenStream {
    quote!(::spr_program::conversion::FromCircuit)
}

pub fn allocator() -> TokenStream {
    quote!(::spr_program::conversion::Allocator)
}

pub fn circuit_error() -> TokenStream {
    quote!(::spr_program::CircuitError)
}

pub fn circuit_type() -> TokenStream {
    quote!(::spr_program::circuit::CircuitType)
}

pub fn circuit_default() -> TokenStream {
    quote!(::spr_program::circuit::CircuitDefault)
}

pub fn circuit_var() -> TokenStream {
    quote!(::spr_program::circuit::CircuitVar)
}

pub fn constant() -> TokenStream {
    quote!(::spr_program::circuit::constant)
}

pub fn private() -> TokenStream {
    quote!(::spr_program::__private)
}

pub fn public_field() -> TokenStream {
    quote!(::spr_program::PublicField)
}

pub fn public_field_kind() -> TokenStream {
    quote!(::spr_program::PublicFieldKind)
}

pub fn public_input_layout() -> TokenStream {
    quote!(::spr_program::PublicInputLayout)
}

pub fn public_value() -> TokenStream {
    quote!(::spr_program::circuit::PublicValue)
}
