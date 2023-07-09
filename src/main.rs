use leptos::ev::SubmitEvent;
use leptos::html::{Button, Input};
use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2>"Controlled Component"</h2>
        <ControlledComponent/>
        <h2>"Uncontrolled Component with form"</h2>
        <UncontrolledComponentWithForm/>
        <h2>"Uncontrolled Component without form"</h2>
        <UncontrolledComponentWithoutForm/>
    }
}

#[component]
fn ControlledComponent(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "".to_string());
    view! { cx,
        <p>"Typing some text on input element, automatically binds"</p>
        <p>"and name value will be updated continueously"</p>
        <input
            type="text"
            on:input=move |ev| { set_name(event_target_value(&ev)); }
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}
#[component]
fn UncontrolledComponentWithForm(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "".to_string());

    let input_element: NodeRef<Input> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element().expect("<input> to exist").value();
        set_name(value);
    };

    view! { cx,
        <p>"Press 'Enter Key' after typing some text on input element"</p>
        <p>"this will fires submit event of the form element"</p>
        <form on:submit=on_submit>
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}

#[component]
pub fn UncontrolledComponentWithoutForm(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "".to_string());

    let input_element = create_node_ref::<Input>(cx);
    let button_element = create_node_ref::<Button>(cx);

    let get_value = move |_| {
        let node = input_element
            .get()
            .expect("input_element should be loaded by now");
        set_name(node.value());
    };
    let disable_input = move |_| {
        let mut input_ref = input_element
            .get()
            .expect("input_element should be loaded by now");
        let mut button_ref = button_element
            .get()
            .expect("input_element should be loaded by now");
        if button_ref.inner_text() == "Disable Input" {
            input_ref = input_ref.attr("disabled", true);
            button_ref.set_inner_text("Enable Input");
        } else {
            input_ref = input_ref.attr("disabled", false);
            button_ref.set_inner_text("Disable Input");
        }
    };

    view! {
      cx,
      <p>"Press 'Enter Key' after typing some text on input element"</p>
      <p>"won't fires button click event, here we don't have any form element"</p>
      <p>"you have to manualy click on 'Click me' button to fire event"</p>
      <div>
        <input node_ref=input_element type="text" value=name/>
        <button on:click=get_value>"Get Value"</button>
        <button on:click=disable_input node_ref=button_element>"Disable Input"</button>
      </div>
      <div>
        <p>"Name is: " {name}</p>
      </div>
    }
}
