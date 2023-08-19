use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let items = create_rw_signal(cx, Vec::new());
    items.set(vec![1.to_string(), 2.to_string()]);
    // let on_click = move |_| {
    //     let nos = items.get().len() as u32;
    //     items.update(|x| x.push(nos.to_string()));
    // };
    let ul_list = move || {
        items
            .get()
            .into_iter()
            .map(|x| view! {cx, <li>{x}</li>})
            .collect_view(cx);
    };
    let list = move || {
        view! {
            cx,
            // <button on:click=on_click >"INC"</button>
            <ul>{ul_list}</ul>
        }
    };
    view! { cx,
        <TakesChildren>
            <ul>{ul_list}</ul>
        </TakesChildren>
    }
}

#[component]
pub fn TakesChildren(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <h2>"Children"</h2>
        {children(cx)}
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
