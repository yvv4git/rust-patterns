// Паттерн Factory Method: определяет интерфейс для создания объектов,
// но позволяет подклассам решать, какой класс инстанцировать.
// Полезен для создания семейств связанных объектов.

/// Трейт для продуктов, создаваемых фабрикой.
pub trait Product {
    fn operation(&self) -> String;
}

/// Конкретный продукт A.
pub struct ConcreteProductA;

impl Product for ConcreteProductA {
    fn operation(&self) -> String {
        "Результат продукта A".to_string()
    }
}

/// Конкретный продукт B.
pub struct ConcreteProductB;

impl Product for ConcreteProductB {
    fn operation(&self) -> String {
        "Результат продукта B".to_string()
    }
}

/// Трейт для создателя, определяющий фабричный метод.
pub trait Creator {
    fn factory_method(&self) -> Box<dyn Product>;
}

/// Конкретный создатель A, создающий продукт A.
pub struct ConcreteCreatorA;

impl Creator for ConcreteCreatorA {
    fn factory_method(&self) -> Box<dyn Product> {
        Box::new(ConcreteProductA)
    }
}

/// Конкретный создатель B, создающий продукт B.
pub struct ConcreteCreatorB;

impl Creator for ConcreteCreatorB {
    fn factory_method(&self) -> Box<dyn Product> {
        Box::new(ConcreteProductB)
    }
}

/// Тест для проверки паттерна Factory Method.
#[test]
fn test_factory_method() {
    let creator_a = ConcreteCreatorA;
    let product_a = creator_a.factory_method();
    assert_eq!(product_a.operation(), "Результат продукта A");

    let creator_b = ConcreteCreatorB;
    let product_b = creator_b.factory_method();
    assert_eq!(product_b.operation(), "Результат продукта B");
}