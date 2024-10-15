use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = convertEmf2Svg)]
pub fn convert_emf_to_svg(buf: &[u8]) -> Result<String, JsValue> {
    set_log_level("info");

    let mut output_wmf: Vec<u8> = vec![];
    let mut output_emf: Vec<u8> = vec![];
    let out_wmf = std::io::BufWriter::new(&mut output_wmf);
    let out_emf = std::io::BufWriter::new(&mut output_emf);

    let wmf_player = wmf_core::converter::SVGPlayer::new(out_wmf);
    let emf_player = emf_core::converter::SVGPlayer::new(out_emf);
    let converter =
        emf_core::converter::EMFConverter::new(buf, emf_player, wmf_player);

    if let Err(err) = converter.run() {
        return Err(err.to_string().into());
    }

    if !output_wmf.is_empty() {
        Ok(String::from_utf8_lossy(&output_wmf).to_string())
    } else {
        Ok(String::from_utf8_lossy(&output_emf).to_string())
    }
}

static INIT: std::sync::Once = std::sync::Once::new();

#[wasm_bindgen(js_name = setLogLevel)]
pub fn set_log_level(level: &str) {
    INIT.call_once(|| {
        // When the `console_error_panic_hook` feature is enabled, we can call
        // the `set_panic_hook` function at least once during
        // initialization, and then we will get better error messages if
        // our code ever panics.
        //
        // For more details see
        // https://github.com/rustwasm/console_error_panic_hook#readme
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        tracing_wasm::set_as_global_default_with_config(
            tracing_wasm::WASMLayerConfigBuilder::new()
                .set_max_level(level.parse().expect("should be parsed"))
                .build(),
        );
    });
}
