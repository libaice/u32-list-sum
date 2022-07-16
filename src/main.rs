fn main() {
    // 第一次使用正常的几个值, 输出正常
    let list = [1, 2, 3, 4, 5];
    let res1 = sum_of_u32(&list);
    match res1 {
        Some(x) => println!("the sum of the list is {}", x),
        None => {
            println!("the sum has overflow")
        }
    }

    // u32 max value = 2 ** 32 - 1 = 4294967295
    let list2 = [4294967295, 1, 0];
    let res2 = sum_of_u32(&list2);
    match res2 {
        Some(x) => println!("the sum of the res2 is {}", x),
        None => {
            println!("the sum has overflow")
        }
    }
}


// 参数传入 &[u32] => u32数组的指针,返回类型为Option
fn sum_of_u32(list: &[u32]) -> Option<u32> {
    list.iter().try_fold(0u32, |acc, &x| acc.checked_add(x))
}


// 使用的Iterator Example  https://doc.rust-lang.org/std/iter/trait.Iterator.html

/*

let a = [1, 2, 3];

// the checked sum of all of the elements of the array
let sum = a.iter().try_fold(0i8, |acc, &x| acc.checked_add(x));

assert_eq!(sum, Some(6));

*/