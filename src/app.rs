

/// Every part on the canvas will be made out of rectangles
struct GameElement {
    x: u16,
    y: u16,
    width: u8,
    height: u8,
}

struct App {
    snake_head: GameElement,
    snake_body: Vec<GameElement>,
}