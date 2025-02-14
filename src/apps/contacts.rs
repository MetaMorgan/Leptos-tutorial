use leptos::*;
use leptos_router::*;

#[component]
pub fn ContactApp() -> impl IntoView {
    view! {
        <Router>
            <h1>"Contact App"</h1>
            // this <nav> will show on every routes, because its outside the <Routes/>
            // note: we can just use normal <a> tags and the router will use client-side navigation
            <nav>
                <a href="/">"Home"</a>
                <a href="/contacts">"Contacts"</a>
            </nav>
            <main>
                <Routes>
                    // / just has an un-nested "home"
                    <Route path="/" view=|| view! {<h3>"Home"</h3>}/>
                    // contact has nested routes
                    <Route path="/contacts" view=ContactList>
                        // if no id specified, fall back
                        <Route path=":id" view=ContactInfo>
                            <Route path="" view=|| view! {<div class="tab">"(Contact Info)"</div>} />
                            <Route path="conversations" view=|| view! {<div class="tab">"(Conversations)"</div>} />
                        </Route>
                        <Route path="" view=|| view! {<div class="select-user">"Select a user to view contact info."</div>} />
                    </Route>                    
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn ContactList() -> impl IntoView {
    view! {
        <div class="contact-list">
            // here ours contact list component itseld
            <h3>"Contacts"</h3>
            <div class="contact-list-contacts">
                <A href="alice">"Alice"</A>
                <A href="bob">"Bob"</A>
                <A href="steve">"Steve"</A>
            </div>

            // <Outlet/> will show the nested child route we can position this
            // outlet wherever we want within the layout
            <Outlet/>
        </div>
    }
}

#[component]
pub fn ContactInfo() -> impl IntoView {
    // we can access the :id param reactively with use_params_map
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    // imagine we're loading data from the API here
    let name = move || match id().as_str() {
        "alice" => "Alice",
        "bob" => "Bob",
        "steve" => "Steve",
        _ => "User not found."
    };

    view! {
        <h4>{name}</h4>
        <div class="contact-info">
            <div class="tabs">
                <A href="" exact=true>"Contact Info"</A>
                <A href="conversations">"Conversations"</A>
            </div>

            // <outlet> here is the tabs that are neste underneath the /contacts/:id route
            <Outlet/>
        </div>
    }
}