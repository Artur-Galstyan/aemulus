pub mod sim;

pub struct ShakerModel {
    pub id: &'static str,
    pub min_rpm: i64,
    pub max_rpm: i64,
}

pub const MODELS: &[ShakerModel] = &[ShakerModel {
    id: "qinstruments/bioshake-3000-elm",
    min_rpm: 200,
    max_rpm: 3000,
}];
