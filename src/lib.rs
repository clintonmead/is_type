//! # Equality constraints for types
//!
//! `is_type` gives the trait `Is<Type=X>` which acts like an equality constraint.
//! 
//! There's also functions to move between these two equivalent types,
//! as Rust will still not realise these types are
//! 
//! The constraint `X : Is<Type=Y>` requires that `X` == `Y`.
//! 
//! The idea was stolen from 
//! [this comment on equality constraints in where clauses] which
//! contains an example use case.
//! 
//! [this comment on equality constraints in where clauses]: 
//! https://github.com/rust-lang/rust/issues/20041#issuecomment-414551783
//!
//! Note that `into_val` and `from_val` are basically `into` and `from`,
//! but for this trait to work universally we need a universal instance, 
//! which one can't define on `Into` or `From`.
pub trait Is 
{
    type Type;

    fn into_val(self) -> Self::Type;
    fn into_ref(&self) -> &Self::Type;
    fn into_mut_ref(&mut self) -> &mut Self::Type;

    fn from_val(x : Self::Type) -> Self;
    fn from_ref(x : &Self::Type) -> &Self;
    fn from_mut_ref(x : &mut Self::Type) -> &mut Self;
}

impl<T> Is for T {
    type Type = T;

    fn into_val(self) -> Self::Type {
        self
    }

    fn into_ref(&self) -> &Self::Type {
        self
    }

    fn into_mut_ref(&mut self) -> &mut Self::Type {
        self
    }

    fn from_val(x : Self::Type) -> Self {
        x
    }

    fn from_ref(x : &Self::Type) -> &Self {
        x
    }

    fn from_mut_ref(x : &mut Self::Type) -> &mut Self {
        x
    }
}
