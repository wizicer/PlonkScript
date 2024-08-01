use proc_macro2::TokenStream;
use quote::quote;

pub fn get_header() -> TokenStream {
    quote! {
        #![allow(unused_imports)]
        #![allow(dead_code)]
        #![allow(unused_mut)]
        #![allow(unused_doc_comments)]
        use std::marker::PhantomData;
        use std::{collections::HashMap, io};

        use halo2_proofs::{
            circuit::{floor_planner::V1, *},
            pasta::group::ff::PrimeField,
            plonk::*,
            poly::Rotation,
        };
    }
}
