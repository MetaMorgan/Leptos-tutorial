use leptos::*;
mod components;

fn main() {
    let values = vec![0, 1, 2];
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { 
        <>
        <components::app::App />
        <components::app::App /> 
        // this will just render 012
        <p>{values.clone()}</p>
        // or we can wrap them in <li>
        <ul>
            {values.into_iter()
                .map(|n| view! {
                    <li>{n}</li>
                })
                .collect_view()}
        </ul>

        <div style="float:left"><components::static_list::StaticList length=10 /></div>
        <div style="float:left"><components::dynamic_list::DynamicList initial_length=10 /></div>
        <div style="clear:both" />
        <ForExample />

        <div style="float:left"><components::input_controlled::ControlledComponent /></div>
        <div style="float:left"><components::input_uncontrolled::UncontrolledComponent /></div>
        <div style="clear:both" />

        <div style="float:left"><components::control_flow::ControlFlow /></div>
        <div style="float:left"><components::error_handling::ErrorHandling /></div>
        <div style="clear:both" />
        
        <h1>"PARENT->CHILD"</h1>
        <div style="float:left"><components::parent_child::ParentChild /></div>
        <div style="float:left"><components::parent_child::PassChildren /></div>
        <div style="clear:both" />

        <h1>"EFFECTS"</h1>
        <components::effects::Effects />

        <h1>"ASYNC COMPONENT"</h1>
        <div style="float:left"><components::async_component::AsyncComponent /></div>
        <div style="float:left"><components::suspense_component::SuspenseComponent /></div>
        </>
        
    })
}


#[component]
fn ForExample() -> impl IntoView {
    #[derive(Debug, Clone)]
    struct DatabaseEntry {
        key: String,
        value: i32
    }

    let (data, set_data) = create_signal(vec![
        DatabaseEntry { key: "foo".to_string(), value: 10 },
        DatabaseEntry { key: "bar".to_string(), value: 20 },
        DatabaseEntry { key: "baz".to_string(), value: 15 }
    ]);

    view! {
        // when we click, update each row,
        // doubling its value
        <button on:click=move |_| {
            set_data.update(|data| {
                for row in data {
                    // every time data changes each memo will be recalulated. If its
                    // value has changed, it will update its text node, withoug rerendering the whole row
                    row.value *= 2;
                }
            });
            // log the new value of the signal
            logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        // iterate over the rows and display each value
        <For
            // convert the data signal into an enumerated iterator
            each=move || data().into_iter().enumerate()
            key=|(_, state)| state.key.clone()
            
            // use the children prop explicitly, to make it easier to run some non-view code
            children=move |(index, _)| {
                // define a value memo and use that in the view.  This value field doesn't actually
                // use the child being passed into each row. instead it uses the index and reaches
                // into the original data to get the value;
                let value = create_memo(move |_| {
                    data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
                });
                view! {
                    <p>{value}</p>
                }
            }
        />
        
    }
}