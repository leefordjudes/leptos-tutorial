use leptos::*;
fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let double_count = move || count() * 2;

    let update_count = move |_| {
        set_count.update(|n| *n += 1);
    };

    view! { cx,
        <button on:click=update_count>"Click me: "</button><br/>
        <ProgressBarWithProps min=50 progress=count /><br/>
        <ProgressBarWithGeneric progress=double_count /><br/>
    }
}

#[component]
fn ProgressBarWithProps(
    cx: Scope,
    #[prop(optional)] min: u16,
    #[prop(default = 100)] max: u16,
    progress: ReadSignal<i32>,
) -> impl IntoView {
    let value = move || (min as i32) + progress.get();
    view! {cx,
        <progress min=min max=max value=value/>
    }
}
#[component]
fn ProgressBarWithGeneric<F>(
    cx: Scope,
    #[prop(optional)] min: u16,
    #[prop(default = 100)] max: u16,
    progress: F,
) -> impl IntoView
where
    F: Fn() -> i32 + 'static,
{
    view! {cx,
        <progress min=min max=max value=progress/>
    }
}

// #[component]
// fn ProgressBarWithIntoProps(
//     cx: Scope,
//     #[prop(optional)] min: u16,
//     #[prop(default = 100)] max: u16,
//     progress: F,
// ) -> impl IntoView
// where
//     F: Fn() -> i32 + 'static,
// {
//     view! {cx,
//         <progress min=min max=max value=progress/>
//     }
// }
