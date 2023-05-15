// fibonacci:
// 数列由0和1开始，之后的斐波那契数就是由之前的两数相加而得出。

use core::panic;

const F0: u128 = 0;
const F1: u128 = 1;

fn next_fibonacci_num(from: (u128, u128)) -> u128 {
    from.0 + from.1
}

fn main() {
    // 超过 94 的时候数列值超过 u64_max
    let fibonacci_num = 185;
    let mut group = (F0, F1);
    if fibonacci_num < 0 {
        panic!("请设置大于0的数字");
    }

    if fibonacci_num > 185 {
        panic!("请设置小于185的数字");
    }

    println!("{fibonacci_num} 级斐波那契数列（5个一组）：");

    // 0级
    if fibonacci_num > 0 {
        print!("第{:03}行：{}", 1, group.0);
    }

    // 1级
    if fibonacci_num > 1 {
        print!(",{}", group.1);
    }

    let mut num = 3;

    while num <= fibonacci_num {
        group = (group.1, next_fibonacci_num(group));

        match num % 5 {
            0 => {
                print!(",{}", group.1);
            }
            1 => {
                print!("\n第{:03}行：{}", num / 5 + 1, group.1);
            }
            _ => {
                print!(",{}", group.1);
            }
        }
        num += 1
    }
    println!();
}
