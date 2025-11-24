// Паттерн Decorator: динамически добавляет поведение объектам, оборачивая их.
// Полезен для расширения функциональности без изменения кода.
// Пример: напитки с добавками.

/// Трейт для компонента (напитка).
pub trait Beverage {
    fn cost(&self) -> f64;
    fn description(&self) -> String;
}

/// Базовый напиток - кофе.
pub struct Coffee;

impl Beverage for Coffee {
    fn cost(&self) -> f64 {
        5.0
    }

    fn description(&self) -> String {
        "Кофе".to_string()
    }
}

/// Декоратор для добавок.
pub struct BeverageDecorator<T: Beverage> {
    beverage: T,
}

impl<T: Beverage> BeverageDecorator<T> {
    pub fn new(beverage: T) -> Self {
        BeverageDecorator { beverage }
    }
}

/// Конкретный декоратор - молоко.
pub struct Milk<T: Beverage>(BeverageDecorator<T>);

impl<T: Beverage> Milk<T> {
    pub fn new(beverage: T) -> Self {
        Milk(BeverageDecorator::new(beverage))
    }
}

impl<T: Beverage> Beverage for Milk<T> {
    fn cost(&self) -> f64 {
        self.0.beverage.cost() + 1.5
    }

    fn description(&self) -> String {
        format!("{} с молоком", self.0.beverage.description())
    }
}

/// Конкретный декоратор - сахар.
pub struct Sugar<T: Beverage>(BeverageDecorator<T>);

impl<T: Beverage> Sugar<T> {
    pub fn new(beverage: T) -> Self {
        Sugar(BeverageDecorator::new(beverage))
    }
}

impl<T: Beverage> Beverage for Sugar<T> {
    fn cost(&self) -> f64 {
        self.0.beverage.cost() + 0.5
    }

    fn description(&self) -> String {
        format!("{} с сахаром", self.0.beverage.description())
    }
}

/// Тест для паттерна Decorator.
#[test]
fn test_decorator() {
    let coffee = Coffee;
    assert_eq!(coffee.cost(), 5.0);
    assert_eq!(coffee.description(), "Кофе");

    let coffee_with_milk = Milk::new(coffee);
    assert_eq!(coffee_with_milk.cost(), 6.5);
    assert_eq!(coffee_with_milk.description(), "Кофе с молоком");

    let coffee_with_milk_and_sugar = Sugar::new(coffee_with_milk);
    assert_eq!(coffee_with_milk_and_sugar.cost(), 7.0);
    assert_eq!(coffee_with_milk_and_sugar.description(), "Кофе с молоком с сахаром");
}