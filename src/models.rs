#[derive(Clone, Debug, iDeserialize, Serialize)]
pub struct Customer {
    pub guid: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub address: String,
}