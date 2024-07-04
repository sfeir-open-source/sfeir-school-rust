<!-- .slide: class="sfeir-bg-blue-1" sfeir-level="2" sfeir-techno="rust" -->

# **Rust**

## **Les bases**

##==##

<!-- .slide: class="with-code" -->

# Les variables
## Mutabilité

```rust
let valueA = 1234;
valueA = 453; //ERROR !!!
```

<!-- .element: class="fragment" -->

```rust
let valueA = 1234;
let mut valueA = 453; 
```

<!-- .element: class="fragment" -->

## typage

<!-- .element: class="fragment" -->

```rust
let mut valueA = 1234;
valueA = 453.; //ERROR !!!
```

<!-- .element: class="fragment" -->

```rust
let valueA = 1234;
let mut valueA = 453; 
```

<!-- .element: class="fragment" -->

##==##

<!-- .slide: -->

# Les types
<ul>
  <li>Primitive:
    <ul>
      <li>Boolean — bool</li>
      <li>Numeric — entier(isize, i64,32,....), entier non signé (usize, u64, u32,...) et les flottant (f64, f32, ...) </li>
      <li>Textual — char, str et String</li>
      <li>Never </li>
    </ul>
  </li>
  <li>Sequence:
    <ul>
      <li>Tuple — (T, T) </li>
      <li>Tableau — [T, 8] </li>
      <li>Vecteur — Vec&lt;T&gt; </li>
      <li>Slice — [T] </li>
    </ul>
  </li>
  <li>Défini par l'utilisateur — Struct, Enum, Union</li>
  <li>Pointeur — &T et &mut T</li>
  <li>Functions </li>
  <li>Trait </li>

##==##

<!-- .slide: class="with-code" -->

# Les functions

```rust
// passage par valeur
fn add(a: i32, b: i32) -> i32 {
    a + b
}

let a = 1234;
let b = 453;
let result = add(a, b);
println!("{}+{}= {}", a,b, result); // 1234+453= 1687
```

##==##

<!-- .slide: class="with-code" -->

# L'ownership

```rust
{
  let s = String::from("hello");
} // libère s
 
```

##==##

<!-- .slide: class="with-code max-height" -->

# L'ownership

```rust
fn main() {
    let s = String::from("hello");
    change(s);
}

fn change(mut some_string: String) {
    some_string.push_str(", world");
} 
```

##==##

<!-- .slide: class="with-code max-height" -->

# L'ownership

```Rust
fn main() {
    let s = String::from("hello");
    change(s);
    println!("{}",s)
}

fn change(mut some_string: String) {
    some_string.push_str(", world");
}
// error[E0382]: borrow of moved value: `s`
//  --> src/main.rs:6:19
// 2 |     let s = String::from("hello");
//   |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
// 3 |
// 4 |     change(s);
//   |            - value moved here
// 5 |     
// 6 |     println!("{}",s)
//   |                   ^ value borrowed here after move
//   |
```

##==##

<!-- .slide: class="with-code max-height" -->

# L'ownership

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}",s)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

##==##

<!-- .slide: class="with-code max-height" -->

# L'ownership

```rust
// passage par valeur
fn add(a: i32, b: i32) -> i32 {
  a + b
}

let a = 1234;
let b = 453;
let result = add(a, b);
println!("{}+{}= {}", a,b, result); // 1234+453= 1687
```
##==##

<!-- .slide: class="with-code max-height" -->

# Les conditions

```rust
// pub enum Option<T> {
//     None,
//     Some(T),
// }

let mut valueC : Option<String> = None;
if valueC.is_some() {
    println!("condition was true");
} else {
    println!("condition was false");
}
```

##==##

<!-- .slide: class="with-code max-height" -->

# Les conditions

```rust
match valueC {
    Some(value) => println!("Value is {}", value),
    None => println!("No value"),
}
```

##==##

<!-- .slide: class="with-code max-height" -->

# Les boucles

```rust
let values = 0..10; // équivalent à [0,1,2,3,4,5,6,7,8,9] 
for i in values {
    println!("{}", i);
}

let mut i = 0;
while i < 10 {
    println!("{}", i);
    i += 1;
}

```
##==##

<!-- .slide: class="with-code max-height" -->

# Les itérateurs

```rust

fn main() {
  let vector: Vec<i32> = Vec::new();
  let another_vector = vec![1,2,3];

  let numbers: Vec<i32> = another_vector.iter().map(|i| i*2).collect();

  println!("{:?}", numbers);
}

```

