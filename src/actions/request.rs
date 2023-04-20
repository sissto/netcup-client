use serde::Serialize;

#[derive(Serialize)]
pub struct RequestAction<Action> {
    action: String,
    param: Action,
}

pub struct RequestActionBuilder;

impl RequestActionBuilder {
    pub fn build<T>(action: &str, param: T) -> RequestAction<T> {
        RequestAction {
            action: action.to_owned(),
            param,
        }
    }
}
