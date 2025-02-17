# Practice-rust
### Terminal commands
- cargo new  {},x
- cd {},x
- cargo build
- cargo run
- cargo new projectname, creates a folder in the name of the project.
- In the new directory, a folder named src and cargo.toml
- src contains file main.rs
- snake casing in rust(lowercases and underscores).
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
#### Operators
- (plus)+
- (minus)-
- /
- %
###### Floating points
###### Boolean
###### Characters
##### Compound types
###### Primitive compound types
###### Tuples
- different data types.
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

###### Arrays
###### Vector
- dynamic array.
- pop and push to remove and add.
- let mut nums=vec!{1,2,3};
  nums.push(4);</br>//add number 4
  println!({:?},nums);//debug mode
### Functions
- fn to create a function.
- calling same as C.
- For function with parameters it is almost same. Declare variable.
- For function which will return.
### Borrowing and ownership
- Stack and heap differences.
- Each value has a owner.
- There can only be one owner.
- Ownerhip can be transferred.
- When the value's owner goes out of scope, it is dropped or freed.
### Structures and enums
#### Structures
- three types of structures.
- Name field
- Tuple
- Unit structures
#### Methods
- Methods are similar to functions.
- They are declared with fn keyword and name. They take parameters and return values much like functions.
- Keyword: impl
#### Lifetime
- Every refernce has lifetime.
- If no refrence then, no lifetime is required.
- Most of the time lifetime are implicit and inferred,
- Lifetime to prevent dangling refrences.
