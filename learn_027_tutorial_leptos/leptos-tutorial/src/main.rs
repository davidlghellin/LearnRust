use leptos::*;

fn main() {
    // mount_to_body(|| view! { <p>"Hello, world!"</p> })
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let html = "<p>This HTML will be injected.</p>";

    let values: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
    let values2: Vec<char> = vec!['a','b','c','d','e','f'];

    view! {
    <div class="container text-center">
        <div class="row align-items-center">
            <div class="col">
                <button type="button" class="btn btn-danger"
                    on:click=move |_| {set_count.update(|n| *n = 0);}>
                    "Reset"
                </button>
            </div>
            <div class="col">
                <button type="button" class="btn btn-success"
                    on:click=move |_| {set_count.update(|n| *n += 1);}>
                    "Success"
                </button>
            </div>
            <div class="col">
                <button on:click=move |_| {set_count.update(|n| *n += 1);}
                    class:red=move || count() % 2 == 1>
                    "Click me: "
                    // on stable, this is move || count.get();
                    {move || count()}
                </button>
            </div>
        </div>
        <div class="row align-items-center">
            <div class="col">
                <p>
                    <strong class:red=move || count() % 2 == 1>"Reactive: "</strong>
                    // you can insert Rust expressions as values in the DOM
                    // by wrapping them in curly braces
                    // if you pass in a function, it will reactively update
                    {move || count()}
                </p>
            </div>
            <div class="col">
                <p>
                    <strong>"Reactive shorthand: "</strong>
                    // signals are functions, so we can remove the wrapping closure
                    {count}
                </p>
            </div>
            <div class="col">
                <p>
                    <strong>"Not reactive: "</strong>
                    // NOTE: if you write {count()}, this will *not* be reactive
                    // it simply gets the value of count once
                    {count()}
                </p>
            </div>
        </div>
    </div>
    <div class="container text-center">
        <div class="row align-items-center">
            <button on:click={move |_| { set_count.update(|n| *n += 10); }}
                // set the `style` attribute
                style="position: absolute"
                // and toggle individual CSS properties with `style:`
                style:left=move || format!("{}px", count() + 100)
                style:background-color=move || format!("rgb({}, {}, 100)", count(), 100)
                style:max-width="400px"
                // Set a CSS variable for stylesheet use
                style=("--columns", count) type="button" class="btn btn-success"
            >
                "Click to Move"
            </button>
        </div>
    </div>
    <p></p>
    <div class="container text-center">
        <div class="row align-items-center">
            <div class="col">
                <button type="button" class="btn btn-success"
                    on:click=move |_| {set_count.update(|n| *n += 1);}>
                    "+1"
                </button>
            </div>
            <div class="col">
                <button type="button" class="btn btn-success"
                    on:click=move |_| {set_count.update(|n| *n += 2);}>
                    "+2"
                </button>
            </div>
            <div class="col">
                <button type="button" class="btn btn-success"
                    on:click=move |_| {set_count.update(|n| *n += 3);}>
                    "+3"
                </button>
            </div>
        </div>
        <div class="row align-items-center">
            <progress max="100"
            // signals are functions, so `value=count` and `value=move || count.get()`
            // are interchangeable.
            value=count />
        </div>
        <div class="row align-items-center">
            <ProgressBar progress=count/>
        </div>
        <div class="row align-items-center">
            <ProgressBar2 progress=count/>
        </div>
    </div>

    <div inner_html=html/>
    <p>{values.clone()}</p>
    // or we can wrap them in <li>
    <ul>
        {values.into_iter()
            .map(|n| view! { <li>{n}</li>})
            .collect::<Vec<_>>()}
    </ul>
    <div class="container text-center">
        <div class="row align-items-center">
            <table class="table">
                <thead>
                    <tr>
                        <th scope="col">n1</th>
                        <th scope="col">n2</th>
                    </tr>
                </thead>
                <tbody>
                {values2.into_iter()
                    .map(|n| view! {      
                        <tr>
                            <td>{n}</td>
                            <td>{n}</td>
                        </tr>}
                    )
                    .collect::<Vec<_>>()}
                </tbody>
            </table>
        </div>
    </div>

    }
}

#[component]
fn ProgressBar(
    // mark this prop optional
    // you can specify it or not when you use <ProgressBar/>
    #[prop(optional,default = 100)] 
    max: u16,
    progress: ReadSignal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

#[component]
fn ProgressBar2<F>(
    #[prop(default = 100)]
    max: u16,
    progress: F
) -> impl IntoView
where
    F: Fn() -> i32 + 'static,
// ProgressBar2<F: Fn() -> i32 + 'static>
{
    view! {
        <progress
            max=max
            value=progress
        />
    }
}
