// Lets users filter expenses by description or category.
use yew::prelude::*;

#[function_component(SearchBar)]
pub fn search_bar() -> Html {
    html! {
        <div class="mb-4">
            <input type="text" placeholder="Search by description or category" class="border p-2 w-1/2" />
            // Add logic to filter the expense list as you expand your app
        </div>
    }
}