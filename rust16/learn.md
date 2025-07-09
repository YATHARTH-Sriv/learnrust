Concurrent programming, 
    in which different parts of a program execute independently
    
Parallel programming, 
    in which different parts of a program execute at the same time



## Using Threads To Run Code:

    1 . How A Program Runs?

        Every time you run a program, your operating system creates a PROCESS for it.

        A process is like a box that contains:

        Your program’s code

        Its memory

        Resources (like file handles, network connections, etc.)

        Your OS (e.g., Windows, Linux, macOS) can run many processes at the same time.

        For example, you can open your browser, a music player, and a code editor all at once — each of these runs in a separate process.

    
    2. Thread:

        Even inside a single program, you might want to do multiple things at the same time.

        These "independent parts" that can run in parallel are called threads.

        A thread is like a mini-process inside a process.

        All threads in a process share the same memory and resources, but can run independently.

| Concept            | Meaning                                                                    |
| ------------------ | -------------------------------------------------------------------------- |
| **Process**        | A container for a running program. Managed by the OS.                      |
| **Thread**         | A smaller unit of execution within a process. Can run independently.       |
| **Multithreading** | Running multiple threads in a single process to do multiple tasks at once. |


 -- threads can run simultaneously, 
    there’s no inherent guarantee about the order in which parts of your code on different threads will run. 
    This can lead to problems, such as:

        Race conditions, in which threads are accessing data or resources in an inconsistent order
        Deadlocks, in which two threads are waiting for each other, preventing both threads from continuing
        Bugs that happen only in certain situations and are hard to reproduce and fix reliably


🧵 1:1 Threading Model (Used by Rust std::thread)

        1 Rust thread → 1 OS thread

        So if you have 20 tasks, you need 20 threads

        Each thread is heavyweight (costly in memory and CPU)

        Example: Good for CPU-intensive or long-running tasks

        🧠 Analogy: Hire 20 full-time workers for 20 jobs — expensive but simple and effective.

🔀 M:N Threading Model (Used by some libraries or runtimes)

        M OS threads ← N language-level tasks

        Example: 20 tasks running on just 5 OS threads

        Efficient and lightweight — better task scheduling

        Requires a runtime or library (e.g., rayon, tokio, smol, async-std)

        Great for I/O-bound workloads (web servers, file reads)

        🧠 Analogy: 5 smart workers rotating between 20 jobs — more efficient, but needs better coordination.



## Creating a New Thread with spawn

    To create a new thread, we call the thread::spawn function and pass it a closure 
    containing the code we want to run in the new thread

    Note that when the main thread of a Rust program completes,
    all spawned threads are shut down, whether or not they have finished running

    Solution ----> JoinHandle<T>

    A JoinHandle<T> is like a controller or handle to a running thread. It lets you:

        Wait for the thread to finish (using .join())

        Get the result returned by the thread
    

## Getting TO Know JoinHandle<T>:

    🔧 What is JoinHandle<T>?
        JoinHandle<T> is not an enum like Option or Result — 
        it’s a struct provided by Rust’s standard library in the std::thread module.

    
        use std::thread;

        fn main() {
            let handle: JoinHandle<i32> = thread::spawn(|| {
                42  // thread returns this
            });

            let result: Result<i32, Box<dyn std::any::Any + Send>> = handle.join();
            println!("Thread returned: {:?}", result.unwrap()); // prints 42
        }

When you call .join() on a JoinHandle<T>, it returns a Result<T, E>, where:

T is the value returned from the thread

E is the error (if the thread panicked)

Where  YOu do handle.join().unwrap()  aslo matters 

    ✅ 1. Does the position of .join() matter?
        Absolutely yes.

        The .join() method blocks (waits) until the spawned thread finishes. So where you call it determines when the main thread pauses to wait.



##  When We Need To Capture The Enviornment in closure:

    We use move to transfer ownership to closure 

    Problem : ?

            use std::thread;

            fn main() {
                let v = vec![1, 2, 3];

                let handle = thread::spawn(|| {
                    println!("Here's a vector: {v:?}"); // ← referencing `v`
                });

                handle.join().unwrap();
            }

            Compiler Says a slight possibility that v might get dropped in the main thread and maybe be the borrowed value does not exist then we will have a dangling error

    Solution

             use std::thread;

            fn main() {
                let v = vec![1, 2, 3];

                let handle = thread::spawn(move || {
                    println!("Here's a vector: {v:?}"); // ← referencing `v`
                });

                handle.join().unwrap();
            }
    With move we transfer ownership of the enviorment variables captured to the closure


## Message Parsing


        message passing, where threads or actors communicate by sending each other messages containing data.

        To accomplish message-sending concurrency, Rust’s standard library provides an implementation of channels.
        
#### Channels:

         A channel is a general programming concept by which data is sent from one thread to another.

         A channel has two halves: 
            - transmitter  
            - receiver

###### Transmitter:

        The transmitter half is the upstream location where you put the rubber duck into the river

###### Reciever:

         the receiver half is where the rubber duck ends up downstream

   A channel is said to be closed if either the transmitter or receiver half is dropped. 

### mpsc::channel

    mpsc stands for multiple producer, single consume

    means a channel can have multiple sending ends that produce values but only one receiving end that consumes those values.

    The mpsc::channel function returns a tuple, 
     - the first element of which is the sending end—the transmitter  tx
     - the second element of which is the receiving end—the receive   rx

#### Sending basically trasnmitting:
        use std::sync::mpsc;
        use std::thread;

        fn main() {
            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                let val = String::from("hi");
                tx.send(val).unwrap();
            });
        }

        spawned thread owns tx. 
        The spawned thread needs to own the transmitter to be able to send messages through the channel.

        1. The transmitter has a send method that takes the value we want to send
        2. The send method returns a Result<T, E> type,
        3. so if the receiver has already been dropped and there’s nowhere to send a value, the send operation will return an error

#### Recieving The Message:

    let received = rx.recv().unwrap();
    println!("Got: {received}");

    recv and try_recv:

| Method       | Description                                                                      |
| ------------ | -------------------------------------------------------------------------------- |
| `recv()`     | **Blocks** the thread until a message arrives or the channel is closed           |
| `try_recv()` | **Does not block**; returns immediately. If there's no message, returns an error |


#### Sending Multiple Values and Seeing the Receiver Waiting


        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;

        fn main() {
            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("thread"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    thread::sleep(Duration::from_secs(1));
                }
            });

            for received in rx {
                println!("Got: {received}");
            }
        }

        What is Happeing:

            1. the spawned thread has a vector of strings that we want to send to the main thread
            2.  We iterate over them, sending each individually, and pause between each by calling the thread::sleep function with a Duration value of one second.
            3. we’re treating rx as an iterator. For each value received, we’re printing it. 
            4. When the channel is closed, iteration will end.

### Mutliple Producers:

        code:

            let (tx, rx) = mpsc::channel();

            let tx1 = tx.clone();
            thread::spawn(move || {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("thread"),
                ];

                for val in vals {
                    tx1.send(val).unwrap();
                    thread::sleep(Duration::from_secs(1));
                }
            });

            thread::spawn(move || {
                let vals = vec![
                    String::from("more"),
                    String::from("messages"),
                    String::from("for"),
                    String::from("you"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    thread::sleep(Duration::from_secs(1));
                }
            });

            for received in rx {
                println!("Got: {received}");
            }

            // --snip--


    1. before we create the first spawned thread, we call clone on the transmitter.
    2. gives us a new transmitter we can pass to the first spawned thread. 
    3. We pass the original transmitter to a second spawned thread. 
    
    This gives us two threads, each sending different messages to the one receive


## Shared State Concurrency:
    
    






