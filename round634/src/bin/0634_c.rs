use std::collections::{BinaryHeap, HashMap};
use std::io::{self, BufRead};

// TODO: WA
fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();
    handle.read_line(&mut buf).unwrap();
    let t = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    let mut ans = Vec::with_capacity(t);

    'outer: for i in 0..t {
        handle.read_line(&mut buf).unwrap();
        let n = buf.trim().parse::<usize>().unwrap();
        buf.clear();

        let mut map = HashMap::new();
        let mut heap = BinaryHeap::new();

        handle.read_line(&mut buf).unwrap();
        for a in buf
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
        {
            *map.entry(a).or_insert(0) += 1;
        }

        buf.clear();

        if n == 1 {
            ans.push(0.to_string());
            continue;
        }

        for (k, v) in map.clone() {
            if v >= 2 {
                heap.push((v, k));
                map.remove(&k);
            }
        }

        while !heap.is_empty() {
            let (val, key) = heap.pop().unwrap();
            if map.len() >= val {
                ans.push(val.to_string());
                continue 'outer;
            }

            *map.entry(key).or_insert(0) += 1;

            let tmp = std::cmp::max(val - 1, heap.peek().unwrap_or(&(0, 0)).0);
            if map.len() >= tmp {
                ans.push(tmp.to_string());
                continue 'outer;
            }

            match heap.peek() {
                Some(&(v2, _)) if v2 > 1 => heap.push((v2 - 1, key)),
                _ => {
                    ans.push(1.to_string());
                    continue 'outer;
                }
            }
        }
        ans.push(1.to_string());
    }

    println!("{}", ans.join("\n"));
}
