// Паттерн Template Method: определяет скелет алгоритма в суперклассе,
// но позволяет подклассам переопределить определенные шаги алгоритма
// без изменения его структуры.
// Полезен для определения общего алгоритма с вариациями.
// Пример: приготовление кофе и чая.

/// Трейт для напитка с методом шаблона.
pub trait Beverage {
    fn prepare(&self) {
        self.boil_water();
        self.brew();
        self.pour_in_cup();
        if self.customer_wants_condiments() {
            self.add_condiments();
        }
    }

    fn boil_water(&self) {
        println!("Кипячение воды");
    }

    fn pour_in_cup(&self) {
        println!("Налив в чашку");
    }

    fn brew(&self);
    fn add_condiments(&self);
    fn customer_wants_condiments(&self) -> bool {
        true // По умолчанию добавляем
    }
}

/// Конкретный напиток - кофе.
pub struct Coffee;

impl Beverage for Coffee {
    fn brew(&self) {
        println!("Заваривание кофе");
    }

    fn add_condiments(&self) {
        println!("Добавление сахара и молока");
    }
}

/// Конкретный напиток - чай.
pub struct Tea;

impl Beverage for Tea {
    fn brew(&self) {
        println!("Заваривание чая");
    }

    fn add_condiments(&self) {
        println!("Добавление лимона");
    }
}

/// Тест для паттерна Template Method.
#[test]
fn test_template_method() {
    let coffee = Coffee;
    let tea = Tea;

    println!("Приготовление кофе:");
    coffee.prepare();

    println!("\nПриготовление чая:");
    tea.prepare();
}