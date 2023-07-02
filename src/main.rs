use leptos::*;
use rand::Rng;
fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    let on_click_btn = move |_| {
        log!("count:{}", &count.get());
        set_count.update(|count: &mut i32| *count += 1);
    };

    view! {cx,
        <p>
            <input
            type="text"
            prop:value= move || count.get()
            disabled= move || { count.get() % 2 == 0}
            />
        </p>
        <p>
            <input
            type="checkbox"
            checked= move || { count.get() % 2 == 0}/> "Checkbox"
        </p>
        <p>
            <strong>"Reactive: "</strong>
            // you can insert Rust expressions as values in the DOM
            // by wrapping them in curly braces
            // if you pass in a function, it will reactively update
            {move || count.get()}
        </p>
        <p>
            <strong>"Reactive shorthand: "</strong>
            // signals are functions, so we can remove the wrapping closure
            {count}
        </p>
        <p>
            <strong>"Not reactive: "</strong>
            // NOTE: if you write {count()}, this will *not* be reactive
            // it simply gets the value of count once
            {count()}
        </p>
        <button on:click = on_click_btn>"ClickMe"</button>
    }
}
