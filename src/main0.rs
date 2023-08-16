use leptos::ev::MouseEvent;
use leptos::*;
// Parent child communication using signal
fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {cx,
        <ParentToChildPassWriteSignal/>
        <ParentToChildViaCallback/>
        <ParentToChildViaEventListener/>
    }
}

#[component]
pub fn ParentToChildPassWriteSignal(cx: Scope) -> impl IntoView {
    let (toggled, set_toggled) = create_signal(cx, false);
    view! { cx,
        <p>"PassWriteSignal Toggled? " {toggled}</p>
        <ButtonA setter=set_toggled/>
    }
}

#[component]
pub fn ButtonA(cx: Scope, setter: WriteSignal<bool>) -> impl IntoView {
    view! { cx,
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "PassWriteSignal Toggle"
        </button>
    }
}

#[component]
pub fn ParentToChildViaCallback(cx: Scope) -> impl IntoView {
    let (toggled, set_toggled) = create_signal(cx, false);
    let callback_fn = move |_: MouseEvent| set_toggled.update(|value| *value = !*value);
    view! { cx,
        <p>"Callback Toggled? " {toggled}</p>
        <ButtonB on_click=callback_fn/>
    }
}

#[component]
pub fn ButtonB<F>(cx: Scope, on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! { cx,
        <button on:click=on_click>
            "Callback Toggle"
        </button>
    }
}

#[component]
pub fn ParentToChildViaEventListener(cx: Scope) -> impl IntoView {
    let (toggled, set_toggled) = create_signal(cx, false);
    let click_event = move |_: MouseEvent| set_toggled.update(|value| *value = !*value);
    view! { cx,
        <p>"Toggled? " {toggled}</p>
        // note the on:click instead of on_click
        // this is the same syntax as an HTML element event listener
        <ButtonC on:click=click_event/>
    }
}

#[component]
pub fn ButtonC(cx: Scope) -> impl IntoView {
    view! { cx,
        <button>"Toggle"</button>
    }
}
