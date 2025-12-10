/*
The Typestate Pattern in Rust

Typestate - это способ проектирования API, при котором *состояние объекта* кодируется в типах.
Благодаря этому компилятор не даст вам выполнить действия, которые недопустимы в текущем состоянии объекта.

Почему это важно:

✔ Ошибки переносятся из runtime в compile-time
✔ Программы становятся безопаснее и предсказуемее
✔ Снижается необходимость ручных проверок состояния

Пример: файл открыт — можно читать и писать; файл закрыт — операции запрещены.
В Rust такие состояния реализуются через перемещение владения объектом — старое состояние просто невозможно использовать.

Эта идея активно применяется, например, в Serde, где порядок вызовов API гарантируется самим компилятором.

Итог: typestate — это способ создавать API, которые легко использовать правильно и невозможно использовать неправильно.
*/

// Структура для закрытого файла
struct FileClosed {
    path: String,
}

impl FileClosed {
    // Метод для открытия файла
    fn open(self) -> FileOpen {
        println!("Открываем файл: {}", self.path);
        // В реальном коде здесь была бы логика открытия файла
        FileOpen {
            path: self.path,
            content: String::new(),
        }
    }
}

// Структура для открытого файла
struct FileOpen {
    path: String,
    content: String,
}

impl FileOpen {
    // Метод для чтения содержимого
    fn read(&self) -> &str {
        println!("Читаем содержимое файла: {}", self.path);
        &self.content
    }

    // Метод для записи в файл
    fn write(&mut self, data: &str) {
        println!("Записываем '{}' в файл: {}", data, self.path);
        self.content.push_str(data);
    }

    // Метод для закрытия файла
    fn close(self) -> FileClosed {
        println!("Закрываем файл: {}", self.path);
        // В реальном коде здесь была бы логика закрытия файла
        FileClosed {
            path: self.path,
        }
    }
}

#[test]
fn test_typestate_pattern() {
    // Создаем закрытый файл
    let closed_file = FileClosed {
        path: "example.txt".to_string(),
    };

    // Открываем файл - получаем FileOpen
    let mut open_file = closed_file.open();

    // Теперь можно читать и писать
    open_file.write("Hello, ");
    open_file.write("World!");
    assert_eq!(open_file.read(), "Hello, World!");

    // Закрываем файл - получаем FileClosed обратно
    let closed_file_again = open_file.close();

    // Теперь файл закрыт, и мы не можем читать/писать
    // Если бы мы попытались использовать open_file здесь, компилятор бы не дал

    // Можно снова открыть
    let mut open_file2 = closed_file_again.open();
    open_file2.write("New content");
    assert_eq!(open_file2.read(), "New content");
    open_file2.close();
}