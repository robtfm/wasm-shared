use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_wasm_init() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"WASM Root Initialized (console_error_panic_hook set)".into());
}

#[derive(Debug)]
pub struct Ipfs {
    pub realm: RwLock<String>,
}

impl Ipfs {
    pub fn get_realm(&self) -> String {
        self.realm.read().clone()
    }

    pub fn set_realm(&self, new_realm: String) -> () {
        *self.realm.write() = new_realm;
    }
}

#[derive(Debug)]
pub struct SharedResources {
    ipfs: Ipfs,
}

// Static storage shared data
static SHARED_DATA: OnceCell<SharedResources> = OnceCell::new();

#[wasm_bindgen]
pub fn initialize_shared_resources() -> Result<(), JsValue> {
    let shared_resources = SharedResources {
        ipfs: Ipfs {
            realm: RwLock::new("no realm".into()),
        },
    };

    SHARED_DATA
        .set(shared_resources)
        .map_err(|e| JsValue::from_str(&format!("{e:?}")))
}

#[wasm_bindgen]
pub fn read_shared_from_worker() -> Result<String, JsValue> {
    Ok(SHARED_DATA
        .get()
        .ok_or_else(|| JsValue::from_str("shared resources not initialized"))?
        .ipfs
        .get_realm())
}

#[wasm_bindgen]
pub fn set_realm(new_realm: String) -> Result<(), JsValue> {
    SHARED_DATA
        .get()
        .ok_or_else(|| JsValue::from_str("shared resources not initialized"))?
        .ipfs
        .set_realm(new_realm);

    Ok(())
}
