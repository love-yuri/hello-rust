// 标记为未使用
#![allow(unused)]

fn greet_world() {
    let chinese = String::from("你好，世界!");
    let english = String::from("Hello, world!");
    let array = [chinese, english];
    
    // 所有权仍在，因为iter创建引用迭代器
    for text in array.iter() {
        println!("{}", text);
    };
    
    // 所有权转移，迭代后消失
    for text in array.iter() {
        println!("{}", text);
    }
}

fn string_demo() {
    // 多行字符串
    let penguin_data = "
       common name,length (cm)
       Little penguin,33
       Yellow-eyed penguin,65
       Float penguin,60
       Invalid,data
   ";
   
    let records = penguin_data.lines();
    
    for (index, record) in records.enumerate() {
        if record.trim().len() == 0 {
            continue;
        }
        
        // 如果record包含逗号，则分割为字段
        // Vector <_>表示类型自动推导
        let fields: Vec<_> = record
          .split(',')
          .map(|field: &str| field.trim())
          .collect();
        
        // 条件编译
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }
        
        let name = fields[0];
        // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
        //
        // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
        //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
        //   2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
        //
        // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
        if let Ok(length) = fields[1].parse::<f64>() {
            // 输出到标准输出
            println!("{}, {}cm", name, length);
        }
    }
}

/// 类型测试demo
fn type_demo() {
    let a = 2.1_f32;
    println!("{:2} to int {}", a, a as i32);
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    println!("{}", v);
}

fn main() {
    // greet_world();
    string_demo();
    type_demo();
}
