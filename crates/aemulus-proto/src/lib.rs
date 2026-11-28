pub mod sim {
    pub mod v1 {
        include!("gen/aemulus/sim/v1/aemulus.sim.v1.rs");
    }
}

pub mod sila2 {
    pub mod org {
        pub mod silastandard {
            include!("gen/sila2/org/silastandard/sila2.org.silastandard.rs");
        }
    }
    pub mod dev {
        pub mod arturgalstyan {
            pub mod robots {
                pub mod pf400 {
                    pub mod v1 {
                        include!(
                            "gen/sila2/dev/arturgalstyan/robots/pf400/v1/sila2.dev.arturgalstyan.robots.pf400.v1.rs"
                        );
                    }
                }
                pub mod starlet {
                    pub mod v1 {
                        include!(
                            "gen/sila2/dev/arturgalstyan/robots/starlet/v1/sila2.dev.arturgalstyan.robots.starlet.v1.rs"
                        );
                    }
                }
            }
            pub mod shakers {
                pub mod shakercontroller {
                    pub mod v1 {
                        include!(
                            "gen/sila2/dev/arturgalstyan/shakers/shakercontroller/v1/sila2.dev.arturgalstyan.shakers.shakercontroller.v1.rs"
                        );
                    }
                }
            }
        }
    }
}
