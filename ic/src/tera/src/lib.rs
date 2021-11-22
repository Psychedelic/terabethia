mod Tera {
    use ic_cdk::export::Principal;
    use ic_cdk::{api, caller};

    #[update(name = "receiveMessageFromL1")]
    pub async fn receive(from: String, to: Principal, payload: Vec<u8>) -> Result<bool, String> {
        if api::id() == caller() {
            return Err("Attempted to call handler on self. This is not allowed..".to_string());
        }

        match api::call::call_raw(to.clone(), "handler", payload, 0).await {
            Ok(x) => Ok(true),
            Err((code, msg)) => Err(format!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            )),
        }
    }
}
