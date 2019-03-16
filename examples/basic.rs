use dummy::*;

fn main() {
    // generate random u8
    println!("u8 {} in [MIN, MAX)", u8::dummy(ANY));

    // generate random u8 using range
    println!("u8 {} in [3,7)", u8::dummy(3..7));
    println!("u8 {} in [3,7]", u8::dummy(3..=7));
    println!("u8 {} in [3, MAX]", u8::dummy(3..));
    println!("u8 {} in [MIN, 7)", u8::dummy(..7));
    println!("u8 {} in [MIN, 7]", u8::dummy(..=7));
    println!("u8 {} in [MIN, MAX]", u8::dummy(..));

    // to reuse sampler `Uniform` for value generation
    let sampler = distributions::Uniform::new_inclusive(1, 10);
    for _ in 0..10 {
        let v = u8::dummy_ref(&sampler);
        println!("sample value {}", v);
    }

    // generate random Vec<u8> with fixed length
    let v1 = <Vec<u8>>::dummy((ANY, 5));
    let v2 = dummy::vec![u8; 5];
    println!("fixed length vec {:?}", v1);
    println!("fixed length vec {:?}", v2);

    // generate random Vec<u8> with random length
    let v1 = <Vec<u8>>::dummy((ANY, 2..5));
    let v2 = dummy::vec![u8; 2..5];
    println!("random length vec {:?}", v1);
    println!("random length vec {:?}", v2);

    // generate random Vec<u8> with random length and value config
    let v1 = <Vec<u8>>::dummy((1..=10, 2..4));
    let v2 = dummy::vec![u8 as 1..=10; 2..4];
    println!("fixed length and element config vec {:?}", v1);
    println!("fixed length and element config vec {:?}", v2);

    
    // generate nested vec
    let v1 = < Vec<Vec<u8>>>::dummy(((ANY, 1..3), 5));
    let v2 = dummy::vec![u8; 5, 1..3];
    println!("random nested vec {:?}", v1);
    println!("random nested vec {:?}", v2);

    // generated fixed length nested vec [[[u8;2];3];4] with value using samper
    let v3 = dummy::vec![u8 as sampler; 4, 3, 2];
    println!("random nested vec {:?}", v3);
}
