use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use utils::ExpectLog;

mod sqlite;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn main() {
    let db = utils::get_file_bytes("database/database.db").await;

    let query = format!("SELECT O.name AS org_name, S.name AS service_name, S.status, 
        S.details, S.open_hours, S.is_online_only, S.is_application_only, 
        S.eligibility, OrgC.website AS org_website, ServC.website AS service_website,
        ServC.email AS service_email, ServC.uri_phone AS service_uri_phone
        FROM Services AS S
        JOIN Contacts AS ServC ON ServC.id = S.contact_id
        JOIN Orgs AS O ON O.id = S.org_id
        JOIN Contacts AS OrgC ON OrgC.id = O.contact_id;");

    let output = sqlite::query(db, &query);

    sqlite::print_table(&output).await;

    let window = web_sys::window().expect_log("No window found");
    let document = window.document().expect_log("No document found");

    let div_container = document
        .get_element_by_id("dynamic-content")
        .expect_log("Element with ID 'dynamic-content' not found")
        .dyn_into::<HtmlElement>()
        .expect_log("Failed to cast element to HtmlElement");


    let mut combined_contents = String::new();

    for row in &output.contents {
        let empty = String::new(); // fallback value with a long enough lifetime


        let mut details = &markdown_to_html::markdown(row.get("details").unwrap_or(&empty));
        if details == "Sorry, this did not seem to work! Maybe your markdown was not well formed, have you hit [Enter] after your last line?" {
            details = row.get("details").unwrap_or(&empty);
        }

        combined_contents.push_str(&format!(
        "<div class=\"main\">
            <p class=\"status {7}\">Status: {7}</p>
            <h2 class=\"name\"><a href=\"{10}\">{9}</a>: <a href=\"{3}\"> {0}</a></h2>
            <p class=\"hours\">Hours: {8}</p>
            <h5>Details:</h5>
            <p>{1:1}</p>
            <div class=\"contact\">
                <p>
                    <a href=\"{3}\">{2}</a> | <a href=\"mailto:{4}\">{4}</a> | <a href=\"{5}\">{6}</a>
                </p>
            </div>
        </div>",
        row.get("service_name").unwrap_or(&empty),
        details,
        utils::simplify_url(row.get("service_website").unwrap_or(&empty)),
        row.get("service_website").unwrap_or(&empty),
        row.get("service_email").unwrap_or(&empty),
        row.get("service_uri_phone").unwrap_or(&empty),
        utils::uri_phone_to_display(row.get("service_uri_phone").unwrap_or(&empty)),
        row.get("status").unwrap_or(&empty),
        row.get("open_hours").unwrap_or(&empty),
        utils::force_length(row.get("org_name").unwrap_or(&empty), 15, ' '),
        row.get("org_website").unwrap_or(&empty)
    ));
    }

    div_container.set_inner_html(&combined_contents);
}
