use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::slice;

/// 将 `*mut u16` 指针转换为 `String`
///
/// # 参数
/// - `text`: 一个指向宽字符的指针（`*mut u16`）
///
/// # 返回
/// 如果指针不为空，则返回 `String`，否则返回 `None`
#[warn(dead_code)]
pub fn wide_ptr_to_string(text: *mut u16) -> Option<String> {
    if text.is_null() {
        return None;
    }

    unsafe {
        // 计算字符串长度
        let len = wcslen(text);

        // 将指针转换为切片
        let slice = slice::from_raw_parts(text, len);

        // 将 u16 数组转换为 OsString，然后转换为 Rust String
        let os_string = OsString::from_wide(slice);
        Some(os_string.to_string_lossy().into_owned())
    }
}

// 计算 wchar_t* 字符串的长度
fn wcslen(ptr: *const u16) -> usize {
    let mut len = 0;
    unsafe {
        while *ptr.add(len) != 0 {
            len += 1;
        }
    }
    len
}

/// 将 `*mut i32` 指针转换为 `Option<Vec<i32>>`
///
/// # 参数
/// - `arr`: 一个指向 `i32` 数组的指针
/// - `arr_size`: 数组的大小
///
/// # 返回
/// 如果指针不为空且大小有效，则返回 `Option<Vec<i32>>`，否则返回 `None`
#[warn(dead_code)]
pub fn int_ptr_to_vec(arr: *mut i32, arr_size: i32) -> Option<Vec<i32>> {
    if arr.is_null() || arr_size < 1 {
        return None;
    }

    unsafe {
        Some(slice::from_raw_parts(arr, arr_size as usize).to_vec())
    }
}

/// 将 `*mut f64` 指针转换为 `Option<Vec<f64>>`
///
/// # 参数
/// - `arr`: 一个指向 `f64` 数组的指针
/// - `arr_size`: 数组的大小
///
/// # 返回
/// 如果指针不为空且大小有效，则返回 `Option<Vec<f64>>`，否则返回 `None`
#[warn(dead_code)]
pub fn double_ptr_to_vec(arr: *mut f64, arr_size: i32) -> Option<Vec<f64>> {
    if arr.is_null() || arr_size < 1 {
        return None;
    }

    unsafe {
        Some(slice::from_raw_parts(arr, arr_size as usize).to_vec())
    }
}
