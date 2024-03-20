use leptos::{component, create_signal, view, IntoView, Show, SignalUpdate};

#[component]
pub fn ControlFlow() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value() & 1 == 1;
    let odd_text = move || if is_odd() { Some("How odd!") } else { None };

    view! {
        <h1>"Control Flow"</h1>

        // simple ui to update and show a value
        <button on:click=move |_| set_value.update(|n| *n += 1)>
            "+1"
        </button>
        <p>"Value is: " {value}</p>

        <hr />

        <h2><code>"Option<T>"</code></h2>
        // for any T that implements IntoView, so does Option<T>

        <h2>{odd_text}</h2>
        // this means you can use Option methods on it
        <p>{move || odd_text().map(|text| text.len())}</p>

        <h2>"Conditional Logic"</h2>
        // you can do dynamic conditional if-then-else logic in several ways
        //
        // a. An if expression in a function
        //   This will simple re-render every time the value changes,
        // which makes it good for ligthweight UI
        <p>
            {move || if is_odd() {
                "Odd"
            } else {
                "Even"
            }}
        </p>

        // b. Toggling some kind of class
        //  This is smart for an elemnt thats going to toggled often, becuase
        // it doesnt destroy it in between states
        // you can dfind the hidden class in index.html
        <p class:hidden=is_odd>"Appears if even"</p>

        // c. the <show/> component
        //   this only renders the fallback and the child once, lazilty,
        //   and toggles between them when needed.  this makes it more efficent in 
        //   many cases then an {move || if ... } block
        <Show when=is_odd
            fallback=|| view! {<p>"Even steven"</p>}
        >
            <p>"Oddment"</p>
        </Show>

        // d. because bool::then() converts a bool to Option you can use it to create a show/hide togle
        {move || is_odd().then(|| view! { <p>"Oddity"</p>})}

        <h2>"Converting between Types"</h2>
        // e. note: if branches return different types, you can convert between them with
        // .into_any() (for different html element types) or .into_view for all view types
        {move || match  is_odd() {
            true if value() == 1 => {
                // <pre> returns Htlmelement<pre>
                    view! { <pre>"One"</pre>}.into_any()
            },
            false if value() == 2 => {
                view! { <p>"Two"</p> }.into_any()
            }
            _ => view! {<textarea>{value()}</textarea>}.into_any()
        }}   
    }
}