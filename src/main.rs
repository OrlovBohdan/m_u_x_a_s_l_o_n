use itertools::Itertools; // Потрібно додати бібліотеку itertools у Cargo.toml

// Функція для пошуку всіх можливих рішень задачі
fn find_solutions() -> usize {
    let digits = [1, 2, 3, 4, 5, 6, 7, 8]; // Цифри від 1 до 8
    let mut solution_count = 0; // Лічильник кількості рішень

    // Перебираємо всі перестановки цифр
    for perm in digits.iter().permutations(8) {
        // Присвоюємо кожній букві своє значення з перестановки
        let m = *perm[0];
        let u = *perm[1];
        let x = *perm[2];
        let a = *perm[3];
        let s = *perm[4];
        let l = *perm[5];
        let o = *perm[6];
        let n = *perm[7];

        // Формуємо числа "muxa" та "slon" з відповідних цифр
        let muxa = m * 1000 + u * 100 + x * 10 + a;
        let slon = s * 1000 + l * 100 + o * 10 + n;

        // Перевіряємо, чи задовольняє рівність умову "muxa * a = slon"
        if muxa * a == slon {
            // Якщо рівність виконується, виводимо знайдене рішення
            println!("Solution found: \n{} \n* {} \n------ \n{}", muxa, a, slon);
            solution_count += 1; // Збільшуємо лічильник рішень
        }
    }

    solution_count // Повертаємо кількість знайдених рішень
}

fn main() {
    // Викликаємо функцію для пошуку рішень і зберігаємо результат
    let total_solutions = find_solutions();

    // Виводимо загальну кількість рішень
    println!("Total number of solutions: {}", total_solutions);
}

/*
use itertools::Itertools; – імпорт бібліотеки для роботи з перестановками.
fn find_solutions() -> usize – функція для пошуку рішень. Вона повертає кількість знайдених рішень типу usize.
let digits = [1, 2, 3, 4, 5, 6, 7, 8]; – масив цифр від 1 до 8, які будуть використовуватися для заміни букв.
for perm in digits.iter().permutations(8) – цикл, який перебирає всі можливі перестановки 8 цифр.
let muxa = m * 1000 + u * 100 + x * 10 + a; – збираємо число з літер "muxa".
let slon = s * 1000 + l * 100 + o * 10 + n; – збираємо число з літер "slon".
if muxa * x == slon – перевіряємо, чи виконується рівність.
println!("Solution found...") – якщо рішення знайдено, виводимо його.
solution_count += 1; – збільшуємо лічильник кількості рішень.
return solution_count; – повертаємо загальну кількість рішень.
fn main() – головна функція програми, яка викликає find_solutions і виводить кількість знайдених рішень.
*/