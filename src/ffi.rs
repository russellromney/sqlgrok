//! C FFI bindings for sqlgrok.
//!
//! This module exposes a C-compatible API so the library can be consumed
//! from C, C++, or any language that supports the C ABI.
//!
//! # Memory management
//!
//! Every `*mut c_char` returned by a function in this module **must** be freed
//! by calling [`sqlgrok_free`]. Failing to do so will leak memory.
//!
//! The `sqlgrok_*` symbols are the public ABI. The older `sqlglot_*` symbols
//! remain as compatibility shims while early consumers migrate.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::slice;

use crate::dialects::Dialect;

// ── helpers ──────────────────────────────────────────────────────────────

/// Convert a nullable C string to an `Option<&str>`.
/// Returns `None` when the pointer is null or the bytes are not valid UTF-8.
unsafe fn cstr_to_option(p: *const c_char) -> Option<&'static str> {
    if p.is_null() {
        return None;
    }
    unsafe { CStr::from_ptr(p) }.to_str().ok()
}

/// Resolve a C dialect string to a `Dialect` enum, falling back to `Ansi`.
fn resolve_dialect(name: Option<&str>) -> Dialect {
    name.and_then(Dialect::from_str).unwrap_or(Dialect::Ansi)
}

/// Return a C-owned string, or null on encoding failure.
fn to_c_string(s: String) -> *mut c_char {
    CString::new(s)
        .map(CString::into_raw)
        .unwrap_or(ptr::null_mut())
}

// ── public C API ─────────────────────────────────────────────────────────

unsafe fn parse_impl(sql: *const c_char, dialect: *const c_char) -> *mut c_char {
    let sql_str = match unsafe { cstr_to_option(sql) } {
        Some(s) => s,
        None => return ptr::null_mut(),
    };
    let dialect_enum = resolve_dialect(unsafe { cstr_to_option(dialect) });

    match crate::parse(sql_str, dialect_enum) {
        Ok(ast) => match serde_json::to_string(&ast) {
            Ok(json) => to_c_string(json),
            Err(_) => ptr::null_mut(),
        },
        Err(_) => ptr::null_mut(),
    }
}

unsafe fn transpile_impl(
    sql: *const c_char,
    from_dialect: *const c_char,
    to_dialect: *const c_char,
) -> *mut c_char {
    let sql_str = match unsafe { cstr_to_option(sql) } {
        Some(s) => s,
        None => return ptr::null_mut(),
    };
    let from = resolve_dialect(unsafe { cstr_to_option(from_dialect) });
    let to = resolve_dialect(unsafe { cstr_to_option(to_dialect) });

    match crate::transpile(sql_str, from, to) {
        Ok(result) => to_c_string(result),
        Err(_) => ptr::null_mut(),
    }
}

unsafe fn generate_impl(ast_json: *const c_char, dialect: *const c_char) -> *mut c_char {
    let json_str = match unsafe { cstr_to_option(ast_json) } {
        Some(s) => s,
        None => return ptr::null_mut(),
    };
    let dialect_enum = resolve_dialect(unsafe { cstr_to_option(dialect) });

    match serde_json::from_str::<crate::ast::Statement>(json_str) {
        Ok(ast) => to_c_string(crate::generate(&ast, dialect_enum)),
        Err(_) => ptr::null_mut(),
    }
}

unsafe fn free_impl(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(unsafe { CString::from_raw(ptr) });
    }
}

unsafe fn transpile_into_impl(
    sql: *const c_char,
    from_dialect: *const c_char,
    to_dialect: *const c_char,
    buffer: *mut c_char,
    buffer_len: usize,
) -> isize {
    let sql_str = match unsafe { cstr_to_option(sql) } {
        Some(s) => s,
        None => return -1,
    };
    let from = resolve_dialect(unsafe { cstr_to_option(from_dialect) });
    let to = resolve_dialect(unsafe { cstr_to_option(to_dialect) });

    let result = match crate::transpile(sql_str, from, to) {
        Ok(result) => result,
        Err(_) => return -1,
    };
    let required = result.len();
    if buffer.is_null() || buffer_len == 0 {
        return required.try_into().unwrap_or(isize::MAX);
    }
    if buffer_len <= required {
        return required.try_into().unwrap_or(isize::MAX);
    }

    let target = unsafe { slice::from_raw_parts_mut(buffer.cast::<u8>(), buffer_len) };
    target[..required].copy_from_slice(result.as_bytes());
    target[required] = 0;
    required.try_into().unwrap_or(isize::MAX)
}

/// Parse a SQL string and return its AST serialised as JSON.
///
/// * `sql`     – null-terminated SQL string (required).
/// * `dialect` – null-terminated dialect name, e.g. `"postgres"`. Pass `NULL`
///   for ANSI SQL.
///
/// Returns a heap-allocated JSON string on success, or `NULL` on failure.
/// The caller **must** free a non-null return value with [`sqlgrok_free`].
///
/// # Safety
///
/// `sql` must be a valid null-terminated C string. `dialect` may be null or a
/// valid null-terminated C string. The returned pointer must be freed with
/// [`sqlgrok_free`] when non-null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlgrok_parse(sql: *const c_char, dialect: *const c_char) -> *mut c_char {
    unsafe { parse_impl(sql, dialect) }
}

/// Transpile a single SQL statement from one dialect to another.
///
/// * `sql`          – null-terminated SQL string (required).
/// * `from_dialect` – source dialect name, or `NULL` for ANSI.
/// * `to_dialect`   – target dialect name, or `NULL` for ANSI.
///
/// Returns a heap-allocated SQL string on success, or `NULL` on failure.
/// The caller **must** free a non-null return value with [`sqlgrok_free`].
///
/// # Safety
///
/// `sql` must be a valid null-terminated C string. `from_dialect` and
/// `to_dialect` may be null or valid null-terminated C strings. The returned
/// pointer must be freed with [`sqlgrok_free`] when non-null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlgrok_transpile(
    sql: *const c_char,
    from_dialect: *const c_char,
    to_dialect: *const c_char,
) -> *mut c_char {
    unsafe { transpile_impl(sql, from_dialect, to_dialect) }
}

/// Transpile into a caller-owned buffer and return the required byte length.
///
/// The return value is the number of UTF-8 bytes required for the SQL output,
/// excluding the trailing NUL. Pass `NULL` or a zero-length buffer to query the
/// required length. When `buffer_len` is greater than the returned length,
/// sqlgrok writes the output followed by a NUL terminator. Returns `-1` on
/// parse/transpile failure or invalid input.
///
/// This API exists for bindings that want to avoid allocating and freeing an
/// owned C string for every call.
///
/// # Safety
///
/// `sql` must be a valid null-terminated C string. `from_dialect` and
/// `to_dialect` may be null or valid null-terminated C strings. `buffer` may be
/// null only when `buffer_len` is zero; otherwise it must point to writable
/// memory of at least `buffer_len` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlgrok_transpile_into(
    sql: *const c_char,
    from_dialect: *const c_char,
    to_dialect: *const c_char,
    buffer: *mut c_char,
    buffer_len: usize,
) -> isize {
    unsafe { transpile_into_impl(sql, from_dialect, to_dialect, buffer, buffer_len) }
}

/// Generate SQL from a JSON-serialised AST for the given dialect.
///
/// * `ast_json` – null-terminated JSON string of a serialised `Statement`.
/// * `dialect`  – target dialect name, or `NULL` for ANSI.
///
/// Returns a heap-allocated SQL string on success, or `NULL` on failure.
/// The caller **must** free a non-null return value with [`sqlgrok_free`].
///
/// # Safety
///
/// `ast_json` must be a valid null-terminated C string. `dialect` may be null
/// or a valid null-terminated C string. The returned pointer must be freed with
/// [`sqlgrok_free`] when non-null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlgrok_generate(
    ast_json: *const c_char,
    dialect: *const c_char,
) -> *mut c_char {
    unsafe { generate_impl(ast_json, dialect) }
}

/// Return the library version as a static null-terminated string.
///
/// The returned pointer **must not** be freed — it points to static memory.
#[unsafe(no_mangle)]
pub extern "C" fn sqlgrok_version() -> *const c_char {
    // The trailing \0 makes this a valid C string.
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as *const c_char
}

/// Free a string previously returned by any `sqlgrok_*` function.
///
/// Passing `NULL` is safe and results in a no-op.
///
/// # Safety
///
/// `ptr` must be null or a pointer previously returned by this library that has
/// not already been freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlgrok_free(ptr: *mut c_char) {
    unsafe { free_impl(ptr) };
}

/// Compatibility alias for [`sqlgrok_parse`].
///
/// # Safety
///
/// Same requirements as [`sqlgrok_parse`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlglot_parse(sql: *const c_char, dialect: *const c_char) -> *mut c_char {
    unsafe { parse_impl(sql, dialect) }
}

/// Compatibility alias for [`sqlgrok_transpile`].
///
/// # Safety
///
/// Same requirements as [`sqlgrok_transpile`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlglot_transpile(
    sql: *const c_char,
    from_dialect: *const c_char,
    to_dialect: *const c_char,
) -> *mut c_char {
    unsafe { transpile_impl(sql, from_dialect, to_dialect) }
}

/// Compatibility alias for [`sqlgrok_transpile_into`].
///
/// # Safety
///
/// Same requirements as [`sqlgrok_transpile_into`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlglot_transpile_into(
    sql: *const c_char,
    from_dialect: *const c_char,
    to_dialect: *const c_char,
    buffer: *mut c_char,
    buffer_len: usize,
) -> isize {
    unsafe { transpile_into_impl(sql, from_dialect, to_dialect, buffer, buffer_len) }
}

/// Compatibility alias for [`sqlgrok_generate`].
///
/// # Safety
///
/// Same requirements as [`sqlgrok_generate`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlglot_generate(
    ast_json: *const c_char,
    dialect: *const c_char,
) -> *mut c_char {
    unsafe { generate_impl(ast_json, dialect) }
}

/// Compatibility alias for [`sqlgrok_version`].
#[unsafe(no_mangle)]
pub extern "C" fn sqlglot_version() -> *const c_char {
    sqlgrok_version()
}

/// Compatibility alias for [`sqlgrok_free`].
///
/// # Safety
///
/// Same requirements as [`sqlgrok_free`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlglot_free(ptr: *mut c_char) {
    unsafe { free_impl(ptr) };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Dialect;
    use std::ffi::{CStr, CString};

    fn cstring(value: &str) -> CString {
        CString::new(value).unwrap()
    }

    #[test]
    fn transpile_into_reports_required_length_without_buffer() {
        let sql = cstring("SELECT IFNULL(a, 0) FROM t");
        let read = cstring("mysql");
        let write = cstring("sqlite");
        let expected = crate::transpile(sql.to_str().unwrap(), Dialect::Mysql, Dialect::Sqlite)
            .unwrap()
            .len();

        let required = unsafe {
            sqlgrok_transpile_into(
                sql.as_ptr(),
                read.as_ptr(),
                write.as_ptr(),
                ptr::null_mut(),
                0,
            )
        };

        assert_eq!(required, expected as isize);
    }

    #[test]
    fn transpile_into_does_not_write_to_too_small_buffer() {
        let sql = cstring("SELECT IFNULL(a, 0) FROM t");
        let read = cstring("mysql");
        let write = cstring("sqlite");
        let expected = crate::transpile(sql.to_str().unwrap(), Dialect::Mysql, Dialect::Sqlite)
            .unwrap()
            .len();
        let mut buffer = vec![b'X' as c_char; expected];

        let required = unsafe {
            sqlgrok_transpile_into(
                sql.as_ptr(),
                read.as_ptr(),
                write.as_ptr(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };

        assert_eq!(required, expected as isize);
        assert!(buffer.iter().all(|byte| *byte == b'X' as c_char));
    }

    #[test]
    fn transpile_into_writes_nul_terminated_output() {
        let sql = cstring("SELECT IFNULL(a, 0) FROM t");
        let read = cstring("mysql");
        let write = cstring("sqlite");
        let expected =
            crate::transpile(sql.to_str().unwrap(), Dialect::Mysql, Dialect::Sqlite).unwrap();
        let mut buffer = vec![0 as c_char; expected.len() + 1];

        let written = unsafe {
            sqlgrok_transpile_into(
                sql.as_ptr(),
                read.as_ptr(),
                write.as_ptr(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };

        assert_eq!(written, expected.len() as isize);
        let output = unsafe { CStr::from_ptr(buffer.as_ptr()) };
        assert_eq!(output.to_str().unwrap(), expected);
    }

    #[test]
    fn transpile_into_returns_error_for_null_sql() {
        let read = cstring("mysql");
        let write = cstring("sqlite");
        let mut buffer = [0 as c_char; 16];

        let result = unsafe {
            sqlgrok_transpile_into(
                ptr::null(),
                read.as_ptr(),
                write.as_ptr(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };

        assert_eq!(result, -1);
    }

    #[test]
    fn legacy_transpile_into_alias_matches_public_symbol() {
        let sql = cstring("SELECT IFNULL(a, 0) FROM t");
        let read = cstring("mysql");
        let write = cstring("sqlite");
        let mut public_buffer = vec![0 as c_char; 128];
        let mut legacy_buffer = vec![0 as c_char; 128];

        let public_written = unsafe {
            sqlgrok_transpile_into(
                sql.as_ptr(),
                read.as_ptr(),
                write.as_ptr(),
                public_buffer.as_mut_ptr(),
                public_buffer.len(),
            )
        };
        let legacy_written = unsafe {
            sqlglot_transpile_into(
                sql.as_ptr(),
                read.as_ptr(),
                write.as_ptr(),
                legacy_buffer.as_mut_ptr(),
                legacy_buffer.len(),
            )
        };

        assert_eq!(legacy_written, public_written);
        let public_output = unsafe { CStr::from_ptr(public_buffer.as_ptr()) };
        let legacy_output = unsafe { CStr::from_ptr(legacy_buffer.as_ptr()) };
        assert_eq!(legacy_output, public_output);
    }
}
