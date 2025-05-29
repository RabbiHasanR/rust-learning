#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,
    Run(Direction),
    Teleport { x: u32, y: u32 },
}

pub fn enum_example() {
    let dir = Direction::Left;
    let player_move: PlayerMove = PlayerMove::Run(dir);
    println!("On this turn: {player_move:?}");
    let player_move: PlayerMove = PlayerMove::Teleport { x: 10, y: 20 };
    println!("On this turn: {player_move:?}");
    let player_move: PlayerMove = PlayerMove::Pass;
    println!("On this turn: {player_move:?}");
}


#[repr(u32)]
enum Bar {
    A,
    B = 10000,
    C,
}

pub fn enum_repr() {
    println!("A: {}", Bar::A as u32);
    println!("B: {}", Bar::B as u32); 
    println!("c: {}", Bar::C as u32);
}



use std::mem::transmute;

macro_rules! dbg_bits {
    ($e:expr, $bit_type:ty) => {
        println!("- {}: {:#x}", stringify!($e), unsafe { transmute::<_, $bit_type>($e) });
    };
}

pub fn enum_bits() {
    println!("bool:");
    dbg_bits!(false, u8);
    dbg_bits!(true, u8);

    println!("Option<bool>:");
    dbg_bits!(None::<bool>, u8);
    dbg_bits!(Some(false), u8);
    dbg_bits!(Some(true), u8);

    println!("Option<Option<bool>>:");
    dbg_bits!(Some(Some(false)), u8);
    dbg_bits!(Some(Some(true)), u8);
    dbg_bits!(Some(None::<bool>), u8);
    dbg_bits!(None::<Option<bool>>, u8);

    println!("Option<&i32>:");
    dbg_bits!(None::<&i32>, usize);
    dbg_bits!(Some(&0i32), usize);
}