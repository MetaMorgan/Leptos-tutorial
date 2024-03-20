use leptos::{component, create_signal, view, For, IntoView, SignalDispose, SignalUpdate};

/// A list of counter that allow you to add or remove counters.
#[component]
pub fn DynamicList(
    /// the number of counters to begin with.
    initial_length: usize
) -> impl IntoView {
    // this dynamic list will use <For/> component. <For/> is a keyed list. This means
    // that each row has a defined key. If the key does not change, the row will not
    // be re-rendered.  When the list changes, only the minimum number of changes will
    // be made to the DOM.

    // next_counter_id will let us generate unique Ids
    // we do this by simply incrementing the ID by one
    // each time we create a counter
    let mut next_counter_id = initial_length;

    // we generate an initial list as in <StaticList/>
    // but this time we include the Id along with the signal
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();

    // now we store initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        // create a signal for the new counter
        let sig = create_signal(next_counter_id + 1);
        // add this counter to the list of counters
        set_counters.update(move |counters| {
            // since .update gives us &mut T we can just use normal Vec methods like push
            counters.push((next_counter_id, sig))
        });
        // increament the Id so its always unique
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>"Add Counter"</button>
            <ul>
                // the <For/> component is central here
                // this allows for efficient, key list rendering
                <For
                    // each takes any function that returns an iterator
                    // this should usually be a signal or derived signal
                    // it its not reactive, just render a Vec<_> instead for <For/>
                    each=counters
                    // the key should be unique and stable for each row
                    // using as index is usually a bad idea, unless your list
                    // can only grow, because moving items around inside the list
                    // means their indices will change and they will all rerender
                    key=|counter| counter.0
                    // children receives each item for your each iterator and returns a view
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button on:click=move |_| set_count.update(|n| *n += 1)>{count}</button>
                                <button on:click=move |_| {
                                    set_counters.update(|counters| {
                                        counters.retain(|(counter_id, (signal, _))| {
                                            // NOTE: in this example, we are creating the signals
                                            // in the scope of the parent. This means the memory used to
                                            // store them will not be reclaimed until the parent component
                                            // is unmounted. Here, we're removing the signal early (i.e, before
                                            // the DynamicList is unmounted), so we manually dispose of the signal
                                            // to avoid leaking memory.
                                            //
                                            // This is only necessary in an example with nested signals like this one.
                                            if counter_id == &id {
                                                signal.dispose();
                                            }
                                            counter_id != &id
                                        })
                                    });
                                }>"Remove"</button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}