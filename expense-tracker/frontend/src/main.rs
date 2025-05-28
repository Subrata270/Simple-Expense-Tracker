mod components;

use components::{
    expense_list::ExpenseList,
    expense_form::ExpenseForm,
    search_bar::SearchBar,
    summary_view::SummaryView,
    chart::Chart,
};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="container mx-auto p-4">
            <h1 class="text-2xl font-bold mb-4">{"Expense Tracker"}</h1>
            <SummaryView />
            <SearchBar />
            <ExpenseForm />
            <ExpenseList />
            <Chart /> 
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}