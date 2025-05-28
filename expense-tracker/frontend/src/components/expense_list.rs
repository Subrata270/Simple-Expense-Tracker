use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use gloo_net::http::Request;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Expense {
    pub id: String,
    pub amount: f64,
    pub description: Option<String>,
    pub category: String,
    pub expense_date: String,
}

#[function_component(ExpenseList)]
pub fn expense_list() -> Html {
    let expenses = use_state(|| vec![]);

    {
        let expenses = expenses.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let resp = Request::get("http://127.0.0.1:8081/expenses")
                    .send()
                    .await;
                let data = match resp {
                    Ok(r) => r.json::<Vec<Expense>>().await.unwrap_or_default(),
                    Err(_) => vec![],
                };
                expenses.set(data);
            });
            || ()
        });
    }

    html! {
        <div class="mt-4">
            <h2 class="text-xl font-semibold mb-2">{"Expenses"}</h2>
            <table class="min-w-full border">
                <thead>
                    <tr>
                        <th>{"Amount"}</th>
                        <th>{"Category"}</th>
                        <th>{"Description"}</th>
                        <th>{"Date"}</th>
                    </tr>
                </thead>
                <tbody>
                    { for expenses.iter().map(|e| html! {
                        <tr>
                            <td>{ e.amount }</td>
                            <td>{ &e.category }</td>
                            <td>{ e.description.clone().unwrap_or_default() }</td>
                            <td>{ &e.expense_date }</td>
                        </tr>
                    })}
                </tbody>
            </table>
        </div>
    }
}