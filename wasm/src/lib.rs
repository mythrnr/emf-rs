#![no_std]

extern crate alloc;

use alloc::string::{String, ToString};

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = convertEmf2Svg)]
pub fn convert_emf_to_svg(buf: &[u8]) -> Result<String, JsValue> {
    // When the `console_error_panic_hook` feature is enabled, we can call
    // the `set_panic_hook` function at least once during
    // initialization, and then we will get better error messages if
    // our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    set_log_level("info");

    let wmf_player = wmf_core::converter::SVGPlayer::new();
    let emf_player = emf_core::converter::SVGPlayer::new();
    let converter =
        emf_core::converter::EMFConverter::new(buf, emf_player, wmf_player);
    let output =
        converter.run().map_err(|err| JsValue::from(err.to_string()))?;

    Ok(String::from_utf8_lossy(&output).to_string())
}

#[cfg(feature = "tracing")]
#[wasm_bindgen(js_name = setLogLevel)]
pub fn set_log_level(level: &str) {
    static IN: core::sync::atomic::AtomicBool =
        core::sync::atomic::AtomicBool::new(false);

    if !IN.load(core::sync::atomic::Ordering::Acquire) {
        IN.store(true, core::sync::atomic::Ordering::Release);

        tracing_wasm::set_as_global_default_with_config(
            tracing_wasm::WASMLayerConfigBuilder::new()
                .set_max_level(level.parse().expect("should be parsed"))
                .build(),
        );
    }
}

#[cfg(not(feature = "tracing"))]
#[wasm_bindgen(js_name = setLogLevel)]
pub fn set_log_level(_: &str) {}
