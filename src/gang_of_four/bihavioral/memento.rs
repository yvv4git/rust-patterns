// Паттерн Memento: позволяет сохранить и восстановить предыдущее состояние объекта
// без нарушения инкапсуляции. Полезен для реализации отмены операций.
// Пример: сохранение состояния текстового редактора.

/// Мементо - хранит состояние.
pub struct EditorState {
    content: String,
}

impl EditorState {
    pub fn new(content: String) -> Self {
        EditorState { content }
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }
}

/// Инициатор - текстовый редактор.
pub struct TextEditor {
    content: String,
}

impl TextEditor {
    pub fn new() -> Self {
        TextEditor {
            content: String::new(),
        }
    }

    pub fn write(&mut self, text: &str) {
        self.content.push_str(text);
        println!("Написано: {}", text);
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn save(&self) -> EditorState {
        println!("Сохранение состояния: {}", self.content);
        EditorState::new(self.content.clone())
    }

    pub fn restore(&mut self, state: EditorState) {
        self.content = state.get_content().to_string();
        println!("Восстановлено состояние: {}", self.content);
    }
}

/// Хранитель - история.
pub struct History {
    states: Vec<EditorState>,
}

impl History {
    pub fn new() -> Self {
        History { states: Vec::new() }
    }

    pub fn push(&mut self, state: EditorState) {
        self.states.push(state);
    }

    pub fn pop(&mut self) -> Option<EditorState> {
        self.states.pop()
    }
}

/// Тест для паттерна Memento.
#[test]
fn test_memento() {
    let mut editor = TextEditor::new();
    let mut history = History::new();

    editor.write("Привет, ");
    history.push(editor.save());

    editor.write("мир!");
    history.push(editor.save());

    editor.write(" Как дела?");
    println!("Текущее содержание: {}", editor.get_content());

    // Отмена последнего изменения
    if let Some(state) = history.pop() {
        editor.restore(state);
    }

    println!("После отмены: {}", editor.get_content());
}