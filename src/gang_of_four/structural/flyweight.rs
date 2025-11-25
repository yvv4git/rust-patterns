// Паттерн Flyweight: использует разделение для эффективной поддержки множества мелких объектов.
// Полезен для уменьшения использования памяти при работе с большим количеством объектов.
// Пример: символы в текстовом редакторе с общими свойствами.

use std::collections::HashMap;
use std::rc::Rc;

/// Flyweight - символ с общими свойствами.
#[derive(Clone)]
pub struct Character {
    symbol: char,
    font: String,
}

impl Character {
    pub fn new(symbol: char, font: &str) -> Self {
        Character {
            symbol,
            font: font.to_string(),
        }
    }

    pub fn display(&self, position: (i32, i32)) {
        println!("Символ '{}' шрифтом '{}' на позиции ({}, {})", self.symbol, self.font, position.0, position.1);
    }
}

/// Фабрика flyweight для кеширования символов.
pub struct FlyweightFactory {
    characters: HashMap<(char, String), Rc<Character>>,
}

impl FlyweightFactory {
    pub fn new() -> Self {
        FlyweightFactory {
            characters: HashMap::new(),
        }
    }

    pub fn get_character(&mut self, symbol: char, font: &str) -> Rc<Character> {
        let key = (symbol, font.to_string());
        self.characters.entry(key).or_insert_with(|| {
            println!("Создание нового flyweight для символа '{}' шрифтом '{}'", symbol, font);
            Rc::new(Character::new(symbol, font))
        }).clone()
    }
}

/// Тест для паттерна Flyweight.
#[test]
fn test_flyweight() {
    let mut factory = FlyweightFactory::new();

    let char_a1 = factory.get_character('A', "Arial");
    let char_a2 = factory.get_character('A', "Arial"); // Повторное использование
    let char_b = factory.get_character('B', "Arial");

    char_a1.display((10, 20));
    char_a2.display((15, 25));
    char_b.display((20, 30));

    // Проверяем, что это один объект
    assert!(Rc::ptr_eq(&char_a1, &char_a2));
}