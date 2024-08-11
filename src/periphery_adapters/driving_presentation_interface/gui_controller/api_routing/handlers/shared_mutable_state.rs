use std::sync::{Arc, Mutex};

use axum::extract::State;

pub async fn shared_mutable_state(State(message): State<Arc<Mutex<String>>>) {
    let mut message = message.lock().expect("mutex was poisoned");
    message.push_str("!");

    dbg!(&*message);
}
