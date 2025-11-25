// Паттерн Visitor: позволяет добавить новые операции к объектам
// без изменения их классов, определяя новую операцию в классе Visitor.
// Полезен для добавления операций к иерархиям классов.
// Пример: фигуры и операции над ними (рисование, расчет площади).

/// Трейт для посетителя.
pub trait Visitor {
    fn visit_circle(&self, circle: &Circle);
    fn visit_square(&self, square: &Square);
}

/// Трейт для фигуры.
pub trait Shape {
    fn accept(&self, visitor: &dyn Visitor);
}

/// Конкретная фигура - круг.
pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Circle { radius }
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl Shape for Circle {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_circle(self);
    }
}

/// Конкретная фигура - квадрат.
pub struct Square {
    side: f64,
}

impl Square {
    pub fn new(side: f64) -> Self {
        Square { side }
    }

    pub fn get_side(&self) -> f64 {
        self.side
    }
}

impl Shape for Square {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_square(self);
    }
}

/// Конкретный посетитель - рисование.
pub struct DrawVisitor;

impl Visitor for DrawVisitor {
    fn visit_circle(&self, circle: &Circle) {
        println!("Рисуем круг с радиусом {}", circle.get_radius());
    }

    fn visit_square(&self, square: &Square) {
        println!("Рисуем квадрат со стороной {}", square.get_side());
    }
}

/// Конкретный посетитель - расчет площади.
pub struct AreaVisitor;

impl Visitor for AreaVisitor {
    fn visit_circle(&self, circle: &Circle) {
        let area = std::f64::consts::PI * circle.get_radius() * circle.get_radius();
        println!("Площадь круга: {:.2}", area);
    }

    fn visit_square(&self, square: &Square) {
        let area = square.get_side() * square.get_side();
        println!("Площадь квадрата: {:.2}", area);
    }
}

/// Тест для паттерна Visitor.
#[test]
fn test_visitor() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle::new(5.0)),
        Box::new(Square::new(4.0)),
    ];

    let draw_visitor = DrawVisitor;
    let area_visitor = AreaVisitor;

    for shape in &shapes {
        shape.accept(&draw_visitor);
        shape.accept(&area_visitor);
    }
}