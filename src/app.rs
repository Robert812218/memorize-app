use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod linux;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

	<Meta name="color-scheme" content="dark"/>
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    	<Route path="" view=|cx| view! { cx, <HomePage/> }/>
		    	        <Route path="/test" view=|cx| view! { cx, <TestPage/> }/>
			            <Route path="/linux" view=|cx| view! { cx, <LinuxPage /> }/>
			            <Route path="/vim" view=|cx| view! { cx, <VimPage/> }/>
                </Routes>
            </main>
        </Router>
    }
}



/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"New Game"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
pub fn LinuxPage(cx: Scope) -> impl IntoView {
	view! {
		cx,
		<div>
			// linux::Linux();

		</div>
	}
}




#[component]
pub fn VimPage(cx: Scope) -> impl IntoView {
	view! {cx,
		<div>
			<h1>"Vim Stuff"</h1>
		</div>
	}
}


#[component]
pub fn TestPage(cx: Scope) -> impl IntoView {
	view! { 
		cx,
		<div>
			<h1>"Test Page"</h1>
	        	<h2>"nonono"</h2>
			
		</div>
		
	}
}
