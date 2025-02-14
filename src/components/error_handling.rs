use leptos::{component, create_signal, event_target_value, view, ErrorBoundary, IntoView, SignalGet};

#[component]
pub fn ErrorHandling() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    // when input changes, tyr to parse a neumber from the input
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input type="number" on:input=on_input />
            // if and Err(_) has been rendered inside the <ErrorBoundary/>, the fallback will be displayed
            // Otherwise, the childer of the <ErrorBoundary/> will be displayed.
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors "</p>
                        // We can render a list of errors as strings if we'd like
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect::<Vec<_>>()
                            }
                        </ul>
                    </div>
                }
            >
                <p>
                    "You entered "
                    // because value is Result<i32, _>, it will render the i32 if it is OF,
                    // and render nothing and trigger the error boundary if it is Err.  It's a signal,
                    // so this will dynamically update when value changes
                    <strong>{value}</strong>
                </p>
            </ErrorBoundary>
        </label>
    }
}