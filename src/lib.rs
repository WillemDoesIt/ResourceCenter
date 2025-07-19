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
        let _address = build_address(&row);
        let url_address = row.get("url_address").unwrap_or(&unwrap_fail_text);

        combined_contents.push_str(&format!(
            "<div class=\"main read-more-container\">
            <div class=\"read-more-content\" id=\"read-more-content#{id}\">
                <img class=\"favicon\" src=\"{favicon_url}\" />
                <p class=\"org-name\"><a href=\"{org_url}\">{org_name}</a>:</p>
                <h2 class=\"name\"><a href=\"{website_url}\">{service_name}</a></h2>
                <p class=\"hours\"><b>Hours:</b> {hours}</p>
                <div class=\"details\"> 
                    <p>{details}</p>
                </div>
                <div class=\"location\"></div>
                <div class=\"contact\">
                    <a href=\"{url_address}\"><div class=\"pill\">

<svg version=\"1.1\" id=\"Layer_1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" viewBox=\"0 0 512 512\" xml:space=\"preserve\">
    <g id=\"SVGRepo_bgCarrier\" stroke-width=\"0\"></g>
    <g id=\"SVGRepo_tracerCarrier\" stroke-linecap=\"round\" stroke-linejoin=\"round\"></g>
<g id=\"SVGRepo_iconCarrier\"> 
    <g> <g> 
        <path d=\"M256,100.174c-46.03,0-83.478,37.448-83.478,83.478S209.97,267.131,256,267.131s83.478-37.448,83.478-83.478 S302.03,100.174,256,100.174z M256,233.74c-27.618,0-50.087-22.469-50.087-50.087c0-27.618,22.469-50.087,50.087-50.087 c27.618,0,50.087,22.469,50.087,50.087C306.087,211.271,283.618,233.74,256,233.74z\"></path> 
    </g> </g> 
    <g> <g> 
        <path d=\"M256,0C154.734,0,72.347,82.387,72.347,183.653c0,70.835,21.232,98.615,169.771,320.928 c6.603,9.882,21.148,9.903,27.764,0c149.325-223.389,169.771-250.792,169.771-320.928C439.652,82.387,357.266,0,256,0z M256.001,465.261C122.631,265.788,105.74,241.56,105.74,183.653C105.739,100.799,173.146,33.391,256,33.391 s150.261,67.407,150.261,150.261C406.261,239.551,393.41,259.681,256.001,465.261z\"></path> 
    </g> </g> 
    </g>
</svg>

            Location</div></a>
                    <a href=\"{website_url}\"><div class=\"pill\">

<svg version=\"1.1\" id=\"_x32_\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" 
	 viewBox=\"0 0 512 512\"  xml:space=\"preserve\">
<g>	
        <path class=\"st0\" d=\"M425.393 86.607C380.146 41.361 319.988 16.442 256 16.442S131.854 41.361 86.607 86.607C41.361 131.854 16.442 192.012 16.442 256S41.36 380.146 86.607 425.393C131.854 470.64 192.012 495.558 256 495.558s124.146-24.918 169.393-70.165C470.64 380.146 495.558 319.988 495.558 256S470.64 131.854 425.393 86.607zM386.027 242.5c-1.141-38.785-7.187-75.873-17.566-108.605c16.922-4.791 32.653-10.738 47.349-17.882c30.041 34.253 49.265 78.207 52.307 126.487h-82.09zM242.5 466.638c-20.989-5.949-40.869-25.655-57.048-56.984a228.17 228.17 0 0 1-5.844-12.219c11.593-2.202 23.68-3.935 36.277-5.158a427.471 427.471 0 0 1 26.615-1.739v76.1zm27-76.15c21.326.656 42.336 2.977 62.887 6.956a228.196 228.196 0 0 1-5.839 12.209c-16.179 31.329-36.059 51.036-57.048 56.984v-76.149zm-27-26.963c-9.7.314-19.444.927-29.225 1.877c-15.111 1.467-29.588 3.622-43.422 6.429c-9.922-30.536-15.727-65.521-16.87-102.331H242.5v94.025zM152.984 242.5c1.143-36.816 6.95-71.805 16.874-102.345c23.712 4.87 47.989 7.663 72.642 8.375v93.97h-89.516zM242.5 121.523c-21.327-.657-42.338-2.984-62.891-6.959a228.53 228.53 0 0 1 5.843-12.218c16.179-31.33 36.058-51.037 57.048-56.985v76.162zm27-76.161c20.989 5.948 40.869 25.655 57.048 56.985a227.875 227.875 0 0 1 5.871 12.282c-10.417 1.958-21.302 3.531-32.689 4.73a429.567 429.567 0 0 1-30.229 2.096V45.362zm81.038 44.597c-6.618-12.816-13.906-24.061-21.732-33.669c24.658 9.017 47.19 22.48 66.629 39.411c-11.359 4.975-23.438 9.21-36.287 12.755c-2.686-6.4-5.554-12.579-8.61-18.497zm-189.076 0c-3.041 5.888-5.896 12.035-8.57 18.401c-13.017-3.574-25.073-7.775-36.326-12.659c19.438-16.93 41.97-30.393 66.628-39.41c-7.826 9.607-15.114 20.852-21.732 33.668zm-17.892 43.84c-10.398 32.755-16.455 69.878-17.597 108.701h-82.09c3.041-48.266 22.254-92.208 52.281-126.457c14.553 7.039 30.243 12.923 47.406 17.756zM125.973 269.5c1.142 38.814 7.196 75.928 17.589 108.678c-16.978 4.812-32.778 10.77-47.359 17.823c-30.049-34.255-49.278-78.215-52.32-126.501h82.09zm26.909 134.116a257.5 257.5 0 0 0 8.58 18.425c6.618 12.816 13.906 24.061 21.731 33.669c-24.647-9.014-47.171-22.469-66.604-39.389c11.31-4.92 23.428-9.151 36.293-12.705zm206.215.051c12.792 3.547 24.916 7.797 36.26 12.702c-19.421 16.898-41.926 30.336-66.55 39.341c7.825-9.608 15.113-20.853 21.732-33.669c3.036-5.88 5.887-12.018 8.558-18.374zm-16.954-31.825c-23.709-4.874-47.99-7.655-72.643-8.367V269.5h89.516c-1.144 36.815-6.95 71.803-16.873 102.342zM269.5 242.5v-94.023a456.82 456.82 0 0 0 33.056-2.267c13.854-1.458 27.024-3.464 39.606-5.993c9.912 30.525 15.712 65.492 16.855 102.283H269.5zm146.193 153.618l.068-.141c-14.598-7.008-30.463-12.952-47.339-17.75c10.403-32.762 16.463-69.894 17.605-108.728h82.089c-3.045 48.343-22.315 92.347-52.423 126.619z\"/>
</g>
</svg>

             Website</div></a>
                    <a href=\"mailto:{email}\"><div class=\"pill\">

<svg version=\"1.1\" id=\"_x32_\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" 
	 viewBox=\"0 0 512 512\"  xml:space=\"preserve\">
<g>
	<path class=\"st0\" d=\"M510.678,112.275c-2.308-11.626-7.463-22.265-14.662-31.054c-1.518-1.915-3.104-3.63-4.823-5.345
		c-12.755-12.818-30.657-20.814-50.214-20.814H71.021c-19.557,0-37.395,7.996-50.21,20.814c-1.715,1.715-3.301,3.43-4.823,5.345
		C8.785,90.009,3.63,100.649,1.386,112.275C0.464,116.762,0,121.399,0,126.087V385.92c0,9.968,2.114,19.55,5.884,28.203
		c3.497,8.26,8.653,15.734,14.926,22.001c1.59,1.586,3.169,3.044,4.892,4.494c12.286,10.175,28.145,16.32,45.319,16.32h369.958
		c17.18,0,33.108-6.145,45.323-16.384c1.718-1.386,3.305-2.844,4.891-4.43c6.27-6.267,11.425-13.741,14.994-22.001v-0.064
		c3.769-8.653,5.812-18.171,5.812-28.138V126.087C512,121.399,511.543,116.762,510.678,112.275z M46.509,101.571
		c6.345-6.338,14.866-10.175,24.512-10.175h369.958c9.646,0,18.242,3.837,24.512,10.175c1.122,1.129,2.179,2.387,3.112,3.637
		L274.696,274.203c-5.348,4.687-11.954,7.002-18.696,7.002c-6.674,0-13.276-2.315-18.695-7.002L43.472,105.136
		C44.33,103.886,45.387,102.7,46.509,101.571z M36.334,385.92V142.735L176.658,265.15L36.405,387.435
		C36.334,386.971,36.334,386.449,36.334,385.92z M440.979,420.597H71.021c-6.281,0-12.158-1.651-17.174-4.552l147.978-128.959
		l13.815,12.018c11.561,10.046,26.028,15.134,40.36,15.134c14.406,0,28.872-5.088,40.432-15.134l13.808-12.018l147.92,128.959
		C453.137,418.946,447.26,420.597,440.979,420.597z M475.666,385.92c0,0.529,0,1.051-0.068,1.515L335.346,265.221L475.666,142.8
		V385.92z\"/>
</g>
</svg>

            Email</div></a> 
                    <a href=\"{uri_phone}\"><div class=\"pill\">

<svg viewBox=\"0 0 24 24\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">
    <g id=\"SVGRepo_bgCarrier\" stroke-width=\"0\"></g>
    <g id=\"SVGRepo_tracerCarrier\" stroke-linecap=\"round\" stroke-linejoin=\"round\"></g>
    <g id=\"SVGRepo_iconCarrier\"> 
        <path d=\"M18.4 20.75H18.17C15.5788 20.4681 13.0893 19.5846 10.9 18.17C8.86618 16.8747 7.13938 15.1513 5.84 13.12C4.42216 10.925 3.53852 8.42823 3.26 5.83001C3.22816 5.52011 3.2596 5.20696 3.35243 4.90958C3.44525 4.6122 3.59752 4.33677 3.8 4.10001C3.99694 3.86008 4.24002 3.66211 4.51486 3.51782C4.78969 3.37354 5.09068 3.28587 5.4 3.26001H8C8.56312 3.26058 9.10747 3.46248 9.53476 3.82925C9.96205 4.19602 10.2441 4.70349 10.33 5.26001C10.425 5.97489 10.6028 6.67628 10.86 7.35001C11.0164 7.77339 11.0487 8.23264 10.9531 8.67375C10.8574 9.11485 10.6378 9.51947 10.32 9.84001L9.71 10.45C10.6704 11.9662 11.9587 13.2477 13.48 14.2L14.09 13.6C14.4105 13.2822 14.8152 13.0626 15.2563 12.9669C15.6974 12.8713 16.1566 12.9036 16.58 13.06C17.2545 13.3148 17.9556 13.4926 18.67 13.59C19.236 13.6751 19.7515 13.9638 20.1198 14.402C20.488 14.8403 20.6837 15.3978 20.67 15.97V18.37C20.67 18.9942 20.4227 19.593 19.9823 20.0353C19.5419 20.4776 18.9442 20.7274 18.32 20.73L18.4 20.75ZM8 4.75001H5.61C5.49265 4.75777 5.37809 4.78924 5.27325 4.84252C5.1684 4.8958 5.07545 4.96979 5 5.06001C4.92658 5.14452 4.871 5.24302 4.83663 5.34957C4.80226 5.45612 4.7898 5.56852 4.8 5.68001C5.04249 8.03679 5.83362 10.304 7.11 12.3C8.28664 14.1467 9.85332 15.7134 11.7 16.89C13.6973 18.1798 15.967 18.9878 18.33 19.25C18.4529 19.2569 18.5759 19.2383 18.6912 19.1953C18.8065 19.1522 18.9117 19.0857 19 19C19.1592 18.8368 19.2489 18.6181 19.25 18.39V16C19.2545 15.7896 19.1817 15.5848 19.0453 15.4244C18.9089 15.2641 18.7184 15.1593 18.51 15.13C17.6839 15.0189 16.8724 14.8177 16.09 14.53C15.9359 14.4724 15.7686 14.4596 15.6075 14.4933C15.4464 14.5269 15.2982 14.6055 15.18 14.72L14.18 15.72C14.0629 15.8342 13.912 15.9076 13.7499 15.9292C13.5877 15.9508 13.423 15.9195 13.28 15.84C11.1462 14.6342 9.37997 12.8715 8.17 10.74C8.08718 10.598 8.05402 10.4324 8.07575 10.2694C8.09748 10.1065 8.17286 9.95538 8.29 9.84001L9.29 8.84001C9.40468 8.72403 9.48357 8.57751 9.51726 8.41793C9.55095 8.25835 9.53802 8.09244 9.48 7.94001C9.19119 7.15799 8.98997 6.34637 8.88 5.52001C8.85519 5.30528 8.75133 5.10747 8.58865 4.96513C8.42597 4.82278 8.21613 4.7461 8 4.75001Z\" ></path> 
    </g>
</svg>

             Phone</div></a>
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
