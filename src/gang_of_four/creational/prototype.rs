// Паттерн Prototype: позволяет создавать новые объекты путем клонирования
// существующих прототипов, вместо создания через конструктор.
// Это полезно, когда создание объекта дорого или сложно.

/// Трейт для объектов, поддерживающих клонирование по паттерну Prototype.
pub trait Prototype {
    fn clone_prototype(&self) -> Self;
}

/// Структура Shape, представляющая геометрическую фигуру.
/// Реализует Clone для простого клонирования.
#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    pub shape_type: String,
    pub data: Vec<f64>,
}

/// Реализация паттерна Prototype для Shape.
/// В данном случае использует стандартный Clone.
impl Prototype for Shape {
    fn clone_prototype(&self) -> Self {
        self.clone()
    }
}

/// Тест для проверки паттерна Prototype.
/// Создает прототип и клонирует его, проверяя равенство данных и различие экземпляров.
#[test]
fn test_prototype() {
    let prototype = Shape {
        shape_type: "Circle".to_string(),
        data: vec![5.0], // radius
    };

    let clone = prototype.clone_prototype();

    assert_eq!(prototype, clone); // Данные одинаковые
    assert_ne!(&prototype as *const _, &clone as *const _); // Разные экземпляры в памяти
}