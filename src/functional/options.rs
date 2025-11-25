// Функциональный паттерн Options: позволяет конфигурировать объекты с помощью функций-опций.
// Полезен для создания объектов с множеством опциональных параметров.
// Пример: конфигурация дома с различными опциями.

const DEFAULT_FLOORS: i32 = 2;
const DEFAULT_HAS_FIREPLACE: bool = true;
const DEFAULT_MATERIAL: &str = "wood";

// HouseOption - функция для конфигурации дома.
pub type HouseOption = Box<dyn Fn(&mut House)>;

/// Структура дома.
// #[derive(Debug, PartialEq)] - атрибут derive автоматически генерирует реализации трейтов Debug и PartialEq.
// Debug позволяет выводить структуру в отладочном формате (println!("{:?}", house)).
// PartialEq позволяет сравнивать структуры на равенство (house1 == house2).
#[derive(Debug, PartialEq)]
pub struct House {
    pub material: String,
    pub has_fireplace: bool,
    pub floors: i32,
}

impl House {
    /// Создает новый дом с опциями.
    pub fn new(options: Vec<HouseOption>) -> Self {
        let mut house = House {
            material: DEFAULT_MATERIAL.to_string(),
            has_fireplace: DEFAULT_HAS_FIREPLACE,
            floors: DEFAULT_FLOORS,
        };

        for option in options {
            option(&mut house);
        }

        house
    }
}

#[allow(dead_code)]
/// Опция для установки материала кирпич.
pub fn with_material_kerpic() -> HouseOption {
    Box::new(|h: &mut House| {
        h.material = "kerpic".to_string();
    })
}

#[allow(dead_code)]
/// Опция для отключения камина.
pub fn without_fireplace() -> HouseOption {
    Box::new(|h: &mut House| {
        h.has_fireplace = false;
    })
}

#[allow(dead_code)]
/// Опция для установки количества этажей.
pub fn with_floors(floors: i32) -> HouseOption {
    Box::new(move |h: &mut House| {
        h.floors = floors;
    })
}

#[test]
fn test_option() {
    // Дом по умолчанию
    let default_house = House::new(vec![]);
    assert_eq!(default_house.material, "wood");
    assert_eq!(default_house.has_fireplace, true);
    assert_eq!(default_house.floors, 2);

    // Дом с кирпичом
    let kerpic_house = House::new(vec![with_material_kerpic()]);
    assert_eq!(kerpic_house.material, "kerpic");
    assert_eq!(kerpic_house.has_fireplace, true);
    assert_eq!(kerpic_house.floors, 2);

    // Дом без камина и с 3 этажами
    let custom_house = House::new(vec![without_fireplace(), with_floors(3)]);
    assert_eq!(custom_house.material, "wood");
    assert_eq!(custom_house.has_fireplace, false);
    assert_eq!(custom_house.floors, 3);

    // Дом со всеми опциями
    let full_house = House::new(vec![with_material_kerpic(), without_fireplace(), with_floors(5)]);
    assert_eq!(full_house.material, "kerpic");
    assert_eq!(full_house.has_fireplace, false);
    assert_eq!(full_house.floors, 5);
}