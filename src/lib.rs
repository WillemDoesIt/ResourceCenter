use std::collections::HashMap;
use url::Url;
use utils::ExpectLog;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{CssStyleDeclaration, Element, Event, HtmlElement, HtmlInputElement, window};

mod sqlite;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn dynamic_loading_div_contents(
    output: sqlite::Table,
    div_container: &HtmlElement,
    div_result_count: &HtmlElement,
    duration_ms: i32,
) {
    /// ### Description
    /// Creates a string for the address based on contents from the row
    /// This is necessary because unit may be absent sometimes
    ///
    /// ### Example
    /// ```rust
    /// let address = build_address(&row);
    /// ```
    fn build_address(row: &HashMap<String, String>) -> String {
        /*
        if !row.get("unit").expect_log("get unit error").is_empty() {
            return format!(
                "<a href=\"{0}\">{1},<br>{2},<br>{3},<br>{4},<br>{5}</a>",
                row.get("url_address").expect_log("error"),
                row.get("street").expect_log("error"),
                row.get("unit").expect_log("error"),
                row.get("city").expect_log("error"),
                row.get("state").expect_log("error"),
                row.get("zipcode").expect_log("error")
            );
        }
        */
        return format!(
            "<a href=\"{0}\">{1},<br>{2},<br>{3},<br>{4}</a>",
            row.get("url_address").expect_log("error"),
            row.get("street").expect_log("error"),
            row.get("city").expect_log("error"),
            row.get("state").expect_log("error"),
            row.get("zipcode").expect_log("error")
        );
    }

    /// ### Description
    /// Gets the markdown details to be formatted as html
    /// This is necessary as the library sucks and instead of returning a result sometimes may
    /// return a verbose string explaining it errored.
    ///
    /// ### Example
    /// ```rust
    /// let details = fix_details(&row);
    /// ```
    fn fix_details(row: &HashMap<String, String>) -> String {
        let init_details = markdown_to_html::markdown(row.get("details").unwrap_or(&String::new()));
        if init_details
            == "Sorry, this did not seem to work! Maybe your markdown was not well formed, have you hit [Enter] after your last line?"
        {
            row.get("details").unwrap_or(&String::new()).to_string()
        } else {
            init_details
        }
    }

    /// ### Description
    /// Turns a uri_phone number into how it should be displayed
    ///
    /// ### Example
    /// ```rust
    /// uri_phone_to_display("tel://5093254541"); // returns "(509) 325-4541"
    /// uri_phone_to_display("tel://15093254541"); // returns "+1 (509) 325-4541"
    /// uri_phone_to_display("tel://5093254541.ext1127"); // returns "(509) 325-4541 ext. 1127"
    /// ```
    fn uri_phone_to_display(uri_phone: &str) -> String {
        let digits = uri_phone.strip_prefix("tel://").unwrap_or(uri_phone);

        match digits.len() {
            10 => {
                format!("({}) {}-{}", &digits[0..3], &digits[3..6], &digits[6..10])
            }
            11 => {
                format!(
                    "+1 ({}) {}-{}",
                    &digits[1..4],
                    &digits[4..7],
                    &digits[7..11]
                )
            }
            n if n > 12 => {
                format!(
                    "({}) {}-{}  ext. {}",
                    &digits[0..3],
                    &digits[3..6],
                    &digits[6..10],
                    &digits[14..]
                )
            }
            _ => digits.to_string(),
        }
    }

    /// ### Description
    /// Simplifies a URL by removing common prefixes and returning only the base domain
    ///
    /// This function:
    /// - Strips the `https://` or `http://` scheme prefix if present
    /// - Removes the leading `www.` if present
    /// - Discards any path, query parameters, or fragments after the domain
    ///
    /// ### Example
    /// ```rust
    /// let url = "https://www.example.com/path?query=1";
    /// let simplified = simplify_url(url);
    /// assert_eq!(simplified, "example.com");
    /// ```
    fn simplify_url(input: &str) -> String {
        let input = input
            .strip_prefix("https://")
            .or_else(|| input.strip_prefix("http://"))
            .unwrap_or(input);

        let input = input.strip_prefix("www.").unwrap_or(input);

        // Cut off any path, query, or fragment
        input.split('/').next().unwrap_or("").to_string()
    }

    fn guess_favicon_url(page_url: &str) -> Option<String> {
        let url = Url::parse(page_url).ok()?;
        let origin = format!("{}://{}", url.scheme(), url.host_str()?);
        Some(format!("{}/favicon.ico", origin))
    }

    let results: i32 = output.contents.len().try_into().unwrap_or(0);
    let result_count = format!("{results} Results in {duration_ms} miliseconds</p>");
    div_result_count.set_inner_html(&result_count);

    let mut combined_contents = String::new();
    let unwrap_fail_text = &format!("");

    for (id, row) in output.contents.iter().enumerate() {
        let service_name = row.get("service_name").unwrap_or(&unwrap_fail_text);
        let details = fix_details(&row);
        let website_url = row.get("service_website").unwrap_or(&unwrap_fail_text);
        let favicon_url = guess_favicon_url(website_url)
            .unwrap_or(format!("https://www.svgrepo.com/show/532532/globe-alt.svg"));
        let simple_url = simplify_url(website_url);
        let email = row.get("service_email").unwrap_or(&unwrap_fail_text);
        let uri_phone = row.get("service_uri_phone").unwrap_or(&unwrap_fail_text);
        let display_phone = uri_phone_to_display(uri_phone);
        let status = row.get("status").unwrap_or(&unwrap_fail_text);
        let hours = row.get("open_hours").unwrap_or(&unwrap_fail_text);
        let org_name = row.get("org_name").unwrap_or(&unwrap_fail_text);
        let org_url = row.get("org_website").unwrap_or(&unwrap_fail_text);
        let address = build_address(&row);

        combined_contents.push_str(&format!(
            "<div class=\"main read-more-container\">
            <div class=\"read-more-content\" id=\"read-more-content#{id}\">
                <p class=\"status {status}\">Status: {status}</p>
                <img class=\"favicon\" src=\"{favicon_url}\" />
                <p class=\"org-name\"><a href=\"{org_url}\">{org_name}</a>:</p>
                <h2 class=\"name\"><a href=\"{website_url}\">{service_name}</a></h2>
                <p class=\"hours\"><b>Hours:</b> {hours}</p>
                <div class=\"details\"> 
                    <p>{details}</p>
                </div>
                <div class=\"location\">
                    {address}
                </div>
                <div class=\"contact\">
                    <a href=\"{website_url}\"><div class=\"pill\">🌐 {simple_url}</div></a>
                    <a href=\"mailto:{email}\"><div class=\"pill\">📧 {email}</div></a> 
                    <a href=\"{uri_phone}\"><div class=\"pill\">📞 {display_phone}</div></a>
                </div>
            </div>
            </div>"
        ));
    }

    div_container.set_inner_html(&combined_contents);
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    let db = utils::get_file_bytes("database/database.db").await;

    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document");

    let div_container = document
        .get_element_by_id("dynamic-content")
        .expect_log("Element with ID 'dynamic-content' not found")
        .dyn_into::<HtmlElement>()
        .expect_log("Failed to cast element to HtmlElement");

    let div_result_count = document
        .get_element_by_id("dynamic-result-count")
        .expect_log("Element with ID 'dynamic-result-count' not found")
        .dyn_into::<HtmlElement>()
        .expect_log("Failed to cast element to HtmlElement");

    let search_input = document
        .get_element_by_id("search-input")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();

    let group_input = document
        .get_element_by_id("group-input")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();

    let search_input_clone = search_input.clone();
    let group_input_clone = group_input.clone();

    let closure = Closure::<dyn Fn()>::new(move || {
        let search_val = search_input_clone.value();
        let group_val = group_input_clone.value();

        if search_val.is_empty() {
            return;
        } // helps a lot with performance

        let query = format!(
            "
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

            WHERE K.service_type = '{search_val}'
            ORDER BY 
                CASE 
                    WHEN K.service_groups LIKE '%{group_val}%' THEN 0 
                    ELSE 1 
                END;
        "
        );

        use js_sys::Date;
        let start = Date::now();

        let output = sqlite::query(&db, &query);

        let duration_ms = Date::now() - start;
        let duration_ms = duration_ms.round() as u32;

        dynamic_loading_div_contents(
            output,
            &div_container,
            &div_result_count,
            duration_ms.try_into().unwrap(),
        );
    });

    // Trigger on input changes
    search_input.set_oninput(Some(closure.as_ref().unchecked_ref()));
    group_input.set_oninput(Some(closure.as_ref().unchecked_ref()));

    // Trigger on dropdown click
    let document = web_sys::window().unwrap().document().unwrap();
    let dropdowns = ["dropdown", "group-dropdown"];

    for id in dropdowns.iter() {
        if let Some(dropdown) = document.get_element_by_id(id) {
            let closure_ref = closure.as_ref().clone();
            let _ = dropdown.add_event_listener_with_callback(
                "click",
                closure_ref.dyn_ref::<js_sys::Function>().unwrap(),
            );
        }
    }

    closure.forget();

    Ok(())
}
