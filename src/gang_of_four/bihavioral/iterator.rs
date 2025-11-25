// Паттерн Iterator: предоставляет способ последовательного доступа к элементам коллекции
// без раскрытия внутренней структуры. Полезен для унифицированного обхода коллекций.
// Пример: итератор для списка имен.

/// Трейт для итератора.
pub trait Iterator<T> {
    fn has_next(&self) -> bool;
    fn next(&mut self) -> Option<T>;
}

/// Агрегат - список имен.
pub struct Names {
    names: Vec<String>,
}

impl Names {
    pub fn new(names: Vec<String>) -> Self {
        Names { names }
    }

    pub fn create_iterator(&self) -> NamesIterator {
        NamesIterator {
            names: &self.names,
            index: 0,
        }
    }
}

/// Конкретный итератор - NamesIterator.
pub struct NamesIterator<'a> {
    names: &'a Vec<String>,
    index: usize,
}

impl<'a> Iterator<String> for NamesIterator<'a> {
    fn has_next(&self) -> bool {
        self.index < self.names.len()
    }

    fn next(&mut self) -> Option<String> {
        if self.has_next() {
            let name = self.names[self.index].clone();
            self.index += 1;
            println!("Итерация: {}", name);
            Some(name)
        } else {
            None
        }
    }
}

/// Тест для паттерна Iterator.
#[test]
fn test_iterator() {
    let names = Names::new(vec![
        "Алексей".to_string(),
        "Мария".to_string(),
        "Иван".to_string(),
    ]);

    let mut iterator = names.create_iterator();

    while iterator.has_next() {
        let _ = iterator.next();
    }
}