use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        {control_flow(cx)}
        <h2>"OverRendering"</h2>
        <OverRendering/>
        <h2>"RenderWithShow"</h2>
        <RenderWithShow/>
        <h2>"TypeConversion"</h2>
        <TypeConversion/>
    }
}

fn control_flow(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, 0);
    let is_odd = move || value() & 1 == 1;
    let odd_text = move || if is_odd() { Some("How odd!") } else { None };
    let upd_fn = move |_| set_value.update(|n| *n += 1);
    view! { cx,
        <h2>"ControlFlow"</h2>
        <button on:click=upd_fn>"+1"</button>
        <p>"Value is: " {value}</p>
        <hr/>
        <h2>"Option<T>"</h2>
        <p>{odd_text}</p>
        <p>{move || odd_text().map(|text| text.len())}</p>
        <h2>"Conditional Logic"</h2>
        <p>
            {move || if is_odd() {
                "Odd"
            } else {
                "Even"
            }}
        </p>
        <p class:hidden=is_odd>"Appears if even."</p>
    }
}

#[component]
fn OverRendering(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, 0);
    let message = move || {
        if value() > 5 {
            log!("{}: over rendering Big", value());
            // "Big"
            view! {cx, <Big/>}
        } else {
            log!("{}: over rendering Small", value());
            // "Small"
            view! {cx, <Small/>}
        }
    };

    view! { cx,
        <button on:click=move |_| set_value.update(|n| *n += 1)>"Click Me"</button>
        // <p>{message}</p>
        {message}
    }
}

#[component]
fn RenderWithShow(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, 0);
    create_effect(cx, move |_| log!("value: {:?}", value()));
    view! { cx,
        <button on:click=move |_| set_value.update(|n| *n = *n + 1)>"Click Me"</button>
        <Show
            when=move || {value() > 5 }
            fallback=|cx| {view! { cx, <Small/> }}
        >
            <Big/>
        </Show>
    }
}

#[component]
fn Big(cx: Scope) -> impl IntoView {
    log!("Component Big rendered");
    view! {cx,
        <p>"Big"</p>
    }
}
#[component]
fn Small(cx: Scope) -> impl IntoView {
    log!("Component Small rendered");
    view! {cx,
        <p>"Small"</p>
    }
}

#[component]
fn TypeConversion(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, 0);
    let is_odd = move || value() & 1 == 1;
    create_effect(cx, move |_| log!("value: {:?}", value()));
    view! { cx,
        <button on:click=move |_| set_value.update(|n| *n = *n + 1)>"Click Me"</button>
        <main>
            {move || match is_odd() {
                true if value() == 1 => {
                    // returns HtmlElement<Pre>
                    view! { cx, <pre>"One"</pre> }.into_any()   //here & below lines - without .into_any() it shows error
                },
                false if value() == 2 => {
                    // returns HtmlElement<P>
                    view! { cx, <p>"Two"</p> }.into_any()
                }
                // returns HtmlElement<Textarea>
                _ => view! { cx, <textarea>{value()}</textarea> }.into_any()
            }}
        </main>
    }
}
