pub mod aemulus {
    pub mod sim {
        pub mod v1 {
            include!("gen/aemulus/sim/v1/aemulus.sim.v1.rs");
        }
    }
    pub mod pf400 {
        pub mod v1 {
            include!("gen/aemulus/pf400/v1/aemulus.pf400.v1.rs");
        }
    }
}
