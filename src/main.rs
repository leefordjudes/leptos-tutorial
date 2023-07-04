use leptos::*;
fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let voucher_types = vec![
        "Payment",
        "Receipt",
        "Cash withdrawal",
        "Cash deposit",
        "Journal",
    ];

    let counters_fn = move || {
        (1..=3)
            .map(|idx| create_signal(cx, idx))
            .collect::<Vec<_>>()
    };

    view! { cx,
        <h2>"Static views: Vec<_>"</h2>
        <h3>"Print without iterate"</h3>
        <p>{voucher_types.clone()}</p>
        <h3>"IterateWithVec"</h3>
        <IterateWithVec values=voucher_types.clone().into() />
        <h3>"IterateWithCollectView"</h3>
        <IterateWithCollectView values=voucher_types.clone().into() />
        <h3>"IterateWithSignals"</h3>
        <IterateWithSignals length=3/>
        <h3>"IterateWithFn"</h3>
        <IterateWithFn counters_fn=counters_fn/>
        <h2>"Dynamic views:"</h2>
        <h3>"IterateCountersUsingFor"</h3>
        <IterateCountersUsingFor length=3/>
    }
}

#[component]
fn IterateWithVec(cx: Scope, values: Vec<&'static str>) -> impl IntoView {
    view! {cx,
        <ul>
            {log!("iterate with vec");}
            {
                values
                .into_iter()
                .map(|v|view! {cx, <li>{v.to_owned()}</li>})
                .collect::<Vec<_>>()
            }
        </ul>
    }
}

#[component]
fn IterateWithCollectView(cx: Scope, values: Vec<&'static str>) -> impl IntoView {
    view! {cx,
        <ul>
            {log!("iterate with collect view");}
            {
                values
                .into_iter()
                .map(|v|view! {cx, <li>{v.to_owned()}</li>})
                .collect_view(cx)
            }
        </ul>
    }
}

#[component]
fn IterateWithSignals(cx: Scope, length: u8) -> impl IntoView {
    // create a list of N signals
    let counters = (1..=length).map(|idx| create_signal(cx, idx));

    // each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = counters
        .map(|(count, set_count)| {
            let update_count = move |_| set_count.update(|n| *n += 1);
            view! { cx,
                <li style="margin-bottom:5px">
                    {log!("iterate with signals");}
                    <button on:click=update_count>"Value: " {count} </button>
                </li>
            }
        })
        .collect_view(cx);

    view! { cx,
        <ul style="list-style-type:none;">{counter_buttons}</ul>
    }
}

#[component]
fn IterateWithFn<F, T1, T2>(cx: Scope, counters_fn: F) -> impl IntoView
where
    F: Fn() -> Vec<(T1, T2)> + 'static,
    T1: 'static,
    T2: 'static + leptos::SignalUpdate<u8>,
    (leptos::Scope, T1): leptos::IntoView,
{
    let counter_buttons = counters_fn()
        .into_iter()
        .map(|(count, set_count)| {
            let update_count = move |_| set_count.update(|n: &mut u8| *n += 1);
            view! { cx,
                <li style="margin-bottom:5px">
                    {log!("iterate with fn");}
                    <button on:click=update_count>"Value: " {count} </button>
                </li>
            }
        })
        .collect_view(cx);

    view! { cx,
        <ul style="list-style-type:none;">{counter_buttons}</ul>
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Counter {
    id: u8,
    count: RwSignal<u8>,
}
#[component]
fn IterateCountersUsingFor(cx: Scope, length: u8) -> impl IntoView {
    let counter_signals = (0..length)
        .map(|id| Counter {
            id,
            count: create_rw_signal(cx, id + 1),
        })
        .collect::<Vec<_>>();
    let (counters, _set_counters) = create_signal::<Vec<Counter>>(cx, counter_signals);

    log!("Dynamic views: IterateCountersUsingFor");

    view! {
      cx,
      <div>
      <ul style="list-style-type:none;">
        <For
          // a function that returns the items we're iterating over; a signal is fine
          each=move || counters.get()
          // a unique key for each item
          key=|counter| counter.id
          // renders each item to a view
          view=move |cx, counter: Counter| {
            view! {
              cx,
              <li style="margin-bottom:5px">
              {log!("iterate using for");}
            //   <button>"Value: " {move || counter.count.get()}</button>
                <button on:click=move |_| counter.count.update(|n| *n += 1)>"Value: " {counter.count} </button>
              </li>
            }
          }
        />
        </ul>
      </div>
    }
}
