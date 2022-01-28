use candid::{candid_method, Principal};
use ic_cdk_macros::update;

use crate::tera::STATE;

pub fn is_authorized() -> Result<(), String> {
    STATE.with(|s| s.is_authorized())
}

#[update(name = "authorize")]
#[candid_method(update)]
fn authorize(other: Principal) {
    STATE.with(|s| s.authorize(other))
}

#[cfg(test)]
mod tests {
    use ic_kit::{mock_principals, MockContext};

    use super::*;

    fn before_each() -> &'static mut MockContext {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject()
    }

    #[test]
    fn test_authorize() {
        let mock_ctx = before_each();

        authorize(mock_principals::bob());

        mock_ctx.update_caller(mock_principals::bob());
        let is_authorized = STATE.with(|s| s.is_authorized());

        println!("{:#?}", is_authorized);

        assert!(is_authorized.is_ok());
    }
}
