## Stack and Heap Memory

    Stack Memory less compute power used faster efficient 

    Heap Memory More Compute Less Efficient String Vectors 

    LIFO

    Garbage Allocater and in other languages you have to do the problem ownership solves


## Ownership Rules 

![alt text](image.png)

## Literal and String::from("")

    let s="hello" // This is on Stack Memory

    let s=String::from("Hello")  // On Heap


## Shallow Copy 

![alt text](image-1.png)

# Move Rather Than Shallow Copy

![alt text](image-2.png)

# Deep Copy

![alt text](image-3.png)


# Copy Trait

![alt text](image-4.png)

# Clone Trait

![alt text](image-5.png)

# Copy Trait is For Only For Stack Memory Data Types

![alt text](image-6.png)



# Without Transferring ownership Referencing

One mutable reference then no more refernce possible
Multiple Immutable reference possible

![alt text](image-7.png)

# Mutable Referencing

    let mut x= String::from("hello")

    let r1= &x --------> Immutable Reference
    let r2= &mut x -----------> Mutable Reference 

# Dangling Pointers

![alt text](image-8.png)


# S.as_bytes(); .as_bytes

    .as_bytes()
    
    This converts a String (or &str) into a byte slice: &[u8].

    b' ' is a byte literal (u8), value 32

![alt text](image-9.png)


![alt text](image-10.png)

# String Slices

![alt text](image-11.png)
