// Form for creating or editing an expense.
use yew::prelude::*;
use serde::Serialize;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

#[derive(Default, Serialize, Clone)]
pub struct ExpenseInput {
    pub amount: f64,
    pub description: String,
    pub category: String,
    pub expense_date: String,
}

#[function_component(ExpenseForm)]
pub fn expense_form() -> Html {
    let input = use_state(ExpenseInput::default);

    let onsubmit = {
        let input = input.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let input = input.clone();
            spawn_local(async move {
                let _ = Request::post("http://127.0.0.1:8081/expenses")
                    .header("Content-Type", "application/json")
                    .json(&*input)
                    .unwrap()
                    .send()
                    .await;
                // Optionally: reset form or show a message
            });
        })
    };

    html! {
        <div class="max-w-md mx-auto bg-white p-6 rounded shadow mb-6">
            <form class="space-y-4" onsubmit={onsubmit}>
                <input type="number" step="0.01" placeholder="Amount"
                    class="border rounded px-3 py-2 w-full"
                    value={input.amount.to_string()}
                    oninput={{
                        let input = input.clone();
                        Callback::from(move |e: InputEvent| {
                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            let amount = value.parse().unwrap_or(0.0);
                            input.set(ExpenseInput { amount, ..(*input).clone() });
                        })
                    }}
                    required=true
                />
                <input type="text" placeholder="Description"
                    class="border rounded px-3 py-2 w-full"
                    value={input.description.clone()}
                    oninput={{
                        let input = input.clone();
                        Callback::from(move |e: InputEvent| {
                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            input.set(ExpenseInput { description: value, ..(*input).clone() });
                        })
                    }}
                />
                <select
                    class="border rounded px-3 py-2 w-full"
                    value={input.category.clone()}
                    onchange={{
                        let input = input.clone();
                        Callback::from(move |e: Event| {
                            let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                            input.set(ExpenseInput { category: value, ..(*input).clone() });
                        })
                    }}
                    required=true
                >
                    <option value="">{"Select Category"}</option>
                    <option value="Work">{"Work"}</option>
                    <option value="Personal">{"Personal"}</option>
                    <option value="Food">{"Food"}</option>
                    <option value="Transport">{"Transport"}</option>
                    <option value="Utilities">{"Utilities"}</option>
                    <option value="Entertainment">{"Entertainment"}</option>
                    <option value="Others">{"Others"}</option>
                </select>
                <input type="date"
                    class="border rounded px-3 py-2 w-full"
                    value={input.expense_date.clone()}
                    onchange={{
                        let input = input.clone();
                        Callback::from(move |e: Event| {
                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            input.set(ExpenseInput { expense_date: value, ..(*input).clone() });
                        })
                    }}
                    required=true
                />
                <button type="submit"
                    class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded w-full font-semibold transition"
                >
                    {"Add Expense"}
                </button>
            </form>
        </div>
    }
}