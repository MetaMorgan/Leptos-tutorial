use leptos::{component, create_signal, view, CollectView, IntoView, SignalUpdate};

/// A list of counters, without the ability to add or remove any
#[component]
pub fn StaticList(
    /// how many counters to include in this list
    length: usize
) -> impl IntoView {
    // create the counter signals that start at incrementing numbers
    let counters = (1..=length).map(|idx| create_signal(idx));

    // when you have a list that doesnt change, you manipulate it using ordinary Rust
    // iterator and collect it inot a Vec<_> to insert it into the DOM
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button on:click=move |_| set_count.update(|n| *n +=1)>{count}</button>
                </li>
            }
    })
    .collect_view();

    // note that if counter_buttons were a reactive list and its value changed,
    // this would be very inefficient: it would render every row every time the list changed.
    view! {
        <ul>{counter_buttons}</ul>
    }
}

