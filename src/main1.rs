use leptos::*;
// Passing signal deep & nested
fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (toggled, set_toggled) = create_signal(cx, false);
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
            <Layout set_toggled/>
        </div>
    }
}
#[component]
pub fn Layout(cx: Scope, set_toggled: WriteSignal<bool>) -> impl IntoView {
    view! { cx,
        <div style="background-color:yellow;padding:10px;margin:10px">
            <h4>"Layout"</h4>
            <header>
                <h1>"My Page"</h1>
            </header>
            <main>
                <Content set_toggled/>
            </main>
        </div>
    }
}

#[component]
pub fn Content(cx: Scope, set_toggled: WriteSignal<bool>) -> impl IntoView {
    view! { cx,
        <div style="background-color:orange;padding:10px;margin:10px">
            <h4>"Content"</h4>
            <Section set_toggled/>
        </div>
    }
}

#[component]
pub fn Section(cx: Scope, set_toggled: WriteSignal<bool>) -> impl IntoView {
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
