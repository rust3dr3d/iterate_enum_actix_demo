# Iterate an Enum Demo

## Introduction
Imagine you have an enum called PaymentMethods defined. You want to send all variants of it as a JSON array so your front-end can list them through an API call as following..

![payement_method](screenshots/enum_variants_select.png)

You can use [Strum Crate](https://crates.io/crates/strum) to iterate over an enum! and much more!

This demo uses [Actix-Web](https://actix.rs/) to host a barebones web server. Using a GET request, we can acquire a JSON response as following...

 ![payment_json](screenshots/payment.png)

## Code screenshots

### Main.rs
![main_rs](screenshots/enum_iter_actix_1.png)

### Payment_method.rs
![payment_method_rs](screenshots/enum_iter_actix_2.png)

### Cargo.toml
![cargo_toml](screenshots/enum_iter_actix_3.png)