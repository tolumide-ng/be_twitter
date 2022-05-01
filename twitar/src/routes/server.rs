use hyper::{Method};

use crate::errors::response::TError;
use crate::helpers::commons::UserId;
use crate::helpers::request::req_query;
use crate::startup::server::{AppState, CurrentUser};
use crate::helpers::response::{ApiBody, ResponseBuilder};
use crate::{helpers::response::TResult};
use crate::controllers::{not_found, authorize_bot, 
    health_check, handle_redirect, revoke_token, refresh_token, user_lookup, 
    get_timeline, handle_delete, request_token
};

pub struct Routes;


impl Routes {
    // pub async fn _routes_hof(state: &mut AppState) -> Box<TResult<Box<dyn Fn(AppState) -> Pin<Box<dyn Future<Output = TResult<ApiBody>>>>>>> {
    //     let req = &state.req;
        
    //     let protected_paths = ["/timeline"];
    
    //     match protected_paths.contains(&req.uri().path()) {
    //         true => {
    //             let query = req.uri().query();
    //             let user_id = req_query(query, "user_id");
    //             if let Ok(parsed_user_id) = UserId::parse(user_id) {
    //                 let auth_user = parsed_user_id.verify(&state.db_pool).await;
    //                 let v2_credentials = parsed_user_id.v2_credentials(&state.db_pool).await;
                    
    //                 if auth_user.is_ok() && v2_credentials.is_ok() {
    //                     let user_credentials = CurrentUser::new(auth_user.unwrap(), v2_credentials.unwrap());
    //                     state.with_user(user_credentials);
    //                     // Pointer for heap allocation for the return type
    //                     return Box::new(
    //                         // we should be able to fail
    //                         Ok(
    //                             // Pointer for heap allocation for the closure (returned function)
    //                             Box::new(|arg: AppState| {
    //                                 // Pointer for the heap allocation for the routes function (immediately invoked when the parent closure is called)
    //                                 // Box::pin(routes(arg))
    //                             })
    //                         )
    //                     );
    //                 }
    
    //             }
    //         }
    //         false => {}
    //     }
    
    //     return Box::new(Err(TError::Unauthenticated("")));
    // }

    pub async fn auth_middleware(state: AppState) -> TResult<AppState> {
        let req = &state.req;
        
        let protected_paths = ["/enable", "/revoke", "/remove", "/refresh", "/user", "/timeline"];

        
        match protected_paths.contains(&req.uri().path()) {
            true => {
                let query = req.uri().query();
                let user_id = req_query(query, "user_id");
                println!("\n\nTHE CONTENT>>>>||||<<<< {:#?}\n\n", user_id);
                if let Ok(parsed_user_id) = UserId::parse(user_id) {
                    println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::;");
                    let auth_user = parsed_user_id.verify(&state.db_pool).await?;
                    let v2_credentials = parsed_user_id.v2_credentials(&state.db_pool).await?;
                    
                    if auth_user.v1_active && auth_user.v2_active {
                        let user_credentials = CurrentUser::new(auth_user, v2_credentials);
                        let new_state = AppState::add_user(state, user_credentials);
                        // Pointer for heap allocation for the return type
                        return Ok(new_state)
                    }

                }
            }
            false => {
                 return Ok(state)
            }
        }

        return Err(TError::Unauthenticated(""));
    }

    pub async fn wrapper(state: AppState) -> TResult<ApiBody> {
        let auth = Self::auth_middleware(state).await;
        if let Ok(new_state) = auth {
            println!(":[[[[[[[[[[[[[[[[]]]]]]]]]]]]]]]]]]]]]]]]]]]]:::::");
            return Self::routes(new_state).await
        }

        println!("ERROR--ERROR--ERROR--ERROR--ERROR--ERROR--ERROR--ERROR--");
        return ResponseBuilder::new("Missing or Invalid user id".into(), Some(""), 401).reply();
    }


    pub async fn routes(
        state: AppState
    ) -> TResult<ApiBody> {
        // migrate this to [routerify](https://docs.rs/routerify/latest/routerify/) eventually
        // OR JUST USE procedural attribute macros (so this looks like the way rocket annotates controllers with route properties)
        
        let req = &state.req;

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
}