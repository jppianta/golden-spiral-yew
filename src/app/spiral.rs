use std::f64::consts::{PI, SQRT_2};

use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use yew::{function_component, html, use_effect, use_node_ref, Html, Properties};
use wasm_bindgen::{JsCast, JsValue};
use log::info;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub fibonacci_sequence: Vec<f64>,
}

#[function_component(Spiral)]
pub fn spiral(props: &Props) -> Html {
    let canvas_ref = use_node_ref();

    fn get_size(canvas: &HtmlCanvasElement) -> (f64, f64) {
        return (canvas.width().into(), canvas.height().into())
    }

    fn get_center(size: (f64, f64)) -> (f64, f64) {
        return (size.0 / 2.0, size.1 / 2.0);
    }

    fn draw_circle(ctx: &CanvasRenderingContext2d, x: f64, y: f64, radius: f64, color: &str) {
        ctx.begin_path();
        ctx.arc(x, y, radius, PI / 4.0, 3.0 * PI / 4.0).expect("failed to draw arc");
        ctx.set_stroke_style(&JsValue::from_str(color));
        ctx.stroke();
        ctx.close_path();
    }

    fn draw(fibonacci: Vec<f64>, ctx: &CanvasRenderingContext2d, canvas: &HtmlCanvasElement, scale: f64) {
        ctx.save();

        let color = "#ffffff";
        let size = get_size(canvas);
        let mut end = get_center(size);

        ctx.translate(end.0, end.1).expect("failed to translate");
        for fi in fibonacci.iter() {
            draw_circle(ctx, (fi * SQRT_2 * scale) / 2.0, -(fi * SQRT_2 * scale) / 2.0, fi * scale, color);
            end.0 = 0.0;
            end.1 = *fi;
            ctx.translate(end.1 * SQRT_2 * scale, end.0).expect("failed to translate");
            ctx.rotate(PI / 180.0 * -90.0).expect("failed to rotate");
        }

        ctx.restore();
    }

    {
        let canvas_ref = canvas_ref.clone();
        let fibonacci_copy = props.fibonacci_sequence.clone();

        use_effect(move || {
            let canvas = canvas_ref
                .cast::<HtmlCanvasElement>()
                .expect("canvas_ref not attached to canvas element");

            let context = canvas
                .get_context("2d")
                .expect("2d context not available")
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            draw(fibonacci_copy, &context, &canvas, 1.5);
        });
    }

    html! {
        <canvas class="canvas" width={700} height={500} ref={canvas_ref}></canvas>
    }
}