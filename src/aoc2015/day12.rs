// --- Day 12: JSAbacusFramework.io ---
//
// Santa's Accounting-Elves need help balancing the books after a recent order. Unfortunately, their accounting software uses a peculiar storage format. That's where you come in.
//
// They have a JSON document which contains a variety of things: arrays ([1,2,3]), objects ({"a":1, "b":2}), numbers, and strings. Your first job is to simply find all of the numbers throughout the document and add them together.
//
// For example:
//
// [1,2,3] and {"a":2,"b":4} both have a sum of 6.
// [[[3]]] and {"a":{"b":4},"c":-1} both have a sum of 3.
// {"a":[-1,1]} and [-1,{"a":1}] both have a sum of 0.
// [] and {} both have a sum of 0.
// You will not encounter any strings containing numbers.
//
// What is the sum of all numbers in the document?
//
extern crate serde_json;


pub fn solve(input: &String) -> Vec<Result<i64, String>> {
    let data: serde_json::Value = serde_json::from_str(input).unwrap();
    let p1 = sum(data.clone());
    let p2 = sum_red(data);
    vec![Ok(p1), Ok(p2)]
}

fn sum(data: serde_json::Value) -> i64 {
    match data {
        serde_json::Value::I64(i) => i,
        serde_json::Value::U64(u) => u as i64,
        serde_json::Value::Array(arr) => {
            let mut s = 0;
            for v in arr {
                s += sum(v);
            }
            s
        }
        serde_json::Value::Object(obj) => {
            let mut s = 0;
            for v in obj.values() {
                s += sum(v.clone());
            }
            s
        }
        _ => 0,
    }
}

fn sum_red(data: serde_json::Value) -> i64 {
    match data {
        serde_json::Value::I64(i) => i,
        serde_json::Value::U64(u) => u as i64,
        serde_json::Value::Array(arr) => {
            let mut s = 0;
            for v in arr {
                s += sum_red(v);
            }
            s
        }
        serde_json::Value::Object(obj) => {
            if obj.contains_key("red") {
                0
            } else {
                let mut s = 0;
                let vals: Vec<serde_json::Value> = obj.values().cloned().collect();
                for v in vals {
                    match v {
                        serde_json::Value::String(ref s) if s == "red" => return 0,
                        _ => s += sum_red(v),
                    }
                }
                s
            }
        }
        _ => 0,
    }
}
