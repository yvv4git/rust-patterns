// Паттерн Strategy: определяет семейство алгоритмов, инкапсулирует каждый из них
// и делает их взаимозаменяемыми. Позволяет изменять алгоритм независимо от клиентов.
// Полезен для выбора алгоритма во время выполнения.
// Пример: сортировка с разными алгоритмами.

/// Трейт для стратегии сортировки.
pub trait SortStrategy {
    fn sort(&self, data: &mut [i32]);
}

/// Конкретная стратегия - быстрая сортировка.
pub struct QuickSort;

impl SortStrategy for QuickSort {
    fn sort(&self, data: &mut [i32]) {
        println!("Сортировка быстрой сортировкой");
        quick_sort(data);
    }
}

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr[arr.len() - 1];
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}

/// Конкретная стратегия - пузырьковая сортировка.
pub struct BubbleSort;

impl SortStrategy for BubbleSort {
    fn sort(&self, data: &mut [i32]) {
        println!("Сортировка пузырьковой сортировкой");
        let len = data.len();
        for i in 0..len {
            for j in 0..len - i - 1 {
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }
            }
        }
    }
}

/// Контекст - сортировщик.
pub struct Sorter {
    strategy: Box<dyn SortStrategy>,
}

impl Sorter {
    pub fn new(strategy: Box<dyn SortStrategy>) -> Self {
        Sorter { strategy }
    }

    pub fn set_strategy(&mut self, strategy: Box<dyn SortStrategy>) {
        self.strategy = strategy;
    }

    pub fn sort(&self, data: &mut [i32]) {
        self.strategy.sort(data);
    }
}

/// Тест для паттерна Strategy.
#[test]
fn test_strategy() {
    let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6];

    let mut sorter = Sorter::new(Box::new(BubbleSort));
    sorter.sort(&mut data);
    println!("После пузырьковой: {:?}", data);

    data = vec![3, 1, 4, 1, 5, 9, 2, 6]; // Сброс
    sorter.set_strategy(Box::new(QuickSort));
    sorter.sort(&mut data);
    println!("После быстрой: {:?}", data);
}