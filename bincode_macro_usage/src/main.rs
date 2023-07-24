use bincode::{error, Decode, Encode};
use bincode_macro::Serde;

#[derive(Serde, Encode, Decode, PartialEq, Debug)]
pub struct Entity {
    pub x: u16,
    pub y: u32,
}

#[derive(Serde, Encode, Decode, PartialEq, Debug)]
struct World(Vec<Entity>);

#[derive(Serde, Encode, Decode, PartialEq, Debug)]
struct Worlds(Vec<World>);

#[derive(Serde, Encode, Decode, PartialEq, Debug)]
struct TestTupleStruct(u32, u32, u32);

#[derive(Serde, Encode, Decode, PartialEq, Debug)]
enum TestEnum {
    Foo,
    Bar { name: u32 },
    Baz(u32, u32, u32),
}

fn main() {
    let entity = TestEnum::Bar { name: 5u32 };
    let encoded: Vec<u8> = entity.pack().unwrap();
    println!("{:?} {}", encoded, encoded.len());
    let (decoded, len): (TestEnum, usize) = entity.unpack(&encoded).unwrap();
    println!("{:?}, {}\n", decoded, len);

    let entity = TestTupleStruct(1, 2, 3);
    let encoded: Vec<u8> = entity.pack().unwrap();
    println!("{:?} {}", encoded, encoded.len());
    let (decoded, len): (TestTupleStruct, usize) = entity.unpack(&encoded).unwrap();
    println!("{:?}, {}\n", decoded, len);

    let mut entity = Entity { x: 1, y: 4 };
    let encoded: Vec<u8> = entity.pack().unwrap();
    println!("{:?} {}", encoded, encoded.len());
    let (decoded, len): (Entity, usize) = entity.unpack(&encoded).unwrap();
    println!("{:?}, {}\n", decoded, len);

    entity.x = 2;
    let encoded: Vec<u8> = entity.pack().unwrap();
    println!("{:?} {}", encoded, encoded.len());
    let (decoded, _len): (Entity, usize) = entity.unpack(&encoded).unwrap();
    println!("{:?}\n", decoded);

    let world = World(vec![Entity { x: 1, y: 4 }, Entity { x: 10, y: 2000 }]);
    let encoded: Vec<u8> = world.pack().unwrap();
    println!("{:?} {}", encoded, encoded.len());
    let (decoded, len): (World, usize) = world.unpack(&encoded).unwrap();
    println!("{:?}, {}\n", decoded, len);

    let workd1 = World(vec![Entity { x: 2, y: 5 }, Entity { x: 6, y: 7 }]);
    let world = Worlds(vec![world, workd1]);
    let encoded: Vec<u8> = world.pack().unwrap();
    println!("{:?} {}", encoded, encoded.len());
    let (decoded, len): (Worlds, usize) = world.unpack(&encoded).unwrap();
    println!("{:?}, {}", decoded, len);
}
