## வணக்கம் தமிழா!

இச்சமயம் நீங்கள் இரஸ்ட் நிருவி இரூப்பீர்கள், எனவே முதல் செயல்முறைத் திட்டம் ஒன்றை எழுதலாம்.
எந்த ஒரு மொழியை கற்று கொள்ளவும் முதலில் `Hello, world!` திட்டம் எழுதுவது வழக்கம்.  நாம்
மாறாக `வணக்கம் தமிழா!` அச்சிடுட் பார்போம்!

> Note: This book assumes basic familiarity with the command line. Rust makes
> no specific demands about your editing or tooling or where your code lives, so
> if you prefer to use an integrated development environment (IDE) instead of
> the command line, feel free to use your favorite IDE. Many IDEs now have some
> degree of Rust support; check the IDE’s documentation for details. Recently,
> the Rust team has been focusing on enabling great IDE support, and progress
> has been made rapidly on that front!

### ஒரு திட்டக் கோப்பகத்தை (directory) உருவாக்குதல்

முதலில் உங்கள் இரஸ்ட் நிரலை சேமிக்க ஒரு கோப்பகத்தை உருவாக்குவோம்.  இரஸ்ட் நிரல்கள்
எந்த ஒரு குரிப்பிட்ட கோப்பகத்திலும் உருவாக்கலாம், ஆனால் இந்த புத்தகத்தில் உள்ள
எடுத்துகாட்டுகளுக்கு *projects* என்ற ஒரு கோப்பை உருவாக்க நாங்கள் பரிந்துரைக்கிறோம்.

*projects* டைரக்டரி உருவாக்க உங்கள் இயக்க செயலியை பொருத்து பின் வரும் கட்டளைகளை
பயன் படுத்துங்கள்.

Linux and macOS - டெர்மினலில், பின் வரும் கட்டளைகளை பயன் படுத்தலாம்:

```text
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

Windows CMD - ல், பின் வரும் கட்டளைகளை பயன் படுத்தலாம்:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

Windows PowerShell - ல், பின் வரும் கட்டளைகளை பயன் படுத்தலாம்:

```powershell
> mkdir $env:USERPROFILE\projects
> cd $env:USERPROFILE\projects
> mkdir hello_world
> cd hello_world
```

### இரஸ்ட் திட்டத்தை எழுதுதல்

அடுத்த்தாக, *main.rs* என்ற ஒரு கோப்பை உருவாகுங்கள். இரஸ்ட் கோப்புகள் எப்பொலுதும் *.rs*
எக்ஸ்டன்சனில் சேமிக படும். பல சொற்க்கள் உங்கள் கோப்பின் பெயரில் பயன் படுத்தினால் அவற்றை
அடிக்கொட்டு எழுத்தை (underscore) பயன்படுத்தி பிரித்து பயன் படுத்துள்கள். உதாரனமா
*helloworld.rs* என்று குறிக்காமல் *hello_world.rs* என்று பயன் படுத்துங்கள்.

இப்பொழுது நீங்கள் உருவாக்கிய *main.rs* கோப்பில் 1-1 குறியிட்ட பகுதில் உள்ள கட்டளைகளை
அச்சிட்டு சேமியுங்கள்.

<span class="filename">Filename: main.rs</span>

```rust
fn main() {
    println!("வணக்கம் தமிழா!");
}
```

<span class="caption">Listing 1-1: `வணக்கம் தமிழா!` என்று அச்சிடும் திட்டம்</span>

சேமித்த உடன் Linux or macOS டேர்மினலில் பின்வரும் கட்டளைகள் பயன் படுத்தி கம்பைல் செய்யவும்:

```text
$ rustc main.rs
$ ./main
வணக்கம் தமிழா!
```

ஒருவேலை Windows பயன் படுத்தினால்,`./main` -க்கு பதில் `.\main.exe` என்று பயன் படுத்துங்கள்:

```powershell
> rustc main.rs
> .\main.exe
வணக்கம் தமிழா!
```
எந்த வகையான செயலியாக இருப்பினும் `வணக்கம் தமிழா!` என்ற வாசகம் அச்சாகி இருக்கும் இல்லை
என்றால், "பழுது நீக்கும்" பக்கதினை பயன் படுத்தி பழுதை நீக்க உதவி பெரலாம்..

`வணக்கம் தமிழா!` அச்சிட பட்டுவிட்டால் வாழ்த்துக்கள் நீங்கள் ஒரு சரியான இரஸ்ட் திட்டத்தை
உருவாக்கி விட்டீர்கள். இது உங்களுக்கு ஒரு நல்ல துவக்கம்!

### இரஸ்ட் திட்டத்தின் உடற்கூற்றியல்

Let’s review in detail what just happened in your Hello, world! program.
Here’s the first piece of the puzzle:

```rust
fn main() {

}
```

These lines define a function in Rust. The `main` function is special: it is
always the first code that runs in every executable Rust program. The first
line declares a function named `main` that has no parameters and returns
nothing. If there were parameters, they would go inside the parentheses, `()`.

Also, note that the function body is wrapped in curly brackets, `{}`. Rust
requires these around all function bodies. It’s good style to place the opening
curly bracket on the same line as the function declaration, adding one space in
between.

At the time of this writing, an automatic formatter tool called `rustfmt` is
under development. If you want to stick to a standard style across Rust
projects, `rustfmt` will format your code in a particular style. The Rust team
plans to eventually include this tool with the standard Rust distribution, like
`rustc`. So depending on when you read this book, it might already be installed
on your computer! Check the online documentation for more details.

Inside the `main` function is the following code:

```rust
    println!("Hello, world!");
```

This line does all the work in this little program: it prints text to the
screen. There are four important details to notice here. First, Rust style is
to indent with four spaces, not a tab.

Second, `println!` calls a Rust macro. If it called a function instead, it
would be entered as `println` (without the `!`). We’ll discuss Rust macros in
more detail in Appendix D. For now, you just need to know that using a `!`
means that you’re calling a macro instead of a normal function.

Third, you see the `"Hello, world!"` string. We pass this string as an argument
to `println!`, and the string is printed to the screen.

Fourth, we end the line with a semicolon (`;`), which indicates that this
expression is over and the next one is ready to begin. Most lines of Rust code
end with a semicolon.

### Compiling and Running Are Separate Steps

You’ve just run a newly created program, so let’s examine each step in the
process.

Before running a Rust program, you must compile it using the Rust compiler by
entering the `rustc` command and passing it the name of your source file, like
this:

```text
$ rustc main.rs
```

If you have a C or C++ background, you’ll notice that this is similar to `gcc`
or `clang`. After compiling successfully, Rust outputs a binary executable.

On Linux, macOS, and PowerShell on Windows, you can see the executable by
entering the `ls` command in your shell as follows:

```text
$ ls
main  main.rs
```

With CMD on Windows, you would enter the following:

```cmd
> dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
```

This shows the source code file with the *.rs* extension, the executable file
(*main.exe* on Windows, but *main* on all other platforms), and, when using
CMD, a file containing debugging information with the *.pdb* extension. From
here, you run the *main* or *main.exe* file, like this:

```text
$ ./main # or .\main.exe on Windows
```

If *main.rs* was your Hello, world! program, this line would print `Hello,
world!` to your terminal.

If you’re more familiar with a dynamic language, such as Ruby, Python, or
JavaScript, you might not be used to compiling and running a program as
separate steps. Rust is an *ahead-of-time compiled* language, meaning you can
compile a program and give the executable to someone else, and they can run it
even without having Rust installed. If you give someone a *.rb*, *.py*, or
*.js* file, they need to have a Ruby, Python, or JavaScript implementation
installed (respectively). But in those languages, you only need one command to
compile and run your program. Everything is a trade-off in language design.

Just compiling with `rustc` is fine for simple programs, but as your project
grows, you’ll want to manage all the options and make it easy to share your
code. Next, we’ll introduce you to the Cargo tool, which will help you write
real-world Rust programs.
