## <img src="assets/images/ferris.png" width="100" height="68" /> Proc Macro Rules!

Azriel Hoh

---

### Agenda

---

### Preamble: `macro_rules!`

+++

### Preamble: `macro_rules!`

```rust
macro_rules! hello {
    () => { println!("Hello") };
}

fn main() {
    hello!();
    hello![];
    hello! {};

    println! {
        "{}{}{}{} {}{}", 
        "\u{69}",
        "\u{74}",
        "\u{27}",
        "\u{73}",
        "\u{6d}",
        "\u{65}",
    };
}
```

+++

### Preamble: `macro_rules!`

`macro_rules!` is happy to take any token tree:

```rust
macro_rules! java {
    (static void $name:ident($($_:tt)+) { $($body:tt)+ }) => {
        fn $name() { java!($($body)+); }
    };

    ($_:ident.$__:ident.$fn_name:ident($args:tt);) => {
        println!($args);
    };
}

java! {
    static void main(String[] args) {
        System.out.println("jRust!");
    }
}

#[cfg(test)]
fn main() {}
```

+++

### Preamble: `macro_rules!`

However:

* Very unintuitive failures and errors.
* Can be very difficult to troubleshoot.

```rust
macro_rules! java {
    (static void $name:ident($($_:tt)+) $body:block) => {
        fn $name() { java!($body); }
        //                 ^^^^^ no rules expected this token in macro call
    };

    ({ $_:ident.$__:ident.$fn_name:ident($args:tt); }) => {
        println!($args);
    };
}

java! {
    static void main(String[] args) {
        System.out.println("jRust!");
    }
}

#[cfg(test)]
fn main() {}
```

---

### Proc Macros

+++

### What Are Proc Macros?

Macros defined with procedural code.

Tokens are parsed into an AST, and code can reason over that generate the output.

+++

### What Are Proc Macros?

```rust
// Any of:
function_like!("Indistinguiable from `macro_rules!`");
function_like! {
    "Indistinguiable from `macro_rules!`"
}

#[derive(Debug, CustomDerive)]
struct Hello;

#[custom_attribute]
fn do_something() {}
```

+++

### Why Proc Macros?

For users:

* 🎨 Better error messages.
* 📓 Nicer syntax.

+++

### Why Proc Macros?

For developers:

* 📦 Dedicated crate.
* 🌲 Parsing AST instead of matching patterns.
* 🔺 Somewhat better diagnostics.
* 💯 Easier to test.

---

### Function Like

+++

### Function Like

1. Takes in *any* well-formed tokens.
2. Outputs other tokens that are compiled.

+++

### Function Like

**Demo:** see `function_like` directory in repository.

1. 

---

### Links

* @size[0.7em](Slides: https://github.com/azriel91/proc_macro_rules)
* @size[0.7em](Blog: https://blog.rust-lang.org/2018/12/21/Procedural-Macros-in-Rust-2018.html)
* @size[0.7em](Reference: https://doc.rust-lang.org/reference/procedural-macros.html)
* @size[0.7em](syn: https://github.com/dtolnay/syn)
* @size[0.7em](macro_rules!: https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html)

---

### Questions, Answers and Comments
