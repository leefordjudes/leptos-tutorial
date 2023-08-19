use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let rendered_property_fn = || view! { cx, <p>"para that rendered as property"</p> };
    view! { cx,
        <TakesChildren render_prop=rendered_property_fn >
            // these get passed to `children`
            "child1: some text"
            <p>"child2: some paragraph"</p>
            <br/>
            <span>"child3: some span"</span>
        </TakesChildren>
    }
}

#[component]
pub fn TakesChildren<F, IV>(
    cx: Scope,
    /// Takes a function (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_prop: F,
    /// `children` takes the `Children` type
    children: Children,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! { cx,
        <h2>"Render Prop"</h2>
        {render_prop()}

        <h2>"Children"</h2>
        {children(cx)}
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
