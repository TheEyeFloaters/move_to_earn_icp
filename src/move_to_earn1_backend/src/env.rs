use candid::Principal;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use getrandom::register_custom_getrandom;


pub trait Environment {

    fn caller(&self) -> Principal;
    fn canister_id(&self) -> Principal;
    fn random_u32(&mut self) -> u32;
}

pub struct CanisterEnv {
    rng: StdRng,
}

impl CanisterEnv {
    pub fn new() -> Self {
        CanisterEnv {
            // Seed the PRNG with the current time.
            rng: {
                let mut seed = [0; 32];
                StdRng::from_seed(seed)
            },
        }
    }
}

impl Environment for CanisterEnv {


    fn caller(&self) -> Principal {
        ic_cdk::caller()
    }

    fn canister_id(&self) -> Principal {
        ic_cdk::id()
    }

    fn random_u32(&mut self) -> u32 {
        self.rng.next_u32()

    }
}

pub struct TestEnv {
    pub caller: Principal,
    pub canister_id: Principal,
    pub random_u32: u32,
}


pub struct EmptyEnv {}

impl Environment for EmptyEnv {

    fn caller(&self) -> Principal {
        Principal::anonymous()
    }

    fn canister_id(&self) -> Principal {
        Principal::anonymous()
    }

    fn random_u32(&mut self) -> u32 {
    0
    }
}