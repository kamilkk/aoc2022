use std::vec;

#[derive(Debug, Clone)]
struct Monkey {
    op: char,
    op_val: u64,
    test: u64,
    throw: Vec<usize>,
}

fn main() -> std::io::Result<()> {
    let monkeys: Vec<Monkey> = vec![
        Monkey {
            op: '*',
            op_val: 3,
            test: 11,
            throw: vec![2, 7],
        },
        Monkey {
            op: '^',
            op_val: 0,
            test: 7,
            throw: vec![0, 2],
        },
        Monkey {
            op: '+',
            op_val: 1,
            test: 3,
            throw: vec![7, 5],
        },
        Monkey {
            op: '+',
            op_val: 8,
            test: 5,
            throw: vec![6, 4],
        },
        Monkey {
            op: '+',
            op_val: 3,
            test: 17,
            throw: vec![1, 0],
        },
        Monkey {
            op: '+',
            op_val: 4,
            test: 13,
            throw: vec![6, 3],
        },
        Monkey {
            op: '*',
            op_val: 17,
            test: 19,
            throw: vec![4, 1],
        },
        Monkey {
            op: '+',
            op_val: 7,
            test: 2,
            throw: vec![5, 3],
        },
    ];

    let mut items: Vec<Vec<u64>> = vec![
        vec![50, 70, 54, 83, 52, 78],
        vec![71, 52, 58, 60, 71],
        vec![66, 56, 56, 94, 60, 86, 73],
        vec![83, 99],
        vec![98, 98, 79],
        vec![76],
        vec![52, 51, 84, 54],
        vec![82, 86, 91, 79, 94, 92, 59, 94],
    ];

    let mut inspections: Vec<i32> = vec![0, 0, 0, 0, 0, 0, 0, 0];

    let mut worry: u64;

    let rounds: u8 = 20;
    for _ in 0..rounds {
        for (monkey_idx, monkey) in monkeys.iter().enumerate() {
            for item in &items.clone()[monkey_idx] {
                inspections[monkey_idx] += 1;
                worry = match monkey.op {
                    '*' => item * monkey.op_val,
                    '+' => item + monkey.op_val,
                    '^' => item * item,
                    _ => 0,
                };

                worry /= 3;

                if worry % monkey.test == 0 {
                    items[monkey.throw[0]].push(worry);
                } else {
                    items[monkey.throw[1]].push(worry);
                }
            }
            items[monkey_idx].clear();
        }
    }
    inspections.sort();
    inspections.reverse();

    let result: u64 = inspections[0] as u64 * inspections[1] as u64;
    println!("Result: {}", result);

    Ok(())
}
