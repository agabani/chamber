pub mod v1 {
    use chamber_api_contract::v1::add_one::AddOneResponse;
    use reqwasm::http::Request;

    pub async fn add_one(number: i64) -> AddOneResponse {
        Request::get(&format!("/api/v1/add_one/{}", number))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
}
