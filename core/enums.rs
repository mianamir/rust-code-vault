/// Demonstrates working with enums in Rust.
pub fn run() {
    // Define a movement enum
    enum Movement {
        Up,
        Down,
        Left,
        Right,
    }

    // Function to move an avatar based on the provided movement
    fn move_avatar(m: Movement) {
        match m {
            Movement::Up => println!("Moving up"),
            Movement::Down => println!("Moving down"),
            Movement::Left => println!("Moving left"),
            Movement::Right => println!("Moving right"),
        }
    }

    // Move avatars in different directions
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}

