# dll编写
``` 
cargo new rustdll --lib


[lib]
name = "rustdll"
crate-type = ["cdylib"]


添加交叉构建
rustup target list

生成dll

// mt5 必须64位
cargo build --release --target=x86_64-pc-windows-msvc


// mt4
rustup target add i686-pc-windows-msvc
cargo build --release --target=i686-pc-windows-msvc

```

``` 
#[no_mangle]
pub extern "system" fn hello2(cstr: *const c_char) {
    // Check if the pointer is null
    if cstr.is_null() {
        println!("hello2: Received a null pointer");
        return;
    }

    // Convert the C string pointer to a Rust &CStr
    let c_str = unsafe { CStr::from_ptr(cstr) };

    // Convert the &CStr to a Rust String
    match c_str.to_str() {
        Ok(string) => {
            // http://127.0.0.1:8181/hello
            let body: String = ureq::get(format!("http://127.0.0.1:8181/{}", string).as_str())
                .call().unwrap()
                .into_string().unwrap();

            println!("{}", body);
        },
        Err(e) => println!("hello2: Invalid UTF-8 sequence: {}", e),
    }
}


#import "rustdll.dll"
int hello(int count);
void hello2(const char &text[]);;
#import

void OnStart()
  {
//---
   Print("hello:", hello(0));
   Print("hello:", hello(2));

    char path_array[256];
    StringToCharArray("Hello from MQL5", path_array, 0);  // ファイルパスをchar配列に変換
    hello2(path_array);
  }
//+------------------------------------------------------------------+
```

参考：
``` 
https://www.mql5.com/ja/articles/18
https://note.com/hosono_p/n/n3883b4c79d09
```


v2 这样写mql调用就变简单了
``` 

#import "rustdll.dll"
int hello(int count);
void hello2(string str);

#import

void OnStart()
  {
//---
   Print("hello:", hello(0));
   Print("hello:", hello(2));

    hello2("Hello from MQL5");
  }

#[no_mangle]
pub extern "system" fn hello2(text: *mut u16) {
    if let Some(string) = wide_ptr_to_string(text) {
        println!("Text: {}", string);

        let body: String = ureq::get(&format!("http://127.0.0.1:8181/{}", string))
            .call()
            .unwrap()
            .into_string()
            .unwrap();

        println!("{}", body);
    } else {
        println!("Received a null pointer");
    }
}



/// 将 `*mut u16` 指针转换为 `String`
///
/// # 参数
/// - `text`: 一个指向宽字符的指针（`*mut u16`）
///
/// # 返回
/// 如果指针不为空，则返回 `String`，否则返回 `None`
fn wide_ptr_to_string(text: *mut u16) -> Option<String> {
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

#[no_mangle]
pub extern "system" fn fn_fill_array(arr: *mut f64, arr_size: i32) {
   let resp = double_ptr_to_vec(arr, arr_size).unwrap();
    let body: String = ureq::get(&format!("http://127.0.0.1:8181/{}", json!(resp).to_string()))
        .call()
        .unwrap()
        .into_string()
        .unwrap();

    println!("{}", body);
}

#import "rustdll.dll"
int hello(int count);
void hello2(string str);
void fn_fill_array(double &arr[],int arr_size);
#import
int OnInit()
  {
//--- create timer
   EventSetTimer(60);
   Print("hello:", hello(0));
   Print("hello:", hello(2));

   hello2("Hello from MQL5" + Symbol());
   int imaHander = iMA(Symbol(), PERIOD_H1,20,0,MODE_EMA,PRICE_CLOSE);
   double arrj[];
   CopyBuffer(imaHander,0,0,10,arrj);
   fn_fill_array(arrj, ArraySize(arrj));

// int arr[10];  // 声明一个长度为 10 的数组

// 使用循环将数组填充为 1 到 10
// for(int i = 0; i < 10; i++)
//  {
//  arr[i] = i + 1;
//  }
// fn_fill_array(arr, ArraySize(arr));
//---
   return(INIT_SUCCEEDED);
  }
```