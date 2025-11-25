// Паттерн Interpreter: определяет представление грамматики языка
// и интерпретатор предложений этого языка.
// Полезен для интерпретации языков или грамматик.
// Пример: простой калькулятор арифметических выражений.

/// Трейт для выражения.
pub trait Expression {
    fn interpret(&self) -> i32;
}

/// Терминальное выражение - число.
pub struct Number {
    value: i32,
}

impl Number {
    pub fn new(value: i32) -> Self {
        Number { value }
    }
}

impl Expression for Number {
    fn interpret(&self) -> i32 {
        println!("Число: {}", self.value);
        self.value
    }
}

/// Нетерминальное выражение - сложение.
pub struct Add {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Add {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Add { left, right }
    }
}

impl Expression for Add {
    fn interpret(&self) -> i32 {
        let left_val = self.left.interpret();
        let right_val = self.right.interpret();
        let result = left_val + right_val;
        println!("Сложение: {} + {} = {}", left_val, right_val, result);
        result
    }
}

/// Нетерминальное выражение - умножение.
pub struct Multiply {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Multiply {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Multiply { left, right }
    }
}

impl Expression for Multiply {
    fn interpret(&self) -> i32 {
        let left_val = self.left.interpret();
        let right_val = self.right.interpret();
        let result = left_val * right_val;
        println!("Умножение: {} * {} = {}", left_val, right_val, result);
        result
    }
}

/// Тест для паттерна Interpreter.
#[test]
fn test_interpreter() {
    // Построение выражения: (1 + 2) * 3
    let expr = Multiply::new(
        Box::new(Add::new(
            Box::new(Number::new(1)),
            Box::new(Number::new(2)),
        )),
        Box::new(Number::new(3)),
    );

    let result = expr.interpret();
    println!("Результат: {}", result);
    assert_eq!(result, 9);
}