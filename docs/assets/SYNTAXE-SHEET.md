
<!-- .slide: class="with-code" -->

# SYNTAXE SHEET

```rust
let valueA = 1234; // Immutable
let mut valueB = 453; // Mutable
let mut valueC : Option<String> = None;

// Conditions
match valueC {
    Some(value) => println!("Value is {}", value),
    None => println!("No value"),
}
// or
if valueC.is_some() {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

// Loop
for i in 0..10 {
    println!("{}", i);
}

while valueB < 1000 {
    valueB += 1;
}

// Function
fn add(a: i32, b: i32) -> i32 {
    a + b
}

let result = add(1234, 453);


//STRUCT
[#derive(Debug)]
struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(value: i32) -> MyStruct {
        MyStruct { value }
    }
    
    add(&self, b: MyStruct) -> i32 {
        self.value + b.value
    }
}

let mut my_struct = MyStruct::new(1234);
let my_struct2 = MyStruct::new(453);
println!("{}", my_struct);
my_struct.add(my_struct2);
println!("{}", my_struct); 

```
