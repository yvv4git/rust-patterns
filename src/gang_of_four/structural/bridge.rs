// Паттерн Bridge: отделяет абстракцию от реализации, позволяя изменять их независимо.
// Полезен для избежания жесткой связи между классами.
// Пример: рисование фигур с разными цветами.

/// Трейт для реализации (цвет).
pub trait Color {
    fn apply_color(&self) -> String;
}

/// Конкретная реализация - красный цвет.
pub struct RedColor;

impl Color for RedColor {
    fn apply_color(&self) -> String {
        "красный".to_string()
    }
}

/// Конкретная реализация - синий цвет.
pub struct BlueColor;

impl Color for BlueColor {
    fn apply_color(&self) -> String {
        "синий".to_string()
    }
}

/// Трейт для абстракции (фигура).
pub trait Shape {
    fn draw(&self);
}

/// Конкретная абстракция - круг.
pub struct Circle {
    color: Box<dyn Color>,
}

impl Circle {
    pub fn new(color: Box<dyn Color>) -> Self {
        Circle { color }
    }
}

impl Shape for Circle {
    fn draw(&self) {
        println!("Рисуем круг {} цветом", self.color.apply_color());
    }
}

/// Конкретная абстракция - квадрат.
pub struct Square {
    color: Box<dyn Color>,
}

impl Square {
    pub fn new(color: Box<dyn Color>) -> Self {
        Square { color }
    }
}

impl Shape for Square {
    fn draw(&self) {
        println!("Рисуем квадрат {} цветом", self.color.apply_color());
    }
}

/// Тест для паттерна Bridge.
#[test]
fn test_bridge() {
    let red_circle = Circle::new(Box::new(RedColor));
    let blue_square = Square::new(Box::new(BlueColor));
    let blue_circle = Circle::new(Box::new(BlueColor));

    red_circle.draw();
    blue_square.draw();
    blue_circle.draw();
}