mod spiral;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use spiral::Spiral;
use log::info;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(App)]
pub fn app() -> Html {
    let fibonacci_size = 20;

    fn get_fibonacci_sequence(n: usize) -> Vec<f64> {
        let mut res: Vec<f64> = vec![0.0; n + 1];
        
        for m in 0..n {
            match m {
                0 => { res[m] = 0.0; }
                1 => { res[m] = 1.0; }
                _ => { res[m] = res[m - 1] + res[m - 2]; }
            }
        }

        return res;
    }

    html! {
        <main class="flex-center">
            <Spiral fibonacci_sequence={get_fibonacci_sequence(fibonacci_size)}></Spiral>
        </main>
    }
}
