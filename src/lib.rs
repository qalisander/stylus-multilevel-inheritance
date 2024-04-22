#![cfg_attr(not(test), no_main, no_std)]
extern crate alloc;

use stylus_sdk::prelude::{entrypoint, external, sol_storage};

sol_storage! {
    #[entrypoint]
    struct TopLevelContract {
        #[borrow]
        SecondLevelContract child;
    }
}

#[external]
#[inherit(SecondLevelContract)]
impl TopLevelContract {}

sol_storage! {
    struct SecondLevelContract {
        #[borrow]
        ThirdLevelContract child;
    }
}

#[external]
#[inherit(ThirdLevelContract)]
impl SecondLevelContract {}

sol_storage! {
    struct ThirdLevelContract {}
}


#[external]
impl ThirdLevelContract {}
