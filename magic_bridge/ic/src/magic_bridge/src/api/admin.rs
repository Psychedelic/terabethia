use crate::magic::STATE;

pub fn is_authorized() -> Result<(), String> {
    STATE.with(|s| s.is_authorized())
}


