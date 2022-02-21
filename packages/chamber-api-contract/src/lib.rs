pub mod v1 {
    pub mod add_one {
        #[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
        pub struct AddOneResponse {
            pub data: i64,
        }
    }
}
