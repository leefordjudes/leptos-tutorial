use leptos::*;
// Passing signal anywhere using provide_context & use_context
fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (toggled, set_toggled) = create_signal(cx, false);
    provide_context(cx, set_toggled);
    let color = move || {
        if toggled.get() {
            "background-color:red;padding:10px;margin:10px"
        } else {
            "background-color:green;padding:10px;margin:10px"
        }
    };
    view! { cx,
        <div style=color>
            <h4>"App"</h4>
            <p>"Toggled? " {toggled}</p>
            <Layout/>
        </div>
    }
}
#[component]
pub fn Layout(cx: Scope) -> impl IntoView {
    view! { cx,
        <div style="background-color:yellow;padding:10px;margin:10px">
            <h4>"Layout"</h4>
            <header>
                <h1>"My Page"</h1>
            </header>
            <main>
                <Content/>
            </main>
        </div>
    }
}

#[component]
pub fn Content(cx: Scope) -> impl IntoView {
    view! { cx,
        <div style="background-color:orange;padding:10px;margin:10px">
            <h4>"Content"</h4>
            <Section/>
        </div>
    }
}

#[component]
pub fn Section(cx: Scope) -> impl IntoView {
    let set_toggled =
        use_context::<WriteSignal<bool>>(cx).expect("to have found the setter provided");
    view! { cx,
        <div style="background-color:aqua;padding:10px;margin:10px">
            <h4>"Section"</h4>
            <p>"Pass write signal nestedly from App->Layout->Content->Section"</p>
            <button
                on:click=move |_| set_toggled.update(|value| *value = !*value)
            >
                "Toggle"
            </button>
        </div>
    }
}
