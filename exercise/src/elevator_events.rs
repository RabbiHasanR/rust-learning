#[derive(Debug)]
enum Event {
    ButtonPressed(Button),
    CarArrived(Floor),
    CarDoorOpend,
    CarDoorClosed,
}


type Floor = i32;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
enum Button {
    LobbyCall(Direction, Floor),
    CarFloor(Floor),
}

fn car_arrived(floor: Floor) -> Event {
    Event::CarArrived(floor)
}

fn car_door_opend() -> Event {
    Event::CarDoorOpend
}

fn car_door_closed() -> Event {
    Event::CarDoorClosed
}


fn lobby_call_button_pressed(floor: Floor, dir: Direction) -> Event {
    Event::ButtonPressed(Button::LobbyCall(dir, floor))
}

fn car_floor_button_pressed(floor: Floor) -> Event {
    Event::ButtonPressed(Button::CarFloor(floor))
}


pub fn elevator_event() {
    println!("A ground floor passender has pressed the up button: {:?}", lobby_call_button_pressed(0, Direction::Up));
    println!("The car has arrived on the ground floor: {:?}", car_arrived(0));
    println!("The car door opened: {:?}", car_door_opend());
    println!("A passenger has pressed the 3rd floor button: {:?}", car_floor_button_pressed(3));
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}