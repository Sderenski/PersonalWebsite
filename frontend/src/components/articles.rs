use {
    crate::{
        entities::interfaces::{IArticle, Status},
        routes::AppRoute,
        services::{articles::get_article_list, future::handle_future},
    },
    yew::html,
    yew_functional::{function_component, use_effect_with_deps, use_state},
    yew_router::components::RouterAnchor,
};


// Very similar to that of React or View. 
// The only difference is that we had to predefine the promises in 
// Rust instead of just using them like in JS
#[function_component(Articles)]
pub fn articles() -> Html {
    // Setting states
    let (is_loading, set_loading) = use_state(|| false);
    let (articles, set_articles) = use_state(move || vec![]);

    // Interacting with the states via effects
    use_effect_with_deps(
        move |_| {
            set_loading(true);
            let future = async { get_article_list().await };
            handle_future(future, move |data: Result<Vec<IArticle>, Status>| {
                match data {
                    Ok(articles) => set_articles(articles),
                    Err(_) => (),
                };
                set_loading(false);
            });
            || {}
        },
        (),
    );
    html! {
        {if *is_loading {
            html! {}
        } else {
            html! {
                <div style="display: flex; justify-content: center; flex: 1">
                    <div>
                        {for articles.iter().map(move |article| {
                            html! {
                                <>
                                    <div align_items="center">
                                        <RouterAnchor<AppRoute> route=AppRoute::Article{id: article.id}>
                                            {&article.title}
                                        </RouterAnchor<AppRoute>>
                                    </div>
                                    <hr/>
                                </>
                            }
                        })
                    }
                </div>
            </div>
            }
        }}
    }
}