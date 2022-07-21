# 8. Commom collections

: contain multiple values & stored on the heap
-> don't need to known at compile time, able to change size

## 1. vector

같은 타입의 값 메모리 상에 나란히 저장

```rust
   let mut v:Vec<i32>= Vec::new();
   let v = vec![1,2,3];
   v.push(4);

   let does_not_exist = &v[100]; // panic
   let does_not_exist = v.get(100); // return None

   for i in &mut v {
    println!("{}", i);
    *i += 50;
   }

   enum Type {
    Int(i32),
    Text(String)
   }
   let row = vec![
    Type::Int(3),
    Type::Text(String::from("hi"))
    ];
```

## 2. string

```rust
let mut s = String::new();

let s = "initial contents".to_string();
let s = String::from("initial contents");

let mut s = String::from("foo");
s.push_str("bar");

let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);

let mut s = String::from("lo");
s.push('l');

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
let s = format!("{}-{}-{}", s1, s2, s3);
```

rust에서 String은 utf8 로 인코딩되는데, 한 캐릭터 당 2바이트씩 차지하므로 인덱스 하나로 접근하지 말고 범위로 접근해야 함

```rust

let hello = "Здравствуйте";

let s = &hello[0..4]; // 이건 오케이
//&hello[0..1] 이렇게 하면 오류남


```

## 3. hashmap

키끼리, 값끼리는 같은 타입이어야 함

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let teams = vec![
    String::from("Blue"),
    String::from("Yellow")
    ];
let initial_scores = vec![10, 50];
let mut scores: HashMap<_, _> =
    teams.into_iter().zip(initial_scores.into_iter()).collect();

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");
let mut map = HashMap::new();
map.insert(field_name, field_value);

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
let team_name = String::from("Blue");
let score = scores.get(&team_name); // get은 option리턴하므로 벗겨줘야 함

// 출력 순서는 랜덤임
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// entry는 해당 키가 있으면 그냥 넘어가고 없으면 or_insert()에 넘겨진 인자 삽입하고 그 값에 대한 mutable reference 리턴
let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

```

# 9.Error Hanlding

## 1. unrecoverable errors with panic!

```rust
fn main() {
    panic!("crash and burn");
}
```

setting the RUST_BACKTRACE environment variable to any value except 0
해당 오류가 나기까지 어떤 과정을 거쳤는지 자세히 보여줌

## 2. recoverable errors with Result

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) =>  match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
            }
    };
}


```

The unwrap method is a shortcut method implemented just like the match expression we wrote in Listing 9-4. If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us.

```rust
fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

the expect method lets us also choose the panic! error message. Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier.

```rust

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.

The ? operator can only be used in functions whose return type is compatible with the value the ? is used on.

## 3. to panic! or not to panic!
