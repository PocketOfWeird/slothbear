use rocket::local::Client;
use rocket::http::{Status, ContentType};

use crate::routes;


fn start_client() -> Client {
    let rocket = rocket::ignite().mount( 
        "/", 
        routes_with_openapi![
                routes::index, 
                routes::post_render,
            ]
    );
    let client = Client::new(rocket).expect("valid rocket instance");
    return client;
}

#[test]
fn test_index() {
    let client = start_client();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_post_render() {
    let client = start_client();
    
    let request_body = r#"
        {
            "pathProject": "Hartzler/MyAnimation",
            "pathOutput": "Hartzler/MyAnimation/images/first_try",
            "pathScene": "Hartzler/MyAnimation/scenes/awesomesauce.mb",
            "outputFileName": "awesome",
            "camera": "persp01",
            "frameWidth": 1920,
            "frameHeight": 1080,
            "frames": "1-240"
        }
    "#;
/* 
    let assumed_response = json!(
        {
            "renderer":"Arnold",
            "pathProject": "Hartzler/MyAnimation",
            "pathOutput": "Hartzler/MyAnimation/images/first_try",
            "pathScene": "Hartzler/MyAnimation/scenes/awesomesauce.mb",
            "outputFileName": "awesome",
            "camera": "persp01",
            "frameWidth": 1920,
            "frameHeight": 1080,
            "frames": "1-240",
            "frame_step": 1,
            "split_chunks": 5,
            "rpUser": null,
            "rpJobName": null
        }
    );
 */    
    let /* mut */ response = client.post("/slothbear/render")
                                .header(ContentType::JSON)
                                .body(request_body).dispatch();
    

    assert_eq!(response.status(), Status::Ok);
/*    let body = response.body().unwrap().into_string().unwrap();
    assert_json_include!(actual: json!(body), expected: assumed_response); */
}