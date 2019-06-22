//! # Equality constraints for types
//!
//! `is_type` gives the trait `Is<X>` which acts like an equality constraint.
//! 
//! There's also functions to move between these two equivalent types,
//! as Rust will still not realise these types are
//! 
//! The constraint `X : Is<Y>` requires that `X` == `Y`.
//! 
//! The idea was stolen from 
//! [this comment on equality constraints in where clauses] which
//! contains an example use case.
//! 
//! [this comment on equality constraints in where clauses]: 
//! https://github.com/rust-lang/rust/issues/20041#issuecomment-414551783
//! 
//! Note that whilst I'd like the trait `Is` to have the where condition:
//! `where Self : From<Self::Type>`
//! the Rust compiler doesn't like the self referential nature of this.

pub trait Is 
{
    type Type;

    fn into(self) -> Self::Type;
    fn into_ref(&self) -> &Self::Type;
    fn into_mut_ref(&mut self) -> &mut Self::Type;

    fn from(x : Self::Type) -> Self;
    fn from_ref(x : &Self::Type) -> &Self;
    fn from_mut_ref(x : &mut Self::Type) -> &mut Self;
}

impl<T> Is for T {
    type Type = T;

    fn into(self) -> Self::Type {
        self
    }

    fn into_ref(&self) -> &Self::Type {
        self
    }

    fn into_mut_ref(&mut self) -> &mut Self::Type {
        self
    }

    fn from(x : Self::Type) -> Self {
        x
    }

    fn from_ref(x : &Self::Type) -> &Self {
        x
    }

    fn from_mut_ref(x : &mut Self::Type) -> &mut Self {
        x
    }
}