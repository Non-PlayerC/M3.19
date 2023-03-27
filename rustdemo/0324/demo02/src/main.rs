fn main() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();
    // 返回的是 line 类型

    for(i,record) in records.enumerate() {
        // 返回引用的指针 &
        // println!("i is {},record is {}",i,record);
        // i is 0,record is common name,length (cm)

        if i==0 || record.trim().len() ==0 {
            // trim 去掉字符前后可能的空格,保证字符前后不含空值
            continue;// 跳出循环
        }

        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {// debug_assertions - 若没有开启编译优化时就会成立。

            // if cfg!(target_os = "windows") {
            //     // windows系统要执行的代码段
            //   } else if cfg!(target_os = "linux") {
            //     // linux系统要执行的代码段
            //   }

            // 输出到标准错误输出
          eprintln!("debug: {:?} -> {:?}",
                 record, fields);
        }

        let name = fields[0];

        if let Ok(length) = fields[1].parse::<f32> () {
            // .parse --< 将字符切片解析为另一种模式 "4"-> 4
            println!("{}, {}cm", name, length);
        }
    } 
}
