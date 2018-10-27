## வணக்கம், சரக்கு!

சரக்கு என்பது இரஸ்ட்டின் கட்டட அமைப்பு மற்றும் தொகுப்பு மேலாளர். பெரும்பாலான இரஸ்டாசன்கள் இந்த கருவியை பயன்படுத்துகின்றனர்
சரக்குகள் உங்கள் நிறைய பணிகளை கையாள்கின்றன என்பதால்,இரஸ்ட் திட்டங்களை நிர்வகிக்க,
உங்கள் குறியீட்டை உருவாக்குவது, உங்கள் குறியீடு சார்ந்து நூலகங்கள் பதிவிறக்கம் செய்வது
, மற்றும் நூலகங்களை உருவாக்குவது போன்ற பணிகளை செய்கின்றன. 
 (உங்கள் குறியீடு தேவை * நூலகங்கள் * சார்புகளை நூலகங்கள் அழைக்கிறோம்.)
(We call libraries your code needs *dependencies*.)
எளிமையான இரஸ்ட் நிகழ்ச்சிகள், நாம் இதுவரை எழுதியுள்ளதைப் போலவே
சார்புநிலைகளாகவும் எதுவும் குறிப்பிடப்படவில்லை. நாம் ஹலோ, உலகத்தை கட்டியிருந்தால்! சரக்கு திட்டத்தின் மூலம், அது உங்கள் குறியீட்டை உருவாக்கும் கர்கோவின் பகுதியை மட்டும் பயன்படுத்துங்கள். 
நீங்கள் இன்னும் எழுதும்போது சிக்கலான இரஸ்ட் திட்டத்தின் மூலம், நீங்கள் சார்புகளை சேர்க்க வேண்பெரும்பாலான இரஸ்ட் திட்டங்களில் சரக்குகள், மற்றும் புத்தகத்தின் மற்ற பகுதிகள் பயன்படுத்தப்படுகின்றன
நீங்கள் கூட சரக்கு பயன்படுத்த வேண்டும் என்று கருதுகிறது. நீங்கள் என்றால் ரஸ்ட் கொண்டு சரக்கு நிறுவப்பட்டது
"நிறுவல்" பிரிவில் விவாதிக்கப்பட்ட அதிகாரப்பூர்வ நிறுவிகளைப் பயன்படுத்தினார். நீங்கள் என்றால்
வேறு சில வழிகளிலும் ரஸ்ட் நிறுவப்பட்டது, சரக்கு நிறுவப்பட்டதா என சரிபார்க்கவும்

உங்கள் முனையத்தில் கீழ்க்கண்டவற்றை உள்ளிடுக:டும், மற்றும் நீங்கள் ஒரு திட்டத்தை ஆரம்பித்தால்
சரக்கு பயன்படுத்தி, சார்புகளை சேர்ப்பது மிகவும் எளிதாக இருக்கும்.

```text
$ cargo --version
```

நீங்கள் ஒரு பதிப்பு எண்ணைக் கண்டால், உங்களிடம் இது உள்ளது! `கட்டளை போன்ற ஒரு பிழை நீங்கள் கண்டால் அது கட்டளை`, உங்கள் நிறுவல் முறையை ஆவணத்தில் பாருங்கள்
தனித்தனியாக சரக்கு நிறுவ எப்படி தீர்மானிக்க.

### சரக்கு மூலம் ஒரு திட்டத்தை உருவாக்குதல்
சரக்கு பயன்படுத்தி ஒரு புதிய திட்டத்தை உருவாக்குவோம் மற்றும் அது வணக்கம், உலகம்! என்ற திட்டத்திலிருந்து எப்படி வேறுபடுகிறது என்று பார்ப்போம்.
உங்களின்  திட்ட இயக்கத்தை  அடைவுக்குள் திரும்ப பெறவும் (அல்லது
எங்கு உங்கள் குறியீட்டை சேமித்து வைக்க முடிவு செய்தீர்கள்). பின்னர், எந்த இயக்க முறைமையிலும், பின்வருவதை இயக்கவும்:	
```text
$ cargo new hello_cargo --bin
$ cd hello_cargo
```
முதல் கட்டளை, * hello_cargo * என்ற புதிய பைனரி இயங்கமைப்பு உருவாக்குகிறது.`-bin`ஆர்குயுமன்டை `cargo new`ற்கு அனுப்பி ஒரு நிறைவேற்றக் கூடிய பயன்பாட்டை
ஒரு நூலகத்திற்கு பதிலாக உருவாக்குவோம்(அடிக்கடி
ஒரு பைனரி * என்று அழைக்கப்படுகிறது) ஒரு நூலகத்திற்கு பதிலாக. எங்கள் திட்டத்தை* hello_cargo * என்று பெயரிட்டோம், மற்றும் சரக்கு அதன் கோப்புகளை அதே பெயரின் அடைவில் உருவாக்குகிறது.

Because the vast majority of Rust projects use Cargo, the rest of this book
assumes that you’re using Cargo too. Cargo comes installed with Rust if you
used the official installers discussed in the “Installation” section. If you
installed Rust through some other means, check whether Cargo is installed by
entering the following into your terminal:

```text
$ cargo --version
```

If you see a version number, you have it! If you see an error, such as `command
not found`, look at the documentation for your method of installation to
determine how to install Cargo separately.


Hello_cargo * கோப்பறையில் சென்று கோப்புகளை பட்டியலிடுங்கள். நீங்கள் சரக்கை பார்க்கும் போது,சரக்கு இரண்டு கோப்புகள் மற்றும் ஒரு அடைவு உருவாக்கியது: a * Cargo.toml * கோப்பு மற்றும் ஒரு
* src * கோப்பினை உள்ளே * main.rs * கோப்பில் கொண்டிருக்கும். இது ஒரு புதிய கிட் துவக்கி
* .gitignore * கோப்பினை இணைக்கும் களஞ்சியத்தையும் கொண்டிருக்கும்.

குறிப்பு: Git ஒரு பொது பதிப்பு கட்டுப்பாட்டு அமைப்பு. நீங்கள் `cargo new` என்று மாற்றலாம் வேறுபட்ட பதிப்பு கட்டுப்பாட்டு அமைப்பு அல்லது பதிப்பு கட்டுப்பாட்டு முறைமையைப் பயன்படுத்துவதன் மூலமாக
  `-vcs` கொடி பயன்படுத்தலாம். ஏற்கனவே இருக்கும் விருப்பங்களைக் காண `சரக்கு புதிய --help` ஐ இயக்கவும்.

  
   உங்கள் உரை ஆசிரியரில் * Cargo.toml * தேர்வு செய்து திறக்கவும். அது போலவே பட்டியல் 1-2 வேண்டும்.

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
```

<span class="caption">Listing 1-2:* Cargo.toml இன் பொருளடக்கம் * சரக்கு மூலம் உருவாக்கப்பட்டது</span>

இந்த கோப்பு [* TOML *] [toml] இல் உள்ளது <! --ignore--> (* டாம்'ஸ் வெளிப்படையான, குறைந்தபட்சம்
மொழி *) வடிவம், இது கார்கோவின் கட்டமைப்பு வடிவமாகும்.

[toml]: https://github.com/toml-lang/toml

`[தொகுப்பு]`,இதன் முதல் வரி ஒரு பகுதி தலைப்பு என்று குறிக்கிறது
பின்வரும் அறிக்கைகள் ஒரு தொகுப்பை கட்டமைக்கிறது. மேலும் தகவலைச் சேர்ப்பது போல
இந்த கோப்பு, நாம் பிற பிரிவுகளை சேர்க்க வேண்டும்.


அடுத்த மூன்று கோடுகள் கார்கோவை தொகுக்க வேண்டிய கட்டமைப்பு தகவலை அமைக்கும் திட்டத்தை : பெயர், பதிப்பு, மற்றும் அது யார் எழுதியது என்பதை செய்கிறது.
 சரக்கு உங்கள் பெயர் மற்றும் மின்னஞ்சல் தகவலை  உங்கள் சூழலில் இருந்து பெறுகிறது , அந்த தகவல் சரியாக இல்லை என்றால், தகவலை சரிசெய்து பின்னர் கோப்பை சேமிக்கவும்.


The last line, `[dependencies]`, is the start of a section for you to list any
of your project’s dependencies. In Rust, packages of code are referred to as
*crates*. We won’t need any other crates for this project, but we will in the
first project in Chapter 2, so we’ll use this dependencies section then.

Now open *src/main.rs* and take a look:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo has generated a Hello, world! program for you, just like the one we wrote
in Listing 1-1! So far, the differences between our previous project and the
project Cargo generates are that Cargo placed the code in the *src* directory,
and we have a *Cargo.toml* configuration file in the top directory.

Cargo expects your source files to live inside the *src* directory. The
top-level project directory is just for README files, license information,
configuration files, and anything else not related to your code. Using Cargo
helps you organize your projects. There’s a place for everything, and
everything is in its place.

If you started a project that doesn’t use Cargo, as we did with the Hello,
world! project, you can convert it to a project that does use Cargo. Move the
project code into the *src* directory and create an appropriate *Cargo.toml*
file.

### Building and Running a Cargo Project

Now let’s look at what’s different when we build and run the Hello, world!
program with Cargo! From your *hello_cargo* directory, build your project by
entering the following command:

```text
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

This command creates an executable file in *target/debug/hello_cargo* (or
*target\debug\hello_cargo.exe* on Windows) rather than in your current
directory. You can run the executable with this command:

```text
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```

If all goes well, `Hello, world!` should print to the terminal. Running `cargo
build` for the first time also causes Cargo to create a new file at the top
level: *Cargo.lock*. This file keeps track of the exact versions of
dependencies in your project. This project doesn’t have dependencies, so the
file is a bit sparse. You won’t ever need to change this file manually; Cargo
manages its contents for you.

We just built a project with `cargo build` and ran it with
`./target/debug/hello_cargo`, but we can also use `cargo run` to compile the
code and then run the resulting executable all in one command:

```text
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Notice that this time we didn’t see output indicating that Cargo was compiling
`hello_cargo`. Cargo figured out that the files hadn’t changed, so it just ran
the binary. If you had modified your source code, Cargo would have rebuilt the
project before running it, and you would have seen this output:

```text
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo also provides a command called `cargo check`. This command quickly checks
your code to make sure it compiles but doesn’t produce an executable:

```text
$ cargo check
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Why would you not want an executable? Often, `cargo check` is much faster than
`cargo build`, because it skips the step of producing an executable. If you’re
continually checking your work while writing the code, using `cargo check` will
speed up the process! As such, many Rustaceans run `cargo check` periodically
as they write their program to make sure it compiles. Then they run `cargo
build` when they’re ready to use the executable.

Let’s recap what we’ve learned so far about Cargo:

* We can build a project using `cargo build` or `cargo check`.
* We can build and run a project in one step using `cargo run`.
* Instead of saving the result of the build in the same directory as our code,
  Cargo stores it in the *target/debug* directory.

An additional advantage of using Cargo is that the commands are the same no
matter which operating system you’re working on. So, at this point, we’ll no
longer provide specific instructions for Linux and macOS versus Windows.

### Building for Release

When your project is finally ready for release, you can use `cargo build
--release` to compile it with optimizations. This command will create an
executable in *target/release* instead of *target/debug*. The optimizations
make your Rust code run faster, but turning them on lengthens the time it takes
for your program to compile. This is why there are two different profiles: one
for development, when you want to rebuild quickly and often, and another for
building the final program you’ll give to a user that won’t be rebuilt
repeatedly and that will run as fast as possible. If you’re benchmarking your
code’s running time, be sure to run `cargo build --release` and benchmark with
the executable in *target/release*.

### Cargo as Convention

With simple projects, Cargo doesn’t provide a lot of value over just using
`rustc`, but it will prove its worth as your programs become more intricate.
With complex projects composed of multiple crates, it’s much easier to let
Cargo coordinate the build.

Even though the `hello_cargo` project is simple, it now uses much of the real
tooling you’ll use in the rest of your Rust career. In fact, to work on any
existing projects, you can use the following commands to check out the code
using Git, change to that project’s directory, and build:

```text
$ git clone someurl.com/someproject
$ cd someproject
$ cargo build
```

For more information about Cargo, check out [its documentation].

[its documentation]: https://doc.rust-lang.org/cargo/

## Summary

You’re already off to a great start on your Rust journey! In this chapter,
you’ve learned how to:

* Install the latest stable version of Rust using `rustup`
* Update to a newer Rust version
* Open locally installed documentation
* Write and run a Hello, world! program using `rustc` directly
* Create and run a new project using the conventions of Cargo

This is a great time to build a more substantial program to get used to reading
and writing Rust code. So, in Chapter 2, we’ll build a guessing game program.
If you would rather start by learning how common programming concepts work in
Rust, see Chapter 3 and then return to Chapter 2.
