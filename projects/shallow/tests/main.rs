use shallow::CharacterShallow;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    const TEXT10: &str = "1234567890";
    const TEXT21: &str = "1234567890_1234567890";
    const TEXT27: &str = "1234567890_1234567890_12345";
    let sb = CharacterShallow::new(21, 5);
    assert_eq!(sb.build_cow(TEXT10), TEXT10);
    assert_eq!(sb.build_cow(TEXT21), "1234567890_1234567890");
    assert_eq!(sb.build_cow(TEXT27), "123456789 <...> 12345");
    let sb = sb.with_shallow_text("..."); // replace shallow text
    assert_eq!(sb.build_cow(TEXT27), "1234567890_12...12345");
    let sb = sb.with_end_reserved(0); // cancel end reserved
    assert_eq!(sb.build_cow(TEXT27), "1234567890_1234567...");
}
