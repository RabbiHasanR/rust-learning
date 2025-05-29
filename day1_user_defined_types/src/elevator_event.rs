#![allow(dead_code)]

#[derive(Debug)]
enum Event {
    ButtonPressed(Button),
    CarArrived(Floor),
    CarDoorOpened,
    CarDoorClosed
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

fn car_door_opened() -> Event {
    Event::CarDoorOpened
}

fn car_door_closed() -> Event {
    Event::CarDoorClosed
}

fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::ButtonPressed(Button::LobbyCall(dir, floor))
}

fn car_floor_button_pressed(floor: i32) -> Event {
    Event::ButtonPressed(Button::CarFloor(floor))
}


pub fn elevator_event_example() {
    println!(
        "A ground floor paggenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );

    println!(
        "The car has arrived on the ground floor: {:?}",
        car_arrived(0)
    );
    println!(
        "The car door has opened: {:?}",
        car_door_opened()
    );

    println!(
        "A passenger has pressed the button for the 3rd floor: {:?}",
        car_floor_button_pressed(3)
    );

    println!(
        "The car door has closed: {:?}",
        car_door_closed()
    );
    println!(
        "The car has arrived on the 3rd floor: {:?}",
        car_arrived(3)
    );
}