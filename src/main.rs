use leptos::*;
use rand::Rng;
fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let (x, set_x) = create_signal(cx, 0);
    let (y, set_y) = create_signal(cx, 0);
    let double_count = move || count() * 2;

    let mut rng = rand::thread_rng();
    let update_pos_count = move |_| {
        set_count.update(|n| *n += 1);
        set_x.update(|v| *v = rng.gen::<u8>());
        set_y.update(|v| *v = rng.gen::<u8>());
    };

    view! { cx,
        <button
            on:click=update_pos_count
            // class:red=move || count() % 2 == 1
            class=("red", move || count() % 2 == 1)
        >
            "Click me: "
            {move || count}
            " - "
            {move || x}
            " - "
            {move || y}
        </button><br/>
        <progress max="100" value=count />"  "{move || if count.get() >= 100 {100}else{count.get()}}" %"<br/>
        <progress max="100" value=double_count />"  "{move || if double_count() >= 100 {100} else {double_count()}}" %" <br/>

        <div
            style="position: absolute"
            style:left=move || format!("{}px", x() + 10)
            style:top=move || format!("{}px", y() + 100)
            style:background-color=move || format!("rgb({:?}, {:?}, 100)", {x()}, {y()})
        >
            "Moves when coordinates change"
        </div>
    }
}
