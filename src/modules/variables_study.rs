#![allow(dead_code)]

pub fn run() {
    /* int
      int может быть signed i32 и unsigned u32, signed значения с возможным знаком (минус)
       То есть которые могут быть меньше нуля
       Стандартное значение i32
    */
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    /* float
      float может быть только signed, f32 и f64.
      дефолотное значение f64.
    */
    let x = 5.0;
    println!("The value of x is: {x}");
    let x: f32 = 0.01 + 5.1;
    // В константах обязательно указывать тип
    const Y: f32 = 10.1;
    let mut test = 1;
    println!("{test}");
    test = 2;
    println!("{test}");
    println!("The value of x is: {x}");

    /* tuples
    Или кортеж по русски.
    tuples это базовый сложный тип который позволяет хранить
    коллекцию данных разных типов, фиксированного размера
    */
    let tuple: (i32, f64, bool) = (1, 2.0, false);
    let (t_int, t_float, t_bool) = tuple;
    println!("int: {}| float: {}| bool {}|", t_int, t_float, t_bool);
    // Доступ к элементам tuple проиходит через точечную нотацию
    let tuple_index_test = tuple.2;
    println!("{tuple_index_test}");
    // Возможен пустой tuple

    let _empty_tuple: () = ();

    /* array
    Масивы должны содержать данные одного типа
    В отличие от других языков, массивы в rust имеют фиксированный размер.
    Если нужно больше гибкости, использовать тип vector
    В типе массива выводится тип и длинна массива
    */

    let array_test: [i32; 5] = [1, 2, 3, 4, 0];
    // Массив можно объявить с дефолтным значением    #[allow(unused_variables)]
    let _array_with_def = ['a'; 5];

    let _array_access_test = array_test[4];
}
