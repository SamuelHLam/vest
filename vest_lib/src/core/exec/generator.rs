use crate::core::exec::output::*;

use rand::{Rng, RngExt, SeedableRng};
use rand::rngs::StdRng;

use vstd::prelude::*;

verus! {

pub struct StdGen {
    pub rng: StdRng,
    pub bytes: usize,
}

pub trait Generator<Output, T> where
    Output: OutputBuf,
    // Self: SpecByteLen<T = T::V> + SpecSerializer<SVal = T::V> + Consistency<Val = T::V>,
    T: DeepView,
{
    fn generate(&mut self, g: &mut StdGen, obuf: &mut Output);
    fn generate_val(&mut self, g: &mut StdGen) -> T;
}

impl<Output, T, G> Generator<Output, T> for &G where
    Output: OutputBuf,
    T: DeepView,
    G: Generator<Output, T>,
 {
    #[verifier::prophetic]
    open spec fn exec_inv(&self) -> bool {
        (*self).exec_inv()
    }

    fn generate(&mut self, g: &mut StdGen, obuf: &mut Output) {
        (*self).generate(g, obuf);
    }

    fn generate_val(&mut self, g: &mut StdGen) -> T {
        (*self).generate_val(g)
    }
}

}