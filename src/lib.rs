#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use stylus_sdk::prelude::*;

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
