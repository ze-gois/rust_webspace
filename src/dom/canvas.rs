//! Shared canvas and note helpers for visualizers.
//!
//! Visualizers all use semitone offsets from A4. These helpers convert those
//! offsets into pitch classes, display names, note ranges, and active playback
//! indices.

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

/// Find a canvas by id and return it with a 2D rendering context.
pub fn canvas_context(
    canvas_id: &str,
    label: &str,
) -> Result<(HtmlCanvasElement, CanvasRenderingContext2d), JsValue> {
    let document = window()
        .ok_or_else(|| JsValue::from_str("window is not available"))?
        .document()
        .ok_or_else(|| JsValue::from_str("document is not available"))?;

    let canvas = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| JsValue::from_str(&format!("{label} was not found")))?
        .dyn_into::<HtmlCanvasElement>()?;

    let ctx = canvas
        .get_context("2d")?
        .ok_or_else(|| JsValue::from_str("2D canvas context is not available"))?
        .dyn_into::<CanvasRenderingContext2d>()?;

    Ok((canvas, ctx))
}
