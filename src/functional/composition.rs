// Функциональный паттерн Composition: объединение функций в цепочку.
// Позволяет создавать сложные функции из простых путем композиции.
// Полезен для создания пайплайнов обработки данных.
// Пример: композиция функций для преобразования данных.

/// Функция композиции: f после g (g ∘ f)
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

/// Функция композиции для трех функций
pub fn compose3<A, B, C, D, F, G, H>(f: F, g: G, h: H) -> impl Fn(A) -> D
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
    H: Fn(C) -> D,
{
    move |x| h(g(f(x)))
}

/// Функция пайплайна (левая композиция): f затем g
pub fn pipe<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

#[test]
fn test_composition() {
    // Простые функции
    let add_one = |x: i32| x + 1;
    let multiply_two = |x: i32| x * 2;
    let to_string = |x: i32| format!("Result: {}", x);

    // Композиция: multiply_two ∘ add_one
    let composed = compose(add_one, multiply_two);
    assert_eq!(composed(3), 8); // (3 + 1) * 2 = 8

    // Композиция трех функций
    let composed3 = compose3(add_one, multiply_two, to_string);
    assert_eq!(composed3(3), "Result: 8");

    // Пайплайн: add_one затем multiply_two
    let piped = pipe(add_one, multiply_two);
    assert_eq!(piped(3), 8); // 3 + 1 = 4, 4 * 2 = 8

    // Более сложный пример
    let square = |x: i32| x * x;
    let negate = |x: i32| -x;
    let double = |x: i32| x * 2;

    let complex = compose3(square, negate, double);
    assert_eq!(complex(3), -18); // (3^2) = 9, -9 = -9, -9 * 2 = -18
}