use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // TakesChildrenFnOnce
    let msg = "try leptos".to_string();
    let children_fn_once = move || {
        view! {
            cx,
            <p>{msg}</p>
        }
        .into_view(cx)
    };
    // TakesChildrenFn
    let numbers = create_rw_signal(cx, vec![1, 2, 3]);
    let on_click = move |_| {
        let len = numbers.get_untracked().len();
        numbers.update(|x| x.push(len + 1));
    };
    let number_list = move || {
        numbers
            .get()
            .into_iter()
            .map(|x| view! {cx,<li>{x}</li>})
            .collect_view(cx)
    };
    // TakesChildrenFnMut
    let mut sum = 0;
    let mut total = move || {
        sum = numbers.with(|x| x.iter().fold(0, |s, y| s + y));
        view! {cx,<p>{sum}</p>}.into_view(cx)
    };
    let view_sum = move || sum;
    view! { cx,
        <TakesChildrenFnOnce>
        {children_fn_once()}
        </TakesChildrenFnOnce>
        <hr/>
        <button on:click=on_click>"Inc"</button>
        <TakesChildrenFn>
        <ul>{number_list}</ul>
        </TakesChildrenFn>
        <hr/>
        <TakesChildrenFnMut>
            {total()}
        </TakesChildrenFnMut>
        <WrapsChildren>
        <p>"Here's a child."</p>
        <p>"Here's another child."</p>
    </WrapsChildren>
    }
}

#[component]
pub fn TakesChildrenFnOnce(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <h2>"Takes Children FnOnce"</h2>
        {children(cx)}
    }
}
#[component]
pub fn TakesChildrenFn(cx: Scope, children: ChildrenFn) -> impl IntoView {
    view! { cx,
        <h2>"Takes Children Fn"</h2>
        <ul>
        {children(cx)}
        </ul>
    }
}
#[component]
pub fn TakesChildrenFnMut(cx: Scope, mut children: ChildrenFnMut) -> impl IntoView {
    view! { cx,
        <h2>"Takes Children FnMut"</h2>
        {children(cx)}
    }
}

/// Wraps each child in an `<li>` and embeds them in a `<ul>`.
#[component]
pub fn WrapsChildren(cx: Scope, children: Children) -> impl IntoView {
    // children(cx) returns a `Fragment`, which has a
    // `nodes` field that contains a Vec<View>
    // this means we can iterate over the children
    // to create something new!
    let children = children(cx)
        .nodes
        .into_iter()
        .map(|child| view! { cx, <li>{child}</li> })
        .collect::<Vec<_>>();

    view! { cx,
        <h1><code>"<WrapsChildren/>"</code></h1>
        // wrap our wrapped children in a UL
        <ul>{children}</ul>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
