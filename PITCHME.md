## <img src="assets/images/ferris.png" width="100" height="68" /> Proc Macro Rules!

Azriel Hoh

---

### Agenda

---

### Preamble: `macro_rules!`

+++

### Preamble: `macro_rules!`

[<img src="assets/images/ferris.png" width="50" height="33" /> playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=d1d52c0a2536f9f121b5f7dd9197d5bb)

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
        "\u{69}", "\u{74}", "\u{27}",
        "\u{73}", "\u{6d}", "\u{65}",
    };
}
```

+++

### Preamble: `macro_rules!`

`macro_rules!` is happy to take any token tree [<img src="assets/images/ferris.png" width="50" height="33" />](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=d1d52c0a2536f9f121b5f7dd9197d5bb):

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
```

+++

### Preamble: `macro_rules!`

However [<img src="assets/images/ferris.png" width="50" height="33" />](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=d1d52c0a2536f9f121b5f7dd9197d5bb):

* Very unintuitive failures and errors.
* Can be very difficult to troubleshoot.

```rust
macro_rules! java {
    (static void $name:ident($($_:tt)+) $body:block) => {
        fn $name() { java!($body); }
                        // ^^^^^ no rules expected this token in macro call
    };

    ({ $_:ident.$__:ident.$fn_name:ident($args:tt); }) => {
        println!($args);
    };
}
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

**Demo:** see `function_like` crate in repository.

1. Parsing.
2. Errors.
3. Traversing.

---

### Derive Macros

+++

### Derive Macros

1. Attached to a struct / enum.
2. Generates *additional* tokens.
3. Can have *helper* attributes.
3. **Cannot** see other attributes / derives.

+++

### Derive Macros

Given the following:

```rust
/// Documentation for a struct.
#[derive(Clone, CustomDerive, SomeoneElsesDerive, Debug)]
#[custom_derive_helper_1]
#[someone_elses_attr]
pub struct Struct {
    #[custom_derive_helper_2(value = "Something")]
    pub field: Type,
}
```

+++

### Derive Macros

Your macro will see:

```rust
// Missing: Clone, SomeoneElsesDerive, Debug
// #[someone_elses_attr]

#[derive(CustomDerive)]
#[doc = "Documentation for a struct."]
#[custom_derive_helper_1]
pub struct Struct {
    #[custom_derive_helper_2(value = "Something")]
    pub field: Type,
}
```

+++

### Derive Macros

What you can do:

* Generate `impl` blocks -- normal / trait impls.
* Generate more `type`s -- `struct`s and `enum`s.

What you **can't** do:

* Mutate the `type`, e.g. add fields, documentation.

+++

### Derive Macros

**Demo:** see `derive_mode` crate in repository.

1. Parsing.
2. Erring.
3. `quote`: Variables must be single layer.
4. `cargo expand --test test inner`

---

> **Remember:** Don't be too caught up in questioning
> what you could do, that you forget to question if
> it's something you should do.

---

### Links

* @size[0.7em](Slides: https://github.com/azriel91/proc_macro_rules)
* @size[0.7em](Blog: https://blog.rust-lang.org/2018/12/21/Procedural-Macros-in-Rust-2018.html)
* @size[0.7em](Reference: https://doc.rust-lang.org/reference/procedural-macros.html)
* @size[0.7em](syn: https://github.com/dtolnay/syn)
* @size[0.7em](macro_rules!: https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html)

---

### Questions, Answers and Comments
