// Паттерн Abstract Factory: предоставляет интерфейс для создания
// семейств связанных объектов без указания конкретных классов.
// Пример: создание GUI элементов для разных операционных систем.

/// Трейт для кнопки.
pub trait Button {
    fn paint(&self) -> String;
}

/// Трейт для чекбокса.
pub trait Checkbox {
    fn paint(&self) -> String;
}

/// Кнопка для Windows.
pub struct WinButton;

impl Button for WinButton {
    fn paint(&self) -> String {
        "Отрисовка кнопки в стиле Windows".to_string()
    }
}

/// Чекбокс для Windows.
pub struct WinCheckbox;

impl Checkbox for WinCheckbox {
    fn paint(&self) -> String {
        "Отрисовка чекбокса в стиле Windows".to_string()
    }
}

/// Кнопка для macOS.
pub struct MacButton;

impl Button for MacButton {
    fn paint(&self) -> String {
        "Отрисовка кнопки в стиле macOS".to_string()
    }
}

/// Чекбокс для macOS.
pub struct MacCheckbox;

impl Checkbox for MacCheckbox {
    fn paint(&self) -> String {
        "Отрисовка чекбокса в стиле macOS".to_string()
    }
}

/// Трейт для фабрики GUI элементов.
pub trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}

/// Фабрика для Windows.
pub struct WinFactory;

impl GUIFactory for WinFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WinButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WinCheckbox)
    }
}

/// Фабрика для macOS.
pub struct MacFactory;

impl GUIFactory for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox)
    }
}

/// Функция приложения, использующая фабрику.
#[allow(dead_code)]
pub fn application(factory: &dyn GUIFactory) {
    let button = factory.create_button();
    let checkbox = factory.create_checkbox();

    println!("{}\n{}", button.paint(), checkbox.paint());
}

/// Тест для паттерна Abstract Factory.
#[test]
fn test_abstract_factory() {
    let win_factory = WinFactory;
    let button = win_factory.create_button();
    let checkbox = win_factory.create_checkbox();

    assert_eq!(button.paint(), "Отрисовка кнопки в стиле Windows");
    assert_eq!(checkbox.paint(), "Отрисовка чекбокса в стиле Windows");

    let mac_factory = MacFactory;
    let button = mac_factory.create_button();
    let checkbox = mac_factory.create_checkbox();

    assert_eq!(button.paint(), "Отрисовка кнопки в стиле macOS");
    assert_eq!(checkbox.paint(), "Отрисовка чекбокса в стиле macOS");
}