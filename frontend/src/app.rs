// the file that will act as the root component where the router will be 
// defined and what component to render according to the route matched

use {
    crate::{
        components::{article::Article, articles::Articles, not_found::PageNotFound},
        routes::*,
    },
    yew::html,
    yew_functional::function_component,
    yew_router::{components::RouterAnchor, prelude::Route, router::Router, switch::Permissive},
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div style="display: flex; flex: 1; flex-direction: column; align-items: center; max-width: 1024px; margin: 0 auto;">
            <RouterAnchor<AppRoute> route=AppRoute::Articles>
                <p>{"Articles"}</p>
            </RouterAnchor<AppRoute>>
            <div style="display: flex; flex: 1;">
                <Router<AppRoute, ()>
                    render = Router::render(move |route: AppRoute| {
                        match route {
                            AppRoute::Article { id } => html! {
                                <Article id={id} />
                            },
                            AppRoute::Articles => html! {
                                <Articles />
                            },
                            AppRoute::PageNotFound(Permissive(None)) => html! {<PageNotFound />},
                            AppRoute::PageNotFound(Permissive(Some(missed_route))) => html! {
                                <PageNotFound page_name={format!(" Page '{}' not found.", missed_route)} />
                            }
                        }
                    })
                    redirect = Router::redirect(|route: Route<()>| {
                        AppRoute::PageNotFound(Permissive(Some(route.route)))
                    })
                />
            </div>
            <footer>
                <p>{"guimauve"}</p>
            </footer>
        </div>
    }
}