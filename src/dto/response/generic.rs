use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DefaultServiceResponse {
    pub msg: String
}