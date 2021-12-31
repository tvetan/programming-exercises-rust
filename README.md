# Programming exercises in Rust

For each exercise there will be a separate cargo package.

## Check exercise

```sh
    cd <exercise_folder>
    cargo test
```

## Create exercise

```sh
    cargo init <exercise_folder> && cd <exercise_folder>
    code .
```

## Init tests

Add to main.rs

```rust
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn should_return_correct() {}
    }
```
