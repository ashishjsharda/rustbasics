
fn main()
{
    let mut map=std::collections::HashMap::new();
    map.insert("blue", 10);
    map.insert("yellow", 50);
    map.insert("red", 100);
    map.insert("green", 200);
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

}
