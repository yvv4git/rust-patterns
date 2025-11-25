// Паттерн Observer: определяет зависимость "один ко многим" между объектами,
// так что при изменении состояния одного объекта все зависимые от него объекты
// уведомляются и обновляются автоматически.
// Полезен для реализации систем событий.
// Пример: новостное агентство и подписчики.

use std::rc::Rc;

/// Трейт для наблюдателя.
pub trait Observer {
    fn update(&self, news: &str);
}

/// Трейт для субъекта.
pub trait Subject {
    fn attach(&mut self, observer: Rc<dyn Observer>);
    #[allow(dead_code)]
    fn detach(&mut self, _observer: &Rc<dyn Observer>);
    fn notify(&self);
}

/// Конкретный субъект - новостное агентство.
pub struct NewsAgency {
    observers: Vec<Rc<dyn Observer>>,
    news: String,
}

impl NewsAgency {
    pub fn new() -> Self {
        NewsAgency {
            observers: Vec::new(),
            news: String::new(),
        }
    }

    pub fn set_news(&mut self, news: &str) {
        self.news = news.to_string();
        println!("Новость обновлена: {}", news);
        self.notify();
    }

    #[allow(dead_code)]
    pub fn get_news(&self) -> &str {
        &self.news
    }
}

impl Subject for NewsAgency {
    fn attach(&mut self, observer: Rc<dyn Observer>) {
        self.observers.push(observer);
    }

    fn detach(&mut self, _observer: &Rc<dyn Observer>) {
        // В Rust сложно удалить из Vec по ссылке, так что пропустим для простоты
    }

    fn notify(&self) {
        for observer in &self.observers {
            observer.update(&self.news);
        }
    }
}

/// Конкретный наблюдатель - подписчик.
pub struct Subscriber {
    name: String,
}

impl Subscriber {
    pub fn new(name: &str) -> Rc<Self> {
        Rc::new(Subscriber {
            name: name.to_string(),
        })
    }
}

impl Observer for Subscriber {
    fn update(&self, news: &str) {
        println!("{} получил новость: {}", self.name, news);
    }
}

/// Тест для паттерна Observer.
#[test]
fn test_observer() {
    let mut agency = NewsAgency::new();

    let subscriber1 = Subscriber::new("Алексей");
    let subscriber2 = Subscriber::new("Мария");

    agency.attach(subscriber1);
    agency.attach(subscriber2);

    agency.set_news("Важная новость!");
}