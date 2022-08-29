enum GameEvent
{
    PlayerDied,
    KeyPressed(char),
    Click {x: f32, y: f32},
}

fn show_event(e: GameEvent)
{
    match e {
        GameEvent::PlayerDied => println!("Player died"),
        GameEvent::KeyPressed('q' | 'Q') => println!("Q key pressed"),
        GameEvent::KeyPressed(_) => println!("Some other key pressed"),
        GameEvent::Click{x, y} => println!("Clicked ({} {})", x, y),
    }
}

fn main() {
    println!("Hello, world!");
    let mut e = GameEvent::PlayerDied;
    show_event(e);
    e = GameEvent::KeyPressed('q');
    show_event(e);
    e = GameEvent::Click{x: 0.0, y: 10.0};
    show_event(e);
}
