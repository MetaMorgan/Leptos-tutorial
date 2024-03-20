use std::marker::PhantomData;

use leptos::{create_signal, view, IntoView, Signal, SignalUpdate};
use crate::{component, components::progress_bar::ProgressBar};

/// the *[component] macro marks a function as reusable component
/// Components are the building blocks of you user interface
/// they define a reusable unit of behavior
#[component]
pub fn App() -> impl IntoView {
    // here we create a reactive signal and get a (getter, setter) pair
    // signals are the basic unit of change in the framework
    // we'll talk more about them later
    let (count, set_count) = create_signal(0);
    //let (double_count, set_double_count) = create_signal(0);

    // a "derived signal" is a function that accesses other signals
    // we can use this to create reactive values that depend on the
    // values of one or more other signals
    let double_count = move || count() * 2;

    // the 'view' marco is how we define the user interface
    // it uses an html-like format that can accept certain rust values
    view! {
        <button
            // on:click will run whenever the 'click' event fires
            // every event handler is defined as on:{eventname}
            // 
            // were able to move set_count into the closure
            // because signals are Copy and 'static

            //class:red=move || count() % 2 == 1
            // set the `style` attribute
            // style="position: absolute"
            // and toggle individual CSS properties with `style:`
            //style:left=move || format!("{}px", count() + 100)
            //style:background-color=move || format!("rgb({}, {}, 100)", count(), 100)
            //style:max-width="400px"
            // Set a CSS variable for stylesheet use
            //style=("--columns", count)

            on:click=move |_| {
                set_count.update(|n| *n += 2);
                // set_double_count.update(|d| *d = count() * 2);
            }
        >
            // text nodes in RSX should be wrapped in quotes,
            // like a normal rust string
            "Click me: "

            // NOTE: self-closing tags like <br> need an explicit /
            <br/>

            // We'll update this progress bar every time `count` changes
            <ProgressBar progress=count max=10 />
            <br/>

            // This progress bar will use `double_count`
            // so it should move twice as fast!
            <ProgressBar progress=Signal::derive(double_count) max=20 />

            <p><SizeOf<usize>/></p>
            <p><SizeOf<String>/></p>

            // on stable, this is move || count.get();
            // {move || count()}
            // else short hand
            <p>"Count: " {count}</p>
            <p>"Double Count: " {double_count}</p>
        </button>
    }
}


#[component]
fn SizeOf<T: Sized>(#[prop(optional)] _ty: PhantomData<T>) -> impl IntoView {
    std::mem::size_of::<T>()
}