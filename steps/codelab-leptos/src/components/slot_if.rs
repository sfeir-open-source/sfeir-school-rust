use leptos::prelude::*;

// this example come from leptos documentation: https://github.com/leptos-rs/leptos/blob/main/examples/slots

#[slot]
struct Then {
    children: ChildrenFn,
}

#[slot]
struct ElseIf {
    cond: Signal<bool>,
    children: ChildrenFn,
}

#[slot]
struct Fallback {
    children: ChildrenFn,
}

// Slots are added to components like any other prop.
#[component]
fn SlotIf(
    cond: Signal<bool>,
    then: Then,
    #[prop(default=vec![])] else_if: Vec<ElseIf>,
    #[prop(optional)] fallback: Option<Fallback>,
) -> impl IntoView {
    move || {
        if cond.get() {
            (then.children)().into_any()
        } else if let Some(else_if) = else_if.iter().find(|i| i.cond.get()) {
            (else_if.children)().into_any()
        } else if let Some(fallback) = &fallback {
            (fallback.children)().into_any()
        } else {
            ().into_any()
        }
    }
}