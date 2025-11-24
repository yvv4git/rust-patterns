// Паттерн Abstract Factory (Фабрика): предоставляет интерфейс для создания
// семейств связанных или зависимых объектов без указания их конкретных классов.
// Полезен для создания платформо-зависимых объектов.

/// Трейт для абстрактного продукта A.
pub trait AbstractProductA {
    fn operation_a(&self) -> String;
}

/// Трейт для абстрактного продукта B.
pub trait AbstractProductB {
    fn operation_b(&self) -> String;
    fn collaborate(&self, collaborator: &dyn AbstractProductA) -> String;
}

/// Конкретный продукт A1.
pub struct ConcreteProductA1;

impl AbstractProductA for ConcreteProductA1 {
    fn operation_a(&self) -> String {
        "Результат операции A1".to_string()
    }
}

/// Конкретный продукт B1.
pub struct ConcreteProductB1;

impl AbstractProductB for ConcreteProductB1 {
    fn operation_b(&self) -> String {
        "Результат операции B1".to_string()
    }

    fn collaborate(&self, collaborator: &dyn AbstractProductA) -> String {
        format!("Результат сотрудничества B1 с ({})", collaborator.operation_a())
    }
}

/// Конкретный продукт A2.
pub struct ConcreteProductA2;

impl AbstractProductA for ConcreteProductA2 {
    fn operation_a(&self) -> String {
        "Результат операции A2".to_string()
    }
}

/// Конкретный продукт B2.
pub struct ConcreteProductB2;

impl AbstractProductB for ConcreteProductB2 {
    fn operation_b(&self) -> String {
        "Результат операции B2".to_string()
    }

    fn collaborate(&self, collaborator: &dyn AbstractProductA) -> String {
        format!("Результат сотрудничества B2 с ({})", collaborator.operation_a())
    }
}

/// Трейт для абстрактной фабрики.
pub trait AbstractFactory {
    fn create_product_a(&self) -> Box<dyn AbstractProductA>;
    fn create_product_b(&self) -> Box<dyn AbstractProductB>;
}

/// Конкретная фабрика 1, создающая продукты семейства 1.
pub struct ConcreteFactory1;

impl AbstractFactory for ConcreteFactory1 {
    fn create_product_a(&self) -> Box<dyn AbstractProductA> {
        Box::new(ConcreteProductA1)
    }

    fn create_product_b(&self) -> Box<dyn AbstractProductB> {
        Box::new(ConcreteProductB1)
    }
}

/// Конкретная фабрика 2, создающая продукты семейства 2.
pub struct ConcreteFactory2;

impl AbstractFactory for ConcreteFactory2 {
    fn create_product_a(&self) -> Box<dyn AbstractProductA> {
        Box::new(ConcreteProductA2)
    }

    fn create_product_b(&self) -> Box<dyn AbstractProductB> {
        Box::new(ConcreteProductB2)
    }
}

/// Функция клиента, использующая фабрику.
#[allow(dead_code)]
fn client_code(factory: &dyn AbstractFactory) {
    let product_a = factory.create_product_a();
    let product_b = factory.create_product_b();

    println!("{} {}", product_a.operation_a(), product_b.operation_b());
    println!("{}", product_b.collaborate(&*product_a));
}

/// Тест для проверки паттерна Abstract Factory.
#[test]
fn test_abstract_factory() {
    let factory1 = ConcreteFactory1;
    let product_a1 = factory1.create_product_a();
    let product_b1 = factory1.create_product_b();

    assert_eq!(product_a1.operation_a(), "Результат операции A1");
    assert_eq!(product_b1.operation_b(), "Результат операции B1");
    assert_eq!(product_b1.collaborate(&*product_a1), "Результат сотрудничества B1 с (Результат операции A1)");

    let factory2 = ConcreteFactory2;
    let product_a2 = factory2.create_product_a();
    let product_b2 = factory2.create_product_b();

    assert_eq!(product_a2.operation_a(), "Результат операции A2");
    assert_eq!(product_b2.operation_b(), "Результат операции B2");
    assert_eq!(product_b2.collaborate(&*product_a2), "Результат сотрудничества B2 с (Результат операции A2)");
}