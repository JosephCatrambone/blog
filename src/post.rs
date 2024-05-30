use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::datamodel::*;

// #[component]
// pub fn Post() -> impl IntoView {
// 	// Provides context that manages stylesheets, titles, meta tags, etc.
// 	provide_meta_context();

// 	view! {
// 		// injects a stylesheet into the document <head>
// 		// id=leptos means cargo-leptos will hot-reload this stylesheet
// 		<Stylesheet id="leptos" href="/pkg/joseph-blog.css"/>

// 		// sets the document title
// 		<Title text="Joseph's Blog"/>

// 		// content for this welcome page
// 		<Router>
// 			<main>
// 				<Routes>
// 					<Route path="" view=HomePage/>
// 					<Route path="/*any" view=NotFound/>
// 				</Routes>
// 			</main>
// 		</Router>
// 	}
// }

/// Renders the home page of your application.
#[component]
fn Post(pid: PostID) -> impl IntoView {
	//let (count, set_count) = create_signal(0);
	//let on_click = move |_| set_count.update(|count| *count += 1);

	// Fetch the data from the database and render it.
	let content = "<p>Test.</p>";

	view! {
		<div inner_html=content/>
		//<button on:click=on_click>"Click Me: " {count}</button>
	}
}
