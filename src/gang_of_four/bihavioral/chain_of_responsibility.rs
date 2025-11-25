// Паттерн Chain of Responsibility: позволяет передавать запросы последовательно
// по цепочке обработчиков. Каждый обработчик решает, может ли он обработать запрос,
// и если нет, передает дальше.
// Полезен для систем с множеством обработчиков.
// Пример: поддержка клиентов с разными уровнями сложности.

use std::rc::Rc;
use std::cell::RefCell;

/// Трейт для обработчика.
pub trait Handler {
    fn handle(&self, request: &str) -> bool;
    fn set_next(&self, next: Rc<dyn Handler>);
}

/// Базовый обработчик с общими методами.
pub struct BaseHandler {
    next: RefCell<Option<Rc<dyn Handler>>>,
}

impl BaseHandler {
    pub fn new() -> Self {
        BaseHandler {
            next: RefCell::new(None),
        }
    }

    pub fn handle_next(&self, request: &str) -> bool {
        if let Some(ref next) = *self.next.borrow() {
            next.handle(request)
        } else {
            false
        }
    }
}

impl Handler for BaseHandler {
    fn handle(&self, _request: &str) -> bool {
        self.handle_next(_request)
    }

    fn set_next(&self, next: Rc<dyn Handler>) {
        *self.next.borrow_mut() = Some(next);
    }
}

/// Конкретный обработчик - базовая поддержка.
pub struct BasicSupport {
    base: BaseHandler,
}

impl BasicSupport {
    pub fn new() -> Self {
        BasicSupport {
            base: BaseHandler::new(),
        }
    }
}

impl Handler for BasicSupport {
    fn handle(&self, request: &str) -> bool {
        if request.contains("базовый") {
            println!("Базовая поддержка обработала запрос: {}", request);
            true
        } else {
            println!("Базовая поддержка не может обработать, передаем дальше");
            self.base.handle_next(request)
        }
    }

    fn set_next(&self, next: Rc<dyn Handler>) {
        self.base.set_next(next);
    }
}

/// Конкретный обработчик - средняя поддержка.
pub struct MediumSupport {
    base: BaseHandler,
}

impl MediumSupport {
    pub fn new() -> Self {
        MediumSupport {
            base: BaseHandler::new(),
        }
    }
}

impl Handler for MediumSupport {
    fn handle(&self, request: &str) -> bool {
        if request.contains("средний") {
            println!("Средняя поддержка обработала запрос: {}", request);
            true
        } else {
            println!("Средняя поддержка не может обработать, передаем дальше");
            self.base.handle_next(request)
        }
    }

    fn set_next(&self, next: Rc<dyn Handler>) {
        self.base.set_next(next);
    }
}

/// Конкретный обработчик - продвинутая поддержка.
pub struct AdvancedSupport {
    base: BaseHandler,
}

impl AdvancedSupport {
    pub fn new() -> Self {
        AdvancedSupport {
            base: BaseHandler::new(),
        }
    }
}

impl Handler for AdvancedSupport {
    fn handle(&self, request: &str) -> bool {
        if request.contains("продвинутый") {
            println!("Продвинутая поддержка обработала запрос: {}", request);
            true
        } else {
            println!("Продвинутая поддержка не может обработать, передаем дальше");
            self.base.handle_next(request)
        }
    }

    fn set_next(&self, next: Rc<dyn Handler>) {
        self.base.set_next(next);
    }
}

/// Тест для паттерна Chain of Responsibility.
#[test]
fn test_chain_of_responsibility() {
    let basic = Rc::new(BasicSupport::new());
    let medium = Rc::new(MediumSupport::new());
    let advanced = Rc::new(AdvancedSupport::new());

    // Установка цепочки
    basic.set_next(medium.clone());
    medium.set_next(advanced);

    // Тестирование
    basic.handle("базовый запрос");
    basic.handle("средний запрос");
    basic.handle("продвинутый запрос");
    basic.handle("неизвестный запрос");
}