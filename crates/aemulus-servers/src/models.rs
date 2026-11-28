#[derive(Clone, Copy)]
pub struct ShakerSpec {
    pub min_rpm: i64,
    pub max_rpm: i64,
}

pub enum Kind {
    Pf400,
    Shaker(ShakerSpec),
}

#[cfg_attr(feature = "python", pyo3::pyclass(eq, eq_int, from_py_object))]
#[derive(Clone, Copy, PartialEq, clap::ValueEnum)]
pub enum ModelIdentifier {
    PrecisePf400,
    QInstrumentsBioshake3000Elm,
}

impl ModelIdentifier {
    pub fn kind(self) -> Kind {
        match self {
            ModelIdentifier::PrecisePf400 => Kind::Pf400,
            ModelIdentifier::QInstrumentsBioshake3000Elm => Kind::Shaker(ShakerSpec {
                min_rpm: 200,
                max_rpm: 3000,
            }),
        }
    }
}
