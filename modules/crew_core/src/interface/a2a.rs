// Include the generated code
pub mod a2a_protocol {
    tonic::include_proto!("a2a.v1");
}

pub use a2a_protocol::*;

use tonic::transport::Channel;

pub struct AgentLink {
    // Path mirrors package: a2a.v1 -> a2a::v1
    // Service: A2AService -> a2a_service_client
    client: Option<a2a_protocol::a2a_service_client::A2aServiceClient<Channel>>,
}

impl AgentLink {
    pub fn new() -> Self {
        Self { client: None }
    }

    pub async fn connect(&mut self, addr: String) -> Result<(), Box<dyn std::error::Error>> {
        let client = a2a_protocol::a2a_service_client::A2aServiceClient::connect(addr).await?;
        self.client = Some(client);
        Ok(())
    }
}
