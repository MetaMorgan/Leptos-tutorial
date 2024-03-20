use gloo_timers::future::TimeoutFuture;
use leptos::*;

#[component]    
pub fn AsyncComponent() -> impl IntoView {
    // this count is our synchrounous, local state
    let (count, set_count) = create_signal(0);

    // create_resource takes two arguments after its scope
    let async_data = create_resource(
        // the first is the source signal
        count,
        // the second is the loader
        // it takes the source signals value as its argument
        // and does some async work
        |value| async move { load_data(value).await }
    );
    // whenever the source signal changes, the loader reloads

    // you can also create resources that only load once
    // just return the unit type () from the source signal
    // that doesn't depend on aything: we just load it once
    let stable = create_resource(|| (), |_| async move { load_data(1).await });

    // we can access the resource values with .get()
    // this will reactively return none before the futrue has resolved
    // and update to Some(T) when it has resolved
    let async_result = move || {
        async_data
            .get()
            .map(|value| format!("Server returned {value:?}"))
            .unwrap_or_else(|| "Loading...".into())
    };

    // the resources loading() method gives us a
    // signal to indicate whether it's currently loading
    let loading = async_data.loading();
    let is_loading = move || if loading() { "Loading..."} else {"Idle"};

    // by updating the count we trigger the async_data resource since the source (count) was incremented
    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me"
        </button>
        <p>
            <code>"stable"</code>": " {move || stable.get()}
        </p>
        <p>
            <code>"count"</code>": " {count}
        </p>
        <p>
            <code>"async_value"</code>": "
            {async_result}
            <br/>
            {is_loading}
        </p>
    }.into_view()
}



// Here we define an async function
// this could be anything: a network request, database read, etc.
// here wejust multiply a number by 10 after timer
pub async fn load_data(value: i32) -> i32 {
    // fake a one-second delay
    TimeoutFuture::new(1_000).await;
    value * 10
}