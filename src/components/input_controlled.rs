use leptos::{component, create_signal, event_target_value, view, IntoView, ReadSignal};

#[component]
pub fn ControlledComponent() -> impl IntoView {
    //craete a signal to hold the value
    let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <input type="text"
            // fire an event whenever the input changes
            // on:input = onChange, on:change = onBlur
            on:input=move |ev| {
                // event_target_value is a Leptos helper function
                // it functions the same way as event.target.value
                // in Javascript, but smooths out some of the typecasting
                // necessary to make this work in Rust
                set_name(event_target_value(&ev));
            }
            // the 'prop' syntax lets you update a Dom property, rather than an attribute.
            //
            // IMPORTANT: the value attribute only sets the initial value, until you have made
            // a change.  The value property sets the current value.  This sis a quirk of the DOM;
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
pub fn SelectList() -> impl IntoView {
    let (value, set_value) = create_signal("B".to_string());
    view! {
        <select on:change=move |ev| {
            let new_value = event_target_value(&ev);
            set_value(new_value);
        }>
            <SelectOption value is="A"/>
            <SelectOption value is="B"/>
            <SelectOption value is="C"/>
        </select>
    }
}

#[component]
pub fn SelectOption(is: &'static str, value: ReadSignal<String>) -> impl IntoView {
    view! {
        <option
            value=is
            selected=move || value() == is
        >
            {is}
        </option>
    }
}