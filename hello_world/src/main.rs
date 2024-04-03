fn greet_world() {
    let chinese = "你好，世界！";
    let english = "hello, world!";
    let languages = [chinese, english];
    for lang in languages.iter() {
        println!("{}", &lang);
    }
}
fn main() {
    greet_world();
}
