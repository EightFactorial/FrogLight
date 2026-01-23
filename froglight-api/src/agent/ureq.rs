use ureq::Agent;

use crate::agent::{AgentError, GetOptions, GetResponse, NetworkAgent, PutOptions, PutResponse};

impl NetworkAgent for Agent {
    fn get(&self, url: &str, _opts: GetOptions) -> Result<GetResponse, AgentError> {
        let mut response = self.get(url).call().map_err(|err| AgentError::Other(Box::new(err)))?;

        if response.status().is_success() {
            let headers: Vec<_> = response
                .headers()
                .iter()
                .map(|(h, v)| (h.to_string(), v.as_bytes().to_vec()))
                .collect();
            let status = response.status().as_u16();
            let data = response
                .body_mut()
                .read_to_vec()
                .map_err(|err| AgentError::Other(Box::new(err)))?;
            Ok(GetResponse { status, data, headers })
        } else {
            Err(AgentError::Http(response.status().as_u16()))
        }
    }

    fn put(&self, url: &str, data: Vec<u8>, _opts: PutOptions) -> Result<PutResponse, AgentError> {
        let response = self.put(url).send(data).map_err(|err| AgentError::Other(Box::new(err)))?;

        if response.status().is_success() {
            Ok(PutResponse { status: response.status().as_u16() })
        } else {
            Err(AgentError::Http(response.status().as_u16()))
        }
    }
}
