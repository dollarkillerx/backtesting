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