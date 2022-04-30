use futures::Future;
use std::pin::Pin;
use futures::future::IntoFuture;
use hyper::{Method};

use crate::errors::response::TError;
use crate::helpers::commons::UserId;
use crate::helpers::request::req_query;
use crate::startup::server::{AppState, CurrentUser};
use crate::helpers::response::{ApiBody};
use crate::{helpers::response::TResult};
use crate::controllers::{not_found, authorize_bot, 
    health_check, handle_redirect, revoke_token, refresh_token, user_lookup, 
    get_timeline, handle_delete, request_token
};

// fn user_hof () {}

fn hof<'a>(step_value: & 'a i32) -> Box<dyn Fn(i32) -> i32 + 'a> {
    Box::new(move |x: i32| x + step_value) 
}

fn hof_0<F>(value: i32, step: F) -> i32 where F: Fn(i32) -> i32 {
    step(value)
}

// type RequestOutput = ;
type RequestOutput = dyn futures::Future<Output = TResult<ApiBody>>;

async fn app_middleware<F>(state: AppState, controller: F) -> TResult<ApiBody> where F: Fn(AppState) -> TResult<ApiBody> {
    let abc = controller(state);
    abc
}

fn thing_returning_closure() -> impl Fn(i32) -> bool {
    println!("here's a closure for you!");
    |x: i32| x % 3 == 0
}
// impl futures::Future<Output = TResult<ApiBody>>

async fn names() {}


pub async fn routes_wrapper(state: AppState) -> Box<TResult<Box<dyn Fn(AppState) -> Pin<Box<dyn Future<Output = TResult<ApiBody>>>>>>> {
    let req = &state.req;
    
    let protected_paths = ["/timeline"];

    if protected_paths.contains(&req.uri().path()) {
        let query = req.uri().query();
        let user_id = req_query(query, "user_id");
        if let Ok(parsed_user_id) = UserId::parse(user_id) {
            let auth_user = parsed_user_id.verify(&state.db_pool).await;
            let v2_credentials = parsed_user_id.v2_credentials(&state.db_pool).await;
            
            if auth_user.is_ok() && v2_credentials.is_ok() {
                let user_credentials = CurrentUser::new(auth_user.unwrap(), v2_credentials.unwrap());
                // Pointer for heap allocation for the return type
                return Box::new(
                    // we should be able to fail
                    Ok(
                        // Pointer for heap allocation for the closure (returned function)
                        Box::new(|arg: AppState| {
                            // Pointer for the heap allocation for the routes function (immediately invoked when the parent closure is called)
                            Box::pin(routes(arg))
                        })
                    )
                );
            }

        }

    }




    

    return Box::new(Err(TError::Unauthenticated("")));
}



pub async fn routes(
    state: AppState
) -> TResult<ApiBody> {
    // migrate this to [routerify](https://docs.rs/routerify/latest/routerify/) eventually
    // OR JUST USE procedural attribute macros (so this looks like the way rocket annotates controllers with route properties)
    

    // CREATE THE MIDDLEWARE AS A MACRO??

    match (req.method(), req.uri().path(), req.uri().query()) {
        (&Method::GET, "/", _) => health_check().await,
        (&Method::GET, "/enable", _) => authorize_bot(state).await,
        (&Method::GET, "/oauth/callback", _x) => handle_redirect(state).await,
        (&Method::POST, "/revoke", _) => revoke_token(state).await,
        (&Method::GET, "/refresh", _) => refresh_token(state).await,
        (&Method::GET, "/user", _x) => user_lookup(state).await,
        (&Method::GET, "/timeline", _) => get_timeline(state).await,
        (&Method::POST, "/remove", _) => handle_delete(state).await,
        (&Method::GET, "/oauth1", _) => request_token(state).await,
        _ => {
            not_found().await
        }
    }
}