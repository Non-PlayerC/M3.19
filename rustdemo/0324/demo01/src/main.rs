fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let heart = "see you tomorrow!";
    let chinese = "世界，您好";
    let english  = "Hello,world";
    let regions = [southern_germany,chinese,english,heart];
    for region in regions.iter() {
        // 这个 iter() 应该是返回切片吧
        println!("{}",&region);
    }
}

fn main() {
    greet_world()
}