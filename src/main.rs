use std::convert::Infallible;

use warp::{Filter, reject::{Rejection, self}, reply::Reply};

#[tokio::main]
async fn main() {

    let filter = warp::path::param()
    .and_then(|id: u64| async move {
        if id != 0 {
            Ok(format!("Hello #{}", id))
        } else {
            Err(warp::reject::not_found())
        }
    });

    let hello = 
        warp::path("project")
        .and(get_project()
        )
        .map(|a|{
            println!("project : {:?}", &a);
            format!("project : {:?}", &a)
        });

    warp::serve(hello)
        .run(([0,0,0,0], 3030))
        .await
}

fn get_project() -> impl Filter<Extract = (Project,), Error = Rejection> + Copy {
    warp::path::param().and_then(|id : i32| async move {
        match id {
            1 => Ok(Project{
                id: id+1,
                name: "Project 1".to_string()
            }),
            2 => Ok(Project{
                id: id+1,
                name: "Project 2".to_string()
            }),
            _ => Err(warp::reject()),
            
        }
    })
}

#[derive(Debug)]
struct Project {
    id: i32,
    name: String
}