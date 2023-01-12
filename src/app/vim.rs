use leptos::*;
use leptos_meta::*;
use leptos_router::*;


#[component]
pub fn Vim(cx: Scope) -> impl IntoView {
	println!("Vim Page called");

	view! {
		cx,
		<div>
			<h1>"Vim Page"</h1>
			
		</div>
	}
} 
