// this highlights four different ways child components can communicate with theor parrents:
// 1) <button/>: passing a WriteSingal as one of the childe component props, for the child 
//      component to write into the parent and read
// 2) <buttonB/>: passing a closure as one of the childe component props, for the child component to call
// 3) <buttonC/>: adding as on: event listener to a component
// 4) <buttonD/>: providing a contect that is used in the component (rather than prop drilling)

use leptos::{component, create_signal, ev::MouseEvent, provide_context, use_context, view, Children, IntoView, SignalUpdate, SignalWith, WriteSignal};

#[derive(Copy, Clone)]
struct SmallcapContext(WriteSignal<bool>);

#[component]
pub fn ParentChild() -> impl IntoView {
    // just set some signals to toggle three classes on our <p>
    let (red, set_red) = create_signal(false);
    let (right, set_right) = create_signal(false);
    let (italics, set_italics) = create_signal(false);
    let (smallcaps, set_smalcaps) = create_signal(false);

    // the newtype pattern isnt neccasry here but is a good practice
    // it avoids confusion with other possible future WriteSignal<bool> contexts
    provide_context(SmallcapContext(set_smalcaps));
    
    

    view! {
        <main>
            <p
                // class: attributes take F: fn() => bool, and these signals all implemnt Fn()
                class:red=red
                class:right=right
                class:italics=italics
                class:smallcaps=smallcaps
            >
                "Lorem ipsum sit dolor amet."
            </p>

            // button A: pass the signal setter
            <ButtonA setter=set_red/>

            // button B: pass a closure
            <ButtonB on_click=move |_| set_right.update(|value| *value = !*value)/>

            // button B: uses a regular event listener
            // setting an event listener on a component like bellow applies it
            // to each of the top-level elements the component returs
            // Basically, we are not passing on_click but handeling it here as a html element
            <ButtonC on:click=move |_| set_italics.update(|value| *value = !*value)/>

            // button D gets its setter from context rather that props
            <ButtonD/>
        </main>
    }
}

/// Button A receives a signal setter and updates the signal itself
#[component]
pub fn ButtonA(
    /// Signal that will be toggled when the button is clicked.
    setter: WriteSignal<bool>
) -> impl IntoView {
    view! {
        <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle Red"</button>
    }
}

/// Button B recieves a closure
#[component]
pub fn ButtonB<F>(
    /// callback that will be invoked when the button is clicked.
    on_click: F
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static
{
    view! { 
        <button on:click=on_click>"Toggle Right"</button>
    }

    // just a note: in an ordinary function ButtonB could take on_click: impl Fn(MouseEvent) + 'static
    // and save you from typing out the generic the component macor acctualy expands to define a
    //
    // struct ButtonBProps<F> where F: Fm(MouseEvent) + 'static { on_click:F }
    //
    // this is what allows us to have named props in our component invocation,
    // instead of an ordered list of function arguments
    // If Rust ever had named function arguments we could drop the requirement
}

/// button C is a dummy:  it renders a button but doesn't handle
/// its click.  Instead, the parent component adds and event listener.
#[component]
pub fn ButtonC() -> impl IntoView {
    view! { <button>"Toggle Italics"</button>}
}

/// Button D is very similar to Button A, but instead of passing the setter as a prop we get it from context
#[component]
pub fn ButtonD() -> impl IntoView {
    let setter = use_context::<SmallcapContext>().unwrap().0;

    view! {
        <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle Small Cap"</button>
    }
}

#[component]
pub fn PassChildren() -> impl IntoView {
    let (items, set_items) = create_signal(vec![0, 1, 2]);
    let render_prop = move || {
        // items.with(...) reacts to the value without cloning
        // by applying a function.  Here, we pass the len method
        // on a Vec<_> direclty
        let len = move || items.with(Vec::len);
        view! { <p>"Length: " {len}</p> }
    };

    view! {
        // this component just displays the two kinds of children,
        // embedding them in some other markup
        <TakesChildren
            // for component props, you can shorthand 
            // render_prop=render_prop => render_prop
            // (this doesn't work for HTML element attributes)
            render_prop
        >

            // these look just like the children of an HTML element
            <p>"Here's a child."</p>
            <p>"Here's another child"</p>
        </TakesChildren>
        <hr/>
        // this component actually iterates over and wraps the childer
        <WrapsChildren>
            <p>"Here's a child."</p>
            <p>"Here's another child."</p>
        </WrapsChildren>
    }
}

/// Displays a 'render_prop' and some children within markup
#[component]
pub fn TakesChildren<F, IV>(
    /// Takes a funciton (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_prop: F,
    /// children takes the Children type
    /// this is an alias for Box<dyn FnOnce() -> Fragment>
    children: Children
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView
{
    view! {
        <h1><code>"<TakesChildren/>"</code></h1>
        <h2>"Render Prop"</h2>
        {render_prop()}
        <hr />
        <h2>"Childred"</h2>
        {children()}
    }
}

/// Wraps each child in an <li> and embeds them in a <ul>
#[component]
pub fn WrapsChildren(children: Children) -> impl IntoView {
    // children() returns a Fragment, which has a nodes field that 
    // contains Vec<View> this means we can iterate over the chidren to creat something new!
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect::<Vec<_>>();

    view! {
        <h1><code>"<WrapsChildren/>"</code></h1>
        // wrap our wrapped children in ul
        <ul>{children}</ul>
    }
}