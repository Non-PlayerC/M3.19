fn main() {
    let popt = "
        the white birds 白鸟
        --by: w.b. yeats
        would that we were, my beloved, white birds on the foam of the sea!
        we tire of the flame of the meteor, before it can fade and flee;
        and the flame of the blue star of twilight, hung low on the rim of the sky,
        has awakened in our hearts, my beloved, a sadness that may not die.
    ";
    for (i,texts) in popt.lines().enumerate() {
        if i == 1 {
            println!("<{}>",texts.trim());
        } else if texts.trim().len() > 1{
            let text: Vec<_> = texts.split(",").map(|t| t.trim()).collect();
            println!("{}",text.join(","))
        }
    }
}
