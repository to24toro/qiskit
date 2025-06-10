use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use qiskit_circuit::circuit_data::CircuitData;
use qiskit_qasm3::exporter::Exporter;

/// # Safety
/// Call Rust from C. Be careful for handling the pointer.
#[no_mangle]
pub unsafe extern "C" fn exporter_new(
    includes: *const *const c_char,
    includes_len: usize,
    basis: *const *const c_char,
    basis_len: usize,
    disable_constants: bool,
    allow_aliasing: bool,
    indent: *const c_char,
) -> *mut Exporter {
    unsafe {
        let includes: Vec<String> = (0..includes_len)
            .map(|i| {
                CStr::from_ptr(*includes.add(i))
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();

        let basis_gates: Vec<String> = (0..basis_len)
            .map(|i| CStr::from_ptr(*basis.add(i)).to_string_lossy().into_owned())
            .collect();

        let indent = CStr::from_ptr(indent).to_string_lossy().into_owned();

        let exporter = Exporter::new(
            includes,
            basis_gates,
            disable_constants,
            allow_aliasing,
            indent,
        );
        Box::into_raw(Box::new(exporter)) // ← *mut Exporter を返す
    }
}

/// # Safety
/// Call Rust from C. Be careful for handling the pointer.
#[no_mangle]
pub unsafe extern "C" fn exporter_free(ptr: *mut Exporter) {
    if !ptr.is_null() {
        unsafe {
            drop(Box::from_raw(ptr));
        }
    }
}

/// # Safety
/// Call Rust from C. Be careful for handling the pointer.
#[no_mangle]
pub unsafe extern "C" fn exporter_dumps(
    exporter: *const Exporter,
    circuit: *const CircuitData,
    islayout: bool,
) -> *mut c_char {
    let exporter = unsafe { &*exporter };
    let circuit = unsafe { &*circuit };

    match exporter.dumps(circuit, islayout) {
        Ok(output) => CString::new(output).unwrap().into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// # Safety
/// Call Rust from C. Be careful for handling the pointer.
#[no_mangle]
pub unsafe extern "C" fn exporter_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            drop(CString::from_raw(s));
        }
    }
}
