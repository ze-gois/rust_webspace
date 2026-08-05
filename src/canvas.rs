use web_sys::{HtmlCanvasElement, MouseEvent};

pub fn mouse_position(canvas: &HtmlCanvasElement, event: &MouseEvent) -> (f64, f64) {
    let rect = canvas.get_bounding_client_rect();
    let scale_x = canvas.width() as f64 / rect.width().max(1.0);
    let scale_y = canvas.height() as f64 / rect.height().max(1.0);
    (
        (event.client_x() as f64 - rect.left()) * scale_x,
        (event.client_y() as f64 - rect.top()) * scale_y,
    )
}
