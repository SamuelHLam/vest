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
    T: DeepView + ?Sized,
{
    fn generate(&mut self, g: &mut StdGen, obuf: &mut Output);
}

}