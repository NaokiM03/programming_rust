extern crate iron;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;

extern crate router;
use router::Router;

extern crate urlencoded;

use urlencoded::UrlEncodedBody;

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
        <title>GCD Calculator</title>
        <body>
            <p>hoge</p>
            <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="n"/>
                <button type"submit">Compute GCD</button>
            </form>
        </body>
    "#);

    Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map
    };

    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => nums
    };

}

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("gcd", post_gcd, "gcd");

    println!("Serving on http:localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}
