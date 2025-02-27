use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub version: String,
    pub timestamp: String,
    #[serde(alias = "messageType")]
    pub message_type: String,
    #[serde(alias = "correlationId")]
    pub correlation_id: String,
    pub source: String,
    pub destination: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum JobType {
    #[serde(rename = "multifactor_login")]
    MultiFactorLogin,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Job<T> {
    #[serde(alias = "jobId")]
    pub job_id: String,
    #[serde(alias = "jobType")]
    pub job_type: String,
    pub parameters: Option<T>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payload<P, M> {
    data: Option<P>,
    metadata: Option<M>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Body<T, P, M> {
    pub job: Job<T>,
    pub payload: Payload<P, M>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Envelope<T, P, M> {
    pub header: Header,
    pub body: Body<T, P, M>,
}
