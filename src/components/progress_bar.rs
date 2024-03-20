use leptos::{component, view, IntoView, Signal};

/// shows progress toward a goal.
#[component]
pub fn ProgressBar(
    #[prop(default = 100)]   // marked as optional prop
    max: i16,
    // Will run `.into()` on the value passed into the prop.
    #[prop(into)]
    // `Signal<T>` is a wrapper for several reactive types.
    // It can be helpful in component APIs like this, where we
    // might want to take any kind of reactive value
    /// How much progress should be displayed.
    progress: Signal<i32>) -> impl IntoView {

    view! {
        <progress max=max value=progress />
    }
}