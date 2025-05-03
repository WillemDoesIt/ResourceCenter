// PLEASE READ `../documentation/sqlite.md`
use crate::utils::{self, ExpectLog};
use sqlite_wasm_rs::{self as ffi, SQLITE_OK};
use std::collections::HashMap;
use std::ffi::{CStr, CString};

use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Table {
    pub attributes: Vec<String>,
    pub contents: Vec<HashMap<String, String>>,
}

/// ### Description
/// Prints a formatted table to the console
///
/// ### Example
///
/// ```rust
/// let table = Table { // table setup here ... // };
/// print_table(table);
/// ```
/// logs:
/// ```
/// attribute1  attribute2   attribute3
/// content     content      content
/// ```
pub async fn print_table(table: &Table) {
    let max_width = 20;

    fn format_record(record: &[String], max_width: usize) -> String {
        let mut formatted = Vec::new();
        for col in record {
            formatted.push(utils::force_length(col, max_width, ' '));
        }
        formatted.join(" ")
    }

    log(&format_record(&table.attributes, max_width));

    for (index, record) in table.contents.iter().enumerate() {
        let mut values = Vec::new();
        for attr in &table.attributes {
            let value = record.get(attr).unwrap_or(&String::new()).to_owned();
            values.push(value);
        }
        log(&format!("{0} {1}", (index+1).to_string(), format_record(&values, max_width)));
    }
}

/// ### Description
/// Takes in a read-only SQL query and returns the output Table.
///
/// ### Example
/// ```rust
/// let output: Table = query("SELECT Content FROM Table");
/// ```
pub fn query(db_bytes: Vec<u8>, query: &str) -> Table {
    let mut db = std::ptr::null_mut();
    let open_db_result = unsafe {
        ffi::sqlite3_open_v2(
            c"mem.db".as_ptr().cast(),
            &mut db as *mut _,
            ffi::SQLITE_OPEN_READWRITE | ffi::SQLITE_OPEN_CREATE,
            std::ptr::null(),
        )
    };
    assert_eq!(open_db_result, SQLITE_OK, "Failed to open SQLite database");

    let deserialization_result = unsafe {
        ffi::sqlite3_deserialize(
            db,                                 // The database connection
            c"main".as_ptr().cast(),          // Which DB to reopen with the deserialization
            db_bytes.as_ptr() as *mut u8,     // The serialized database content
            db_bytes.len() as i64,            // Number bytes in the deserialization
            db_bytes.len() as i64,            // Total size of buffer pData[]
            ffi::SQLITE_DESERIALIZE_READONLY, // Zero or more SQLITE_DESERIALIZE_* flags
        )
    };
    assert_eq!(
        deserialization_result, SQLITE_OK,
        "Failed to deserialize database."
    );

    let mut statement_handler = std::ptr::null_mut();
    let prepare_result = unsafe {
        ffi::sqlite3_prepare_v2(
            db,                                           // Database handle
            CString::new(query)
                .expect_log("Failed to get Cstring from query")
                .as_ptr().cast(),                         // SQL statement, UTF-8 encoded
            -1,                                           // Maximum length of zSql in bytes
            &mut statement_handler,                       // OUT: Statement handle
            std::ptr::null_mut(),                         // OUT: Pointer to unused portion of zSql
        )
    };
    assert_eq!(prepare_result, SQLITE_OK, "Failed to prepare SQL statement");

    fn ptr_to_string(ptr: *const i8) -> String {
        if ptr.is_null() {
            return String::new();
        }
        unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
    }

    let attribute_count = unsafe { ffi::sqlite3_column_count(statement_handler) };
    let mut attributes = Vec::new();
    for i in 0..attribute_count {
        let name_ptr = unsafe { ffi::sqlite3_column_name(statement_handler, i) };
        attributes.push(ptr_to_string(name_ptr));
    }

    let mut contents = Vec::new();
    while unsafe { ffi::sqlite3_step(statement_handler) } == ffi::SQLITE_ROW {
        let mut record = HashMap::new();
        for i in 0..attribute_count {
            let value_ptr = unsafe { ffi::sqlite3_column_text(statement_handler, i) };
            let value = ptr_to_string(value_ptr as *const i8);
            record.insert(attributes[i as usize].clone(), value);
        }
        contents.push(record);
    }

    Table {
        attributes,
        contents,
    }
}
