// 华氏度（Fahrenheit scale），符号℉
// 摄氏度（Centigrade），符号℃
// 关系: F = 32 + 1.8 x C

// 华氏度转摄氏度
fn fahren_to_centi(f: f32) -> f32 {
    (f - 32.0) / 1.8
}

// 摄氏度转华氏度
fn centi_fahren_to(c: f32) -> f32 {
    32.0 + 1.8 * c
}

fn main() {
    // 摄氏转华氏
    let c = 100f32;
    let f = centi_fahren_to(c);
    println!("{c}℃ is {f}℉.");

    // 华氏转摄氏
    let f = 100f32;
    let c = fahren_to_centi(f);
    println!("{f}℉ is {c}℃.");
}
