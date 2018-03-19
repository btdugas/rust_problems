# rust_problems

# Problem 1

## Objective

This program will show me your skills with collections, iterators, and Rust's built-in testing functionality.

## Description

For this program you will create a function that receives a vector as a parameter and performs the following operations on the elements of the vector:
* If an element is divisible by two, divide the element by two.
* Otherwise, square the element (using Rust's built-in math functions).

The function must return a vector.

In addition you must:
* Write a driver program to demonstrate the function which prints out the vector using debug print after the function has executed.
* Write a test (that can be run with `cargo test`) that verifies your function works as intended.

## Other Requirements

* You must **not use any form of explicit loop**.
* For this problem, all of the code can be inside of `main.rs`.

# Problem 2

## Objective

This program will show me your skills with custom types, traits, and documentation comments.

## Description

First implement a `Car` type that contains three fields:
* `make`
* `model`
* `year`

These fields should be private.

Implement the `Display` trait for the `Car` type that writes the following string: *Vroom, Vroom, I'm in me mum's {year} {make} {model}!*

In addition you must:
* Write a driver program that prints a `Car`.
* Write documentation comments for the `Car` type as well as the entire crate that contains the `Car` type.
* Run the `cargo` command necessary to create the crate's documentation.

## Other Requirements

* The documentation comment for the `Car` type should contain an *Examples* section that contains sample code (using correct Markdown syntax) for creating a car.
* The folder structure of this problem should follow the standards described in the Rust book.

# Problem 3

## Objective

This program will show me your skills with threads, enums, ownership, and pattern matching.

## Description

For this program, you will create four threads that communicate using `mpsc`. Three of the threads will be producers and one thread, the `main` thread, will be a consumer. The producer threads will be sending `Messages` to the consumer.

You must implement a `Message` type as a tuple struct that contains an `Option<MessageType>`.

You must implement the enum `MessageType` to support two types of values:
* Signed Integers
* Strings

One of the producer threads should send at least 5 integers to the consumer. One of the producer threads should send at least 5 strings to the consumer. One of the producer threads should send `None` to the consumer five times.

When the consumer thread receives data, it should indicate what type of data it received, and it should print the data. The consumer thread should also indicate when it receives `None`.

## Other Requirements

* The folder structure of this problem should follow the standards described in the Rust book.

# Problem 4

## Objective

This problem will show me your skills utilizing smart pointers and generics.

## Description

For this problem you will be creating a very simple linked list implementation.

You will create two types:
* `List<T>`
* `Node<T>`

`List<T>` and `Node<T>` will contain a Link<T> which is a type alias: `type Link<T> = Option<Box<Node<T>>>`. `Node<T>` will also contain the field `value: T`.

After creating these types, you must implement a `push` and `pop` method on `List<T>`. `push` will add a new node to the front of the list. `pop` will pop a node off of the front of the list.

In addition you must:
* Write a driver program that verifies your linked list accepts multiple kinds of data and that your `push` and `pop` functions work correctly.

## Other Requirements

* The folder structure of this problem should follow the standards described in the Rust book.

# Problem 5

## Objective

This problem will show me your skills with the iron and juniper rust libraries. This problem will give you an example of how the core of KubOS works. 

## Description

Create a GraphQl server that accepts queries at the following URI: <http://localhost:8080/graphql>.

The query type accepted at the endpoint should:
* Have one field called `message`.
* Return the message *Hello GraphQL!* when queried.

You **DO NOT** need to implement a context factory.

You **DO NOT** need to implement a Mutation on the endpoint.

In addition you must:
* Test the GraphQL server API by submitting an *HTTP GET* request to <http://localhost:8080/graphql> and verifying that *Hello GraphQL!* is printed to the screen.

## Examples

Feel free to examine the following links for examples:
* <http://graphql.org/learn/>
* <https://github.com/kubos/kubos/blob/master/examples/rust-service/service/src/main.rs>
* <http://juniper.graphql.rs/>

## Other Requirements

* The folder structure of this problem should follow the standards described in the Rust book.
