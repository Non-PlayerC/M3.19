// fn main() {
//     let a = 0.1_f32+0.2_f32;
//     let b = 0.3_f32;
//     let c = 1e-10; // 设置一个非常小的误差值
//     assert!((a - b).abs() < c); // 判断两个浮点数是否相等
// } 
// --> 8


// fn main() {
//     let mut sum = 0;
//     for i in -3..3 {
//         sum += i
//     }

//     assert_eq!(sum, -3);

//     for c in 'a'..='z' {
//         println!("{}",c);
//     }
// }




// 填空
// use
//  std::ops::{Range, RangeInclusive};
// fn main() {
//     assert_eq!((1..5), Range{ start: 1, end: 5 });
//     assert_eq!((1..=5), RangeInclusive::new(1, 5));
// }


// fn main() {
//     assert!((0.1+ 0.2 - 0.3).abs() < 0.001);
// }



fn main() {
    let mut sum = 0;
    for i in -3..3 {//左闭右开
        sum += i
    }

    assert!(sum == -3);

    for c in 'a'..='z' {
        println!("{}",c as u8);
    }
}
