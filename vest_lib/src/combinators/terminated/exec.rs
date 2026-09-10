//! Executable implementation for sequential formats discarding their suffix.
use crate::core::exec::output::*;
use crate::{
    combinators::Pair,
    core::{
        exec::{
            input::InputBuf,
            parser::{PResult, Parser},
            serializer::{ByteLen, PreSerializeError, Prepare, Serializer},
            generator::{StdGen, Generator},
            ParseError,
        },
        spec::{SafeParser, SpecByteLen, SpecParser, SpecSerializer},
    },
};
use rand::{Rng, RngExt, SeedableRng};
use rand::rngs::StdRng;
use vstd::prelude::*;
use OutputBuf;

verus! {

// Malleable version
impl<I, A, B, BVal> Parser<I> for super::Terminated<A, B, BVal, false> where
    I: InputBuf,
    A: Parser<I> + SafeParser,
    B: Parser<I, PT = BVal> + SafeParser<PVal = BVal>,
    BVal: DeepView<V = BVal>,
 {
    type PT = A::PT;

    open spec fn exec_inv(&self) -> bool {
        Pair(&self.a, &self.b).exec_inv()
    }

    fn parse(&self, ibuf: &I) -> PResult<Self::PT> {
        let (n, (v, _)) = Pair(&self.a, &self.b).parse(ibuf)?;
        Ok((n, v))
    }
}

// Non-malleable version
impl<I, A, B, BVal> Parser<I> for super::Terminated<A, B, BVal, true> where
    I: InputBuf,
    A: Parser<I> + SafeParser,
    B: Parser<I, PT = BVal> + SafeParser<PVal = BVal>,
    BVal: DeepView<V = BVal> + PartialEq + Structural,
 {
    type PT = A::PT;

    open spec fn exec_inv(&self) -> bool {
        &&& Pair(&self.a, &self.b).exec_inv()
        &&& forall|v: BVal| v.deep_view() == v
    }

    fn parse(&self, ibuf: &I) -> PResult<Self::PT> {
        let (n, (v, vb)) = Pair(&self.a, &self.b).parse(ibuf)?;
        if vb == self.b_val {
            Ok((n, v))
        } else {
            Err(ParseError::non_canonical())
        }
    }
}

impl<Output: OutputBuf, A, B, BVal, T, const CHECK: bool> Generator<
    Output,
    T,
> for super::Terminated<A, B, BVal, CHECK> where
    T: DeepView,
    BVal: DeepView<V = BVal>,
    A: Generator<Output, T>,
    B: Generator<Output, BVal>,
 {
    #[verifier::prophetic]
    open spec fn exec_inv(&self) -> bool {
        &&& self.a.exec_inv()
        &&& self.b.exec_inv()
        &&& forall|v: BVal| v.deep_view() == v
    }

    fn generate(&mut self, g: &mut StdGen, obuf: &mut Output) {
        broadcast use crate::core::exec::output::outbuf_lemmas;

        self.a.generate(g, obuf);
        self.b.generate(g, obuf);
    }
}

impl<Output: OutputBuf, A, B, BVal, T, const CHECK: bool> Serializer<
    Output,
    T,
> for super::Terminated<A, B, BVal, CHECK> where
    T: DeepView,
    BVal: DeepView<V = BVal>,
    A: Serializer<Output, T>,
    B: Serializer<Output, BVal>,
 {
    #[verifier::prophetic]
    open spec fn exec_inv(&self) -> bool {
        &&& self.a.exec_inv()
        &&& self.b.exec_inv()
        &&& forall|v: BVal| v.deep_view() == v
    }

    fn serialize_into(&self, v: &T, obuf: &mut Output) {
        broadcast use crate::core::exec::output::outbuf_lemmas;

        self.a.serialize_into(v, obuf);
        self.b.serialize_into(&self.b_val, obuf);
    }
}

impl<A, B, BVal, T, const CHECK: bool> ByteLen<T> for super::Terminated<A, B, BVal, CHECK> where
    T: DeepView,
    BVal: DeepView<V = BVal>,
    A: ByteLen<T>,
    B: ByteLen<BVal>,
 {
    open spec fn exec_inv(&self) -> bool {
        &&& self.a.exec_inv()
        &&& self.b.exec_inv()
        &&& forall|v: BVal| v.deep_view() == v
    }

    fn length(&self, v: &T) -> (len: usize) {
        self.a.length(v) + self.b.length(&self.b_val)
    }
}

impl<A, B, BVal, T, const CHECK: bool> Prepare<T> for super::Terminated<A, B, BVal, CHECK> where
    T: DeepView,
    BVal: DeepView<V = BVal>,
    A: Prepare<T>,
    B: Prepare<BVal>,
 {
    open spec fn exec_inv(&self) -> bool {
        &&& self.a.exec_inv()
        &&& self.b.exec_inv()
        &&& forall|v: BVal| v.deep_view() == v
    }

    fn prepare(&self, v: &T) -> (checked: Result<usize, PreSerializeError>) {
        let la = self.a.prepare(v)?;
        let lb = self.b.prepare(&self.b_val)?;
        if let Some(total) = la.checked_add(lb) {
            Ok(total)
        } else {
            Err(PreSerializeError::length_too_large())
        }
    }
}

} // verus!
