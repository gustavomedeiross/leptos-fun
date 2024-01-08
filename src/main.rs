use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: RwSignal<i32>,
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: create_rw_signal(10),
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: create_rw_signal(20),
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: create_rw_signal(15),
        },
    ]);

    view! {
        <button on:click=move |_| {
            set_data
                .update(|data| {
                    for row in data {
                        row.value.update(|value| *value *= 2);
                    }
                });
            logging::log!("{:?}", data.get());
        }>"Update Values"</button>
        <For each=data key=|state| state.key.clone() let:child>
            <p>{child.value}</p>
        </For>

        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }

            class:red=move || count() % 2 == 1
        >

            "Click me: "
            {count}
        </button>
        <br/>
        <ProgressBar progress=count/>
        <br/>
        <ProgressBar progress=Signal::derive(double_count)/>
        <p>"Double Count: " {double_count}</p>
    }
}


#[component]
fn ProgressBar(
    #[prop(default = 100)]
    max: u16,
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView {
    view! { <progress max=max value=progress></progress> }
}
