// Fetches and displays the total expenses for the current month.
use yew::prelude::*;
use serde::Deserialize;

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
            wasm_bindgen_futures::spawn_local(async move {
                let resp = reqwest::get("http://127.0.0.1:8080/expenses/summary")
                    .await
                    .ok()
                    .and_then(|r| r.json::<Summary>().await.ok())
                    .unwrap_or(Summary { total: Some(0.0) });
                summary.set(resp);
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