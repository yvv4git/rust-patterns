// Паттерн Adapter: позволяет объектам с несовместимыми интерфейсами работать вместе.
// Адаптер оборачивает объект, предоставляя интерфейс, ожидаемый клиентом.
// Пример: адаптер для индюка, чтобы он вел себя как утка.

/// Трейт для утки.
pub trait Duck {
    fn quack(&self);
    fn fly(&self);
}

/// Реальная утка - кряква.
pub struct MallardDuck;

impl Duck for MallardDuck {
    fn quack(&self) {
        println!("Кря-кря!");
    }

    fn fly(&self) {
        println!("Утка летит!");
    }
}

/// Трейт для индюка.
pub trait Turkey {
    fn gobble(&self);
    fn fly(&self);
}

/// Дикий индюк.
pub struct WildTurkey;

impl Turkey for WildTurkey {
    fn gobble(&self) {
        println!("Курлы-курлы!");
    }

    fn fly(&self) {
        println!("Индюк летит на короткое расстояние!");
    }
}

/// Адаптер: индюк, адаптированный к интерфейсу утки.
pub struct TurkeyAdapter {
    turkey: WildTurkey,
}

impl TurkeyAdapter {
    pub fn new(turkey: WildTurkey) -> Self {
        TurkeyAdapter { turkey }
    }
}

impl Duck for TurkeyAdapter {
    fn quack(&self) {
        self.turkey.gobble();
    }

    fn fly(&self) {
        for _ in 0..5 {
            self.turkey.fly();
        }
    }
}

/// Тест для паттерна Adapter.
#[test]
fn test_adapter() {
    let duck = MallardDuck;
    let turkey = WildTurkey;
    let turkey_adapter = TurkeyAdapter::new(WildTurkey);

    println!("Утка:");
    duck.quack();
    duck.fly();

    println!("\nИндюк:");
    turkey.gobble();
    turkey.fly();

    println!("\nАдаптер индюка:");
    turkey_adapter.quack();
    turkey_adapter.fly();
}