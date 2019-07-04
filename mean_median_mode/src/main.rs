use std::collections::HashMap;
use rand::Rng;

#[derive(Debug)]
enum StatVal {
    Mean(f64),
    Median(i32),
    Mode(i32),
}

fn main(){
    let n = 10;
    let mut rng = rand::thread_rng();

    let mut vals: Vec<_> = (0..n).map(|_| rng.gen_range(0, 20)).collect();

    vals.sort();

    println!("{:?}",vals);

    let mut res = vec![
        StatVal::Mean(0.0),
        StatVal::Median(0),
        StatVal::Mode(0),
    ];

    let mut tot = 0;
    let mut counter = HashMap::new();

    for i in 0..n {
        tot += vals[i];
        if (i == (n/2 - 1) && n%2 == 0) || (i == n/2 && n%2 == 1) {
            res[1] = StatVal::Median(vals[i]);
        }
        let cnt = counter.entry(vals[i]).or_insert(0);
        *cnt += 1;
    }

    let mean = (tot as f64)/(n as f64);

    res[0] = StatVal::Mean(mean);

    let mode = counter
        .into_iter()
        .max_by_key(|&(_,count)| count)
        .map(|(val, _)| val)
        .expect("Zero Error");

    res[2] = StatVal::Mode(mode);

    for i in res {
        let _mean = StatVal::Mean;
        let _median = StatVal::Median;
        let _mode = StatVal::Mode;
        println!("{:?}",i);
    }
}