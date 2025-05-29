use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

#[derive(Deserialize, Clone)]
struct Summary {
    total: Option<f64>,
}

#[function_component(SummaryView)]
pub fn summary_view() -> Html {
    let summary = use_state(|| Summary { total: Some(0.0) });

    {
        let summary = summary.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let resp = Request::get("http://127.0.0.1:8081/expenses/summary")
                    .send()
                    .await;
                let data = match resp {
                    Ok(r) => r.json::<Summary>().await.unwrap_or(Summary { total: Some(0.0) }),
                    Err(_) => Summary { total: Some(0.0) },
                };
                summary.set(data);
            });
            || ()
        });
    }

    html! {
        <div class="mb-4 font-semibold">
            { format!("Total expenses this month: ₹{:.2}", summary.total.unwrap_or(0.0)) }
        </div>
    }
}