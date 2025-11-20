pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }

    // coins                                    -> [1, 2]
    // target                                   -> 5
    // index(current_amount)                    ->   0 1 2 3
    // amount_holder(current_best_coins_sum)    -> [ 0 1 2 3 ]
    let money_amount_usize = amount as usize;
    let mut coin_amount_holder: Vec<Option<i32>> = vec![None; money_amount_usize + 1];
    coin_amount_holder[0] = Some(0);

    println!("begin:");
    println!("count: {coin_amount_holder:?}");
    println!("amount_usize: {money_amount_usize:?}");
    // Цикл по всему спектру сумм валюты от 1 до amount_usize
    for current_money_amount in 1..=money_amount_usize {
        println!("amount: {current_money_amount} ");
        // Перебор монет всех номиналов
        for &coin in &coins {
            let coin_usize = coin as usize;
            print!("   coin: {coin_usize} ");

            // Проверка что монета меньше текущей суммы
            if coin_usize <= current_money_amount {
                // Находим предыдущее значение валюты, уменьшая текущую сумму валюты на номинал монеты и берем значение суммы монет,
                if let Some(previous_coins_sum) =
                    coin_amount_holder[current_money_amount - coin_usize]
                {
                    // Добавляем к сумме монет одну монету
                    let candidate_to_best_coins_sum = previous_coins_sum + 1;
                    coin_amount_holder[current_money_amount] =
                        match coin_amount_holder[current_money_amount] {
                            // Сравниваем сумму монет кандидата с суммой монет, которая уже лежит по данной сумме валют(чем меньше, тем лучше)
                            Some(current_best_coins_sum) => {
                                if candidate_to_best_coins_sum < current_best_coins_sum {
                                    Some(candidate_to_best_coins_sum)
                                } else {
                                    Some(current_best_coins_sum)
                                }
                            }
                            // Если суммы монет не было, добавляем сумму монет кандидата
                            None => Some(candidate_to_best_coins_sum),
                        }
                }

                print!("true  ");
                print!(
                    "coin number: {:?} ",
                    coin_amount_holder[current_money_amount]
                );
                println!("{coin_amount_holder:?}");
            } else {
                print!("false ");
                print!(
                    "coin number: {:?} ",
                    coin_amount_holder[current_money_amount]
                );
                println!("{coin_amount_holder:?}");
            }
        }
    }

    println!("end: {coin_amount_holder:?}");

    if coin_amount_holder[money_amount_usize].is_none() {
        -1
    } else {
        coin_amount_holder[money_amount_usize].unwrap()
    }
}

pub fn coin_change_r(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }

    let mut coins_usize: Vec<usize> = coins.iter().map(|&c| c as usize).collect();
    coins_usize.sort();

    let target_money_amount_usize = amount as usize;
    let mut coins_amount_holder: Vec<Option<i32>> = vec![None; target_money_amount_usize + 1];
    coins_amount_holder[0] = Some(0);

    for current_money_amount in 1..=target_money_amount_usize {
        for &current_coin in &coins_usize {
            if current_coin <= current_money_amount {
                if let Some(previous_coins_amount) =
                    coins_amount_holder[current_money_amount - current_coin]
                {
                    let candidate_to_best_coins_amount = previous_coins_amount + 1;

                    coins_amount_holder[current_money_amount] =
                        match coins_amount_holder[current_money_amount] {
                            Some(current_best_coins_amount) => {
                                if candidate_to_best_coins_amount < current_best_coins_amount {
                                    Some(candidate_to_best_coins_amount)
                                } else {
                                    Some(current_best_coins_amount)
                                }
                            }
                            None => Some(candidate_to_best_coins_amount),
                        }
                }
            }
        }
    }

    coins_amount_holder.last().unwrap_or(&None).unwrap_or(-1)
}
