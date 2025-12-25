#![allow(dead_code)]
pub fn run() {
    loop {
        println!("The end is never ");
        break;
    }
    /*
    Внезапно лупы могут работать как выражения и также возвращать результат, после break
     */
    let mut i: u128 = 2;
    let mut counter = 0;

    // This is fast
    let wow = loop {
        i = i * 2;
        counter = counter + 1;
        println!("Итерация: {}, результат: {}", counter, i);
        if counter >= 10 {
            //125
            break i;
        }
    };
    dbg!(wow);

    //А ещё лупам можно давать имена и отменять их по именам внутри другого лупа
    'fun: loop {
        loop {
            break 'fun;
        }
    }

    //while лупы
    'why: while i > 0 {
        i /= 2;
        dbg!(i);
        if i < 2 {
            break 'why;
        }
    }
    println!("#########################");
    //for лупы
    const ARR: [i32; 5] = [21, 42, 84, 168, 336];
    for i in ARR {
        dbg!(i);
    }

    for numb in 1..10 {
        println!("{}", numb);
    }
    let _a = String::from("asd");
    println!("{_a}");
}
