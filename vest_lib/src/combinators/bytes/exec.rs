//! Executable implementations for fixed- and variable-length bytes.
use crate::combinators::{AsLen, Tail};
use crate::core::exec::input::{InputBuf, InputSlice};
use crate::core::exec::output::*;
use crate::core::exec::{
    parser::{PResult, Parser},
    serializer::{ByteLen, ComplianceErrorKind, PreSerializeError, Prepare, Serializer},
    generator::{StdGen, Generator},
    ParseError,
};
use crate::core::spec::{Consistency, SpecByteLen, SpecParser};
use rand::{Rng, RngExt, SeedableRng};
use rand::rngs::StdRng;
use vstd::prelude::*;
use OutputBuf;

verus! {

impl<const N: usize, I: InputBuf> Parser<I> for super::Fixed<N> {
    type PT = I;

    fn parse(&self, ibuf: &I) -> PResult<Self::PT> {
        if ibuf.len() < N {
            Err(ParseError::unexpected_eof())
        } else {
            Ok((N, ibuf.take(N)))
        }
    }
}

impl<Output: OutputBuf, const N: usize> Serializer<Output, [u8]> for super::Fixed<N> {
    fn serialize_into(&self, v: &[u8], obuf: &mut Output) {
        obuf.write_bytes(v);
    }
}

impl<Output: OutputBuf, const N: usize> Generator<Output, [u8]> for super::Fixed<N> {
    fn generate(&mut self, g: &mut StdGen, obuf: &mut Output){
        let mut byte = [0u8; N];
        g.rng.fill(&mut byte);
        obuf.write_bytes(&byte);
    }
}

impl<'i, Output: OutputBuf, const N: usize> Serializer<Output, &'i [u8]> for super::Fixed<N> {
    fn serialize_into(&self, v: &&'i [u8], obuf: &mut Output) {
        obuf.write_bytes(*v);
    }
}

impl<'i, Output: OutputBuf, const N: usize> Generator<Output, &'i [u8]> for super::Fixed<N> {
    fn generate(&mut self, g: &mut StdGen, obuf: &mut Output) {
        let mut byte = [0u8; N];
        g.rng.fill(&mut byte);
        obuf.write_bytes(&byte);
    }
}

impl<Output: OutputBuf, const N: usize> Serializer<Output, [u8; N]> for super::Fixed<N> {
    fn serialize_into(&self, v: &[u8; N], obuf: &mut Output) {
        obuf.write_bytes(v);
    }
}

impl<Output: OutputBuf, const N: usize> Generator<Output, [u8; N]> for super::Fixed<N> {
    fn generate(&mut self, g: &mut StdGen, obuf: &mut Output) {
        let mut byte = [0u8; N];
        g.rng.fill(&mut byte);
        obuf.write_bytes(&byte);
    }
}

// impl<Output: OutputBuf, const N: usize> Serializer<Output, [u8; N]> for super::Fixed<N> {
//     fn ex_serialize(&self, v: &[u8; N], obuf: &mut Output) {
//         obuf.write_bytes(v);
//     }
// }
impl<const N: usize> ByteLen<[u8]> for super::Fixed<N> {
    fn length(&self, v: &[u8]) -> (len: usize) {
        v.len()
    }
}

impl<'i, const N: usize> ByteLen<&'i [u8]> for super::Fixed<N> {
    fn length(&self, v: &&'i [u8]) -> (len: usize) {
        v.len()
    }
}

impl<const N: usize> Prepare<[u8]> for super::Fixed<N> {
    fn prepare(&self, v: &[u8]) -> (checked: Result<usize, PreSerializeError>) {
        if v.len() == N {
            Ok(N)
        } else {
            Err(PreSerializeError::not_compliant(ComplianceErrorKind::LengthInconsistent))
        }
    }
}

impl<'i, const N: usize> Prepare<&'i [u8]> for super::Fixed<N> {
    fn prepare(&self, v: &&'i [u8]) -> (checked: Result<usize, PreSerializeError>) {
        if v.len() == N {
            Ok(N)
        } else {
            Err(PreSerializeError::not_compliant(ComplianceErrorKind::LengthInconsistent))
        }
    }
}

impl<Len: AsLen, I: InputBuf> Parser<I> for super::Varied<Len> {
    type PT = I;

    fn parse(&self, ibuf: &I) -> PResult<Self::PT> {
        let len = self.0.get();
        if ibuf.len() < len {
            Err(ParseError::unexpected_eof())
        } else {
            Ok((len, ibuf.take(len)))
        }
    }
}

impl<Output: OutputBuf, Len: AsLen> Serializer<Output, [u8]> for super::Varied<Len> {
    fn serialize_into(&self, v: &[u8], obuf: &mut Output) {
        obuf.write_bytes(v);
    }
}

impl<Output: OutputBuf, Len: AsLen> Generator<Output, [u8]> for super::Varied<Len> {
    fn generate(&mut self, g: &mut StdGen, obuf: &mut Output) {
        let mut byte = [0u8; 1];
        for _ in 0..self.0.get() {
            g.rng.fill(&mut byte);
            obuf.write_bytes(&byte);
        }
    }
}

impl<'i, Output: OutputBuf, Len: AsLen> Serializer<Output, &'i [u8]> for super::Varied<Len> {
    fn serialize_into(&self, v: &&'i [u8], obuf: &mut Output) {
        obuf.write_bytes(*v);
    }
}

impl<'i, Output: OutputBuf, Len: AsLen> Generator<Output, &'i [u8]> for super::Varied<Len> {
    fn generate(&mut self, g: &mut StdGen, obuf: &mut Output) {
        let mut byte = [0u8; 1];
        for _ in 0..self.0.get() {
            g.rng.fill(&mut byte);
            obuf.write_bytes(&byte);
        }
    }
}

impl<Len: AsLen> ByteLen<[u8]> for super::Varied<Len> {
    fn length(&self, v: &[u8]) -> (len: usize) {
        v.len()
    }
}

impl<'i, Len: AsLen> ByteLen<&'i [u8]> for super::Varied<Len> {
    fn length(&self, v: &&'i [u8]) -> (len: usize) {
        v.len()
    }
}

impl<Len: AsLen> Prepare<[u8]> for super::Varied<Len> {
    fn prepare(&self, v: &[u8]) -> (checked: Result<usize, PreSerializeError>) {
        if v.len() == self.0.get() {
            Ok(v.len())
        } else {
            Err(PreSerializeError::not_compliant(ComplianceErrorKind::LengthInconsistent))
        }
    }
}

impl<'i, Len: AsLen> Prepare<&'i [u8]> for super::Varied<Len> {
    fn prepare(&self, v: &&'i [u8]) -> (checked: Result<usize, PreSerializeError>) {
        if v.len() == self.0.get() {
            Ok(v.len())
        } else {
            Err(PreSerializeError::not_compliant(ComplianceErrorKind::LengthInconsistent))
        }
    }
}

impl<I, Len, Inner> Parser<I> for super::ExactLen<Inner, Len> where
    I: InputBuf,
    Len: AsLen,
    Inner: Parser<I>,
 {
    type PT = Inner::PT;

    open spec fn exec_inv(&self) -> bool {
        self.1.exec_inv()
    }

    fn parse(&self, ibuf: &I) -> PResult<Self::PT> {
        super::AndThen(super::Varied(self.0), &self.1).parse(ibuf)
    }
}

impl<I: InputBuf, A, Then> Parser<I> for super::AndThen<A, Then> where
    A: Parser<I, PT = I, PVal = Seq<u8>>,
    Then: Parser<I>,
 {
    type PT = Then::PT;

    open spec fn exec_inv(&self) -> bool {
        &&& self.0.exec_inv()
        &&& self.1.exec_inv()
    }

    fn parse(&self, ibuf: &I) -> PResult<Self::PT> {
        assert(self.exec_inv());

        let (len_a, chunk) = self.0.parse(ibuf)?;
        proof {
            chunk.deep_view_eq_view();
        }
        let (len_b, v) = self.1.parse(&chunk)?;
        if len_b == len_a {
            Ok((len_b, v))
        } else {
            Err(ParseError::length_mismatch())
        }
    }
}

impl<Output: OutputBuf, Len, Inner, T> Serializer<Output, T> for super::ExactLen<Inner, Len> where
    Len: AsLen,
    T: DeepView + ?Sized,
    Inner: Serializer<Output, T> + SpecByteLen<T = T::V>,
 {
    #[verifier::prophetic]
    open spec fn exec_inv(&self) -> bool {
        self.1.exec_inv()
    }

    fn serialize_into(&self, v: &T, obuf: &mut Output) {
        self.1.serialize_into(v, obuf);
    }
}

impl<Output: OutputBuf, Len, Inner, T> Generator<Output, T> for super::ExactLen<Inner, Len> where
    Len: AsLen,
    T: DeepView + ?Sized,
    Inner: Generator<Output, T> + SpecByteLen<T = T::V>,
 {
    fn generate(&mut self, g: &mut StdGen, obuf: &mut Output) {
        self.1.generate(g, obuf);
    }
}

impl<Output: OutputBuf, Then, T> Serializer<Output, T> for super::AndThen<Tail, Then> where
    T: DeepView + ?Sized,
    Then: Serializer<Output, T>,
 {
    #[verifier::prophetic]
    open spec fn exec_inv(&self) -> bool {
        self.1.exec_inv()
    }

    fn serialize_into(&self, v: &T, obuf: &mut Output) {
        self.1.serialize_into(v, obuf);
    }
}

impl<Output: OutputBuf, Then, T> Generator<Output, T> for super::AndThen<Tail, Then> where
    T: DeepView + ?Sized,
    Then: Generator<Output, T>,
 {
    fn generate(&mut self, g: &mut StdGen, obuf: &mut Output) {
        self.1.generate(g, obuf);
    }
}

impl<Len, Inner, InnerST> ByteLen<InnerST> for super::ExactLen<Inner, Len> where
    Len: AsLen,
    InnerST: DeepView + ?Sized,
    Inner: ByteLen<InnerST>,
 {
    open spec fn exec_inv(&self) -> bool {
        self.1.exec_inv()
    }

    fn length(&self, v: &InnerST) -> (len: usize) {
        self.1.length(v)
    }
}

impl<Len, Inner, InnerST> Prepare<InnerST> for super::ExactLen<Inner, Len> where
    Len: AsLen,
    InnerST: DeepView + ?Sized,
    Inner: Prepare<InnerST>,
 {
    open spec fn exec_inv(&self) -> bool {
        self.1.exec_inv()
    }

    fn prepare(&self, v: &InnerST) -> (checked: Result<usize, PreSerializeError>) {
        let len = self.1.prepare(v)?;
        if len == self.0.get() {
            Ok(len)
        } else {
            Err(PreSerializeError::not_compliant(ComplianceErrorKind::LengthInconsistent))
        }
    }
}

impl<Then, T> Prepare<T> for super::AndThen<Tail, Then> where
    T: DeepView + ?Sized,
    Then: Prepare<T>,
 {
    open spec fn exec_inv(&self) -> bool {
        self.1.exec_inv()
    }

    fn prepare(&self, v: &T) -> (checked: Result<usize, PreSerializeError>) {
        let len = self.1.prepare(v)?;
        proof {
            let chunk = Seq::new(len as nat, |_i| 0u8);
            assert(self.0.consistent(chunk));
        }
        Ok(len)
    }
}

impl<A, Then, ThenST> ByteLen<ThenST> for super::AndThen<A, Then> where
    ThenST: DeepView + ?Sized,
    Then: ByteLen<ThenST>,
 {
    open spec fn exec_inv(&self) -> bool {
        self.1.exec_inv()
    }

    fn length(&self, v: &ThenST) -> (len: usize) {
        self.1.length(v)
    }
}

} // verus!
