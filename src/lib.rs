use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, HtmlInputElement};
use utils::ExpectLog;


mod sqlite;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn dynamic_loading_div_contents(output: sqlite::Table, div_container: &HtmlElement) {
    let mut combined_contents = String::new();

    for row in &output.contents {
        let empty = String::new();


        let mut details = &markdown_to_html::markdown(row.get("details").unwrap_or(&empty));
        if details == "Sorry, this did not seem to work! Maybe your markdown was not well formed, have you hit [Enter] after your last line?" {
            details = row.get("details").unwrap_or(&empty);
        }

        let address = format!("<a href=\"{0}\">{1}<br>{2}, {3}<br>{4}</a>",
            row.get("url_address").unwrap(),
            row.get("street").unwrap(),
            row.get("city").unwrap(),
            row.get("state").unwrap(),
            row.get("zipcode").unwrap()
        );

        combined_contents.push_str(&format!(
            "<div class=\"main\">
                <p class=\"status {7}\">Status: {7}</p>
                <p class=\"org-name\"><a href=\"{10}\">{9}</a>:</p>
                <h2 class=\"name\"><a href=\"{3}\"> {0}</a></h2>
                <p class=\"hours\"><b>Hours:</b> {8}</p>
                <div class=\"details\"> 
                    <p>{1:1}</p>
                </div>
                <div class=\"location\">
                    {11}
                </div>
                <div class=\"contact\">
                    <p>
                        <a href=\"{3}\">{2}</a> | <a href=\"mailto:{4}\">{4}</a> | <a href=\"{5}\">{6}</a>
                    </p>
                </div>
            </div>",
            row.get("service_name").unwrap_or(&empty),
            details,
            utils::simplify_url(row.get("service_website").unwrap()),
            row.get("service_website").unwrap(),
            row.get("service_email").unwrap(),
            row.get("service_uri_phone").unwrap(),
            utils::uri_phone_to_display(row.get("service_uri_phone").unwrap()),
            row.get("status").unwrap(),
            row.get("open_hours").unwrap(),
            row.get("org_name").unwrap(),
            row.get("org_website").unwrap(),
            address
        ));
    }
    div_container.set_inner_html(&combined_contents);
}

#[wasm_bindgen]
pub async fn main() {
    let db = utils::get_file_bytes("database/database.db").await;

    let window = web_sys::window().expect_log("No window found");
    let document = window.document().expect_log("No document found");

    let div_container = document.get_element_by_id("dynamic-content")
        .expect_log("Element with ID 'dynamic-content' not found")
        .dyn_into::<HtmlElement>()
        .expect_log("Failed to cast element to HtmlElement");

    let input_box = document.get_element_by_id("query")
        .expect_log("Element with ID 'query' not found")
        .dyn_into::<HtmlInputElement>()
        .expect_log("Failed to cast element to HtmlInputElement");

    let closure = Closure::<dyn Fn()>::new(move || {
        let input = input_box.value();
        //let mut words = input.split(' ');
        /*
        for word in words {
            log(&word);
        }*/

        // this is the base search the non advanced searching

        // For some reason L.unit makes it error????
        let query = format!("
            SELECT O.name AS org_name, S.name AS service_name, S.status, 
            S.details, S.open_hours, S.is_online_only, S.is_application_only, 
            S.eligibility, OrgC.website AS org_website, ServC.website AS service_website,
            ServC.email AS service_email, ServC.uri_phone AS service_uri_phone,
            L.url_address, L.street, L.city, L.state, L.zipcode

            FROM Services AS S
            JOIN Contacts AS ServC ON ServC.id = S.contact_id
            JOIN Orgs AS O ON O.id = S.org_id
            JOIN Contacts AS OrgC ON OrgC.id = O.contact_id
            JOIN Keywords AS K ON S.keyword_id = K.id
            JOIN Locations AS L ON S.location_id = L.id

            WHERE O.name LIKE '%{0}%' 
            OR S.name LIKE '%{0}%'
            OR S.details LIKE '%{0}%'
            OR O.description LIKE '%{0}%'
            OR K.service_type LIKE '%{0}%'
            OR K.quick_filter LIKE '%{0}%'
            OR K.service_groups LIKE '%{0}%'
            OR K.tags LIKE '%{0}%'
            OR K.tag_groups LIKE '%{0}%';", input);

        log(&query);

        //words.next().unwrap_or(""));
        let output = sqlite::query(&db, &query);
        dynamic_loading_div_contents(output, &div_container);
    });

    document.get_element_by_id("submit")
        .expect_log("Element with ID 'submit' not found")
        .dyn_ref::<HtmlElement>()
        .expect_log("Failed to cast element to HtmlElement")
        .set_onclick(Some(closure.as_ref().unchecked_ref()));

    closure.forget();
}
