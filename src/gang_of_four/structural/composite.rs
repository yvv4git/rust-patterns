// Паттерн Composite: позволяет клиентам работать с отдельными объектами и их композициями единообразно.
// Полезен для древовидных структур.
// Пример: файловая система с файлами и папками.

/// Трейт для компонента (файл или папка).
pub trait Component {
    fn operation(&self);
}

/// Лист - файл.
pub struct File {
    name: String,
}

impl File {
    pub fn new(name: &str) -> Self {
        File {
            name: name.to_string(),
        }
    }
}

impl Component for File {
    fn operation(&self) {
        println!("Файл: {}", self.name);
    }
}

/// Композит - папка.
pub struct Folder {
    name: String,
    children: Vec<Box<dyn Component>>,
}

impl Folder {
    pub fn new(name: &str) -> Self {
        Folder {
            name: name.to_string(),
            children: Vec::new(),
        }
    }

    pub fn add(&mut self, component: Box<dyn Component>) {
        self.children.push(component);
    }
}

impl Component for Folder {
    fn operation(&self) {
        println!("Папка: {}", self.name);
        for child in &self.children {
            child.operation();
        }
    }
}

/// Тест для паттерна Composite.
#[test]
fn test_composite() {
    let mut root = Folder::new("Корневая папка");
    let file1 = Box::new(File::new("file1.txt"));
    let file2 = Box::new(File::new("file2.txt"));

    let mut subfolder = Folder::new("Подпапка");
    let file3 = Box::new(File::new("file3.txt"));
    subfolder.add(file3);

    root.add(file1);
    root.add(file2);
    root.add(Box::new(subfolder));

    root.operation();
}