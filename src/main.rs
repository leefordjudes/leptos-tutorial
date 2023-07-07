use leptos::{ev::Event, *};
use nanoid::nanoid;
use rand::Rng;

static ALPHABET: [char; 36] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];
static HEX: [char; 16] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
];
static CHAR_NUMBER: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
static HEX_ALPHABET: [char; 6] = ['a', 'b', 'c', 'd', 'e', 'f'];

fn get_refno() -> String {
    let refno = nanoid!(5, &ALPHABET);
    log!("refno: {:?}", refno);
    refno
}
fn get_oid() -> String {
    let oid_num = nanoid!(2, &CHAR_NUMBER);
    let oid_hex = nanoid!(14, &HEX);
    let oid = format!("{}{}", oid_num, oid_hex);
    log!("oid: {:?}", oid);
    oid
}
fn get_closing() -> f64 {
    let closing = rand::thread_rng().gen_range(100..1000);
    log!("closing: {:?}", closing as f64);
    closing as f64
}

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
        <h3>"IterateDynamicCountersUsingFor"</h3>
        <IterateDynamicCountersUsingFor initial_length=3/>
        <h3>"Array of records"</h3>
        <IterateDynamicItemsUsingFor initial_length=5/>
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
#[component]
fn IterateDynamicCountersUsingFor(cx: Scope, initial_length: u8) -> impl IntoView {
    let mut next_counter_id = initial_length;

    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(cx, id + 1)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = create_signal(cx, initial_counters);

    let add_counter = move |_| {
        let sig = create_signal(cx, next_counter_id + 1);
        set_counters.update(move |counters| counters.push((next_counter_id, sig)));
        next_counter_id += 1;
    };

    log!("Dynamic views: IterateDynamicCountersUsingFor");

    view! {
      cx,
      <div>
      <button on:click=add_counter>"Add Counter"</button>
      <ul style="list-style-type:none;">
        <For
          each=counters
          key=|counter| counter.0
          view=move |cx, (id, (count, set_count))| {
            let update_count = move |_| set_count.update(|n| *n += 1);
            let update_counters = move |_| {
                set_counters.update(|counters| {
                    counters.retain(|(counter_id, _)| counter_id != &id)
                });
            };
            view! {
              cx,
              <li style="margin-bottom:5px">
                <button on:click=update_count>{count}</button>
                <button on:click=update_counters>"Remove"</button>
              </li>
            }
          }
        />
        </ul>
      </div>
    }
}

#[derive(Clone, Debug)]
struct Item {
    pending: String,
    refno: String,
    closing: f64,
    amount: f64,
}

impl Item {
    fn default() -> Self {
        Self {
            pending: get_oid(),
            refno: get_refno(),
            closing: get_closing(),
            amount: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
struct GroupItem {
    pending: String,
    item: RwSignal<Item>,
}
#[component]
fn IterateDynamicItemsUsingFor(cx: Scope, initial_length: u8) -> impl IntoView {
    let default_items = (0..initial_length)
        .map(|_| Item::default())
        .collect::<Vec<_>>();

    let initial_items = default_items
        .into_iter()
        .map(|item| (item.pending.clone(), create_signal(cx, item)))
        .collect::<Vec<_>>();

    let (items, set_items) = create_signal(cx, initial_items);

    // let add_item = move |_| {
    //     let i = Item::default();
    //     let sig = create_signal(cx, i.clone());
    //     set_items.update(move |items| items.push((i.pending, sig)));
    // };

    let add_item = move |_| {};

    log!("Dynamic views: IterateDynamicItemsUsingFor");

    let set_amount = move |pending, value| {
        log!("pending: {:?}, value: {:?}", pending, value);
        // let items = items.get();
        // if let Some(itm) = items.into_iter().find(|x| x.0 == pending) {
        //     let (get, set) = itm.1;
        //     let v = value.parse::<f64>().unwrap_or_default();
        //     set.update(|i| *i.amount = v);
        // }
    };

    view! {
      cx,
      <div>
      <p>"item list"</p>
      <button on:click=add_item>"Add Item"</button>
      <table width="30%">
          <tr>
              <th width="40%">"pending"</th>
              <th width="20%">"refno"</th>
              <th width="20%">"closing"</th>
              <th width="20%">"amount"</th>
          </tr>

    //   {
    //     default_items
    //       .into_iter()
    //       .map(|v|{
    //           view! {cx,
    //           <tr>
    //               <td width="40%">{v.pending.to_owned()}</td>
    //               <td width="20%">{v.refno.to_owned()}</td>
    //               <td width="20%">{v.closing.to_owned()}</td>
    //               <td width="20%"><input type="number" prop:value={v.amount.to_owned()}/></td>
    //           </tr>
    //           }
    //       })
    //       .collect::<Vec<_>>()
    //   }
        <For
          each=items
          key=|item| item.0.clone()
          view=move |cx, (id, (item, set_item))| {
            let itm = item.get();
            view! {
              cx,
              <tr>
                <td width="40%">{itm.pending.clone()}</td>
                <td width="20%">{itm.refno.clone()}</td>
                <td width="20%">{itm.closing}</td>
                <td width="20%"><input type="number" prop:value={itm.amount} on:input=move |ev| {
                    let id=itm.pending.clone();
                    set_amount(id, event_target_value(&ev));
                }/></td>
              </tr>
            }
          }
        />

      </table>

    //   <table width="30%">
    //       <tr>
    //           <th width="40%">"pending"</th>
    //           <th width="20%">"refno"</th>
    //           <th width="20%">"closing"</th>
    //           <th width="20%">"amount"</th>
    //       </tr>
    //   <For
    //     each=move || items.get()
    //     key=|item| item.0
    //     view=move |cx, (id, (item, set_item))| {
    //         let itm = item.get();
    //       view! {
    //         cx,
    //         <tr>
    //             <td width="40%">{itm.pending.clone()}</td>
    //             <td width="20%">{itm.refno.clone()}</td>
    //             <td width="20%">{itm.closing}</td>
    //             <td width="20%">{itm.amount}</td>
    //         </tr>
    //       }
    //     }
    //   />
    //   </table>


      </div>
    }
}
