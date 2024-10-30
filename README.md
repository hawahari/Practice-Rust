# Practice-rust
### Terminal commands
- cargo new  {},x
- cd {},x
- cargo build
- cargo run
### Variable and data types
![Screenshot from 2024-06-27 13-28-44](https://github.com/hawahari/Practice-rust/assets/149294262/1a250c4f-0763-45cc-a8e0-33f5495f969d)
This is the data type comparision in rust and other languages.</br>
let v :u32 = 3;</br>
let x :char = 'J';</br>
let y :&str = "Joseph";</br>
#### Mutable and immutable 
In order to mark a variable we want to change, we must apply mut before variable.Eg:</br>
let mut x=3;</br>
x=5;</br>
#### Data types
##### Scalar data types
###### Integers
- Integers can be 8 bits, 16 bits, 32 bits, 64 bits and 128 bits.
####### Integer Literals
- 
#### Operators
-  +
-  -
- /
- %
###### Floating points
###### Boolean
###### Characters
###### Compound types
####### Primitive compound types
######## Tuples
- have fixed length. Once declared cannot grow or shrink in size.
- let tup={1,true,"bye"};
- To access it: println!("{}",tup.2);
- let {x,y,z}=tup;
println!("{}",x);</br>
println!("{}",y);</br>
println!("{}",z);</br>
  Output</br>
1</br>
true</br>
bye</br>
######## Arrays
### Borrowing and ownership

