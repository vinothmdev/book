# ஒரு யோசனை விளையாட்டு நிரலாக்க

ஒன்றாக கைகளில் வேலை செய்யும் திட்டம் மூலம் ரஸ்டில் குதிக்கலாம்! இந்த
அத்தியாயத்தில் ஒரு சில பொதுவான ரஸ்டின் கருத்துக்கள் எப்படி உண்மையான திட்டத்தில் பயன்படுத்தலாம்  என்று அறிமுகப்படுத்துகிறது.
 'Let`, `match`, முறைகள், தொடர்புடையவை  செயல்பாடுகள், வெளிப்புற கோடுகள் மற்றும் பல  கருத்துகள் பற்றி நீங்கள் அறிந்துகொள்வீர்கள்! 
 பின்வரும் அத்தியாயங்கள் மேலும் விரிவான கருத்துக்களை ஆராயும்.
 இந்த அத்தியாயத்தில், நீங்கள் அடிப்படைகளை பயிற்சி செய்வீர்கள்.

ஒரு கற்பனை விளையாட்டு: நாங்கள் ஒரு சிறந்த தொடக்க நிரலாக்க சிக்கலை செயல்படுத்த வேண்டும். இங்கே
அது எப்படி வேலை செய்கிறது: திட்டம் 1 மற்றும் 100 க்கு இடையில் சீரற்ற முழுமையையும் உருவாக்கும்
பின்னர் வீரர் ஒரு யூகத்தை உள்ளிடுமாறு கேட்கும். ஒரு யூகம் நுழைந்தவுடன், அந்த
நிரல் கணிப்பு மிகவும் குறைவாகவோ அல்லது அதிகமாகவோ உள்ளதா என்பதைக் குறிக்கும். 
யூகம் சரியானது என்றால், விளையாட்டு வாழ்த்து செய்தியை அச்சிடும் மற்றும் வெளியேறும்.

## ஒரு புதிய திட்டத்தை அமைத்தல்

புதிய திட்டத்தை பாடம் 1ல் அமைக்க, நீங்கள் உருவாக்கிய * திட்டங்கள் * அடைவுக்குச் செல்லவும்
 மற்றும் கார்கோ பயன்படுத்தி ஒரு புதிய திட்டம் செய்யவும்:
 
```text
$ cargo new guessing_game --bin
$ cd guessing_game
```

முதல் கட்டளையான `cargo new` என்பதை, இந்த திட்டத்தின் பெயரை(`guessing game`) முதல் வாதத்தில் கொண்டுவருகிறது.
பாடம் 1ல் பைனரி திட்டத்தை உருவாக்க ` -bin` கொடி கார்கோவைக் கூறுகிறது. இரண்டாவது கட்டளை, ஒரு புதிய திட்டத்தின் அடைவை மாற்றுகிறது.

உருவாக்கப்பட்ட Cargo.toml * கோப்பை பாருங்கள்:

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
```

உங்கள் சுற்றுச்சூழலில் இருந்து கார்கோ-வில் பெறப்பட்ட ஆசிரியர் தகவல் சரி இல்லை என்றால், 
கோப்பில் அதை சரி செய்து அதை மீண்டும் சேமிக்கவும்.


பாடம் 1 இல் பார்த்தபடி, ` ஒரு" ஹலோ, உலகம்! "திட்டத்தை உங்களுக்காக உருவாக்குகிறது. 
* Src / main.rs * கோப்பைப் பாருங்கள்:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

இப்போது "Hello, world!" என்ற தொகுப்பை தொகுக்கலாம் மற்றும்
 அதே படிவத்தில் `கார்கோ ரன்` கட்டளையை பயன்படுத்தி அதை இயக்கவும்
 
```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50 secs
     Running `target/debug/guessing_game`
Hello, world!
```

The `run` command comes in handy when you need to rapidly iterate on a project,
as we’ll do in this game, quickly testing each iteration before moving on to
the next one.

* Src / main.rs * கோப்பை மீண்டும் திறக்கவும். இந்த கோப்பில் உள்ள அனைத்து குறியீடும் எழுதுவீர்கள்.

## ஒரு கணித செயல்பாடு

யூகிக்கக்கூடிய விளையாட்டு திட்டத்தின் முதல் பகுதியானது, பயனர் உள்ளீடு, உள்ளீட்டின் செயலாக்கம்,மற்றும் உள்ளீடு சரிபார்த்தல்  போன்றவற்றை எதிர்பார்க்கப்பட்ட வடிவத்தில் இருக்கிறத என்று கேட்கும். 
தொடங்கும் போது, நாம்
வீரர் ஒரு யூகத்தின் மூலம் உள்ளீட அனுமதிக்கலாம்
. பட்டியல் 2-1 இல் குறியீட்டை *src/main.rs* உள்ளிடவும். 

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

<span class="caption">Listing 2-1: Code that gets a guess from the user and
prints it</span>

இந்த குறியீடு நிறைய தகவலைக் கொண்டுள்ளது, எனவே வரிக்கு வரி வழியாக செல்லலாம்.
பயனர் உள்ளீட்டைப் பெறுங்கள், பின்னர் வெளியீட்டை வெளியீட்டை அச்சிடுவோம், நாம்
`io` (உள்ளீடு / வெளியீடு) நூலகத்தை எடுத்துக்கொண்டு ஸ்கோப்பில் சேர்க்க வேண்டும் .
 நிலையான நூலகம் (இது 'STD` என அறியப்படுகிறது), `Io` நூலகம் இருந்து வருகிறது:

```rust,ignore
use std::io;
```

முன்னிருப்பாக, ரஸ்ட் ஒவ்வொரு திட்டத்தின் வரம்பில் ஒரு சில வகைகளை மட்டுமே கொண்டுவருகிறது
[prelude *] [முன்னோடி] <! - புறக்கணிக்க ->. நீங்கள் பயன்படுத்த விரும்பும் ஒரு வகை முன்னுரை இல்லை என்றால்,
 நீங்கள் அந்த வகையை வெளிப்படையாக கொண்டு வர`` என்ற அறிக்கையை பயன்படுத்தலாம். 
 `Std :: io` நூலகத்தைப் பயன்படுத்துவது உங்களுக்கு பல பலவகை அம்சங்களை  வழங்குகிறது,
பயனர் உள்ளீடு ஏற்கும் திறன் உள்ளிட்ட அம்சங்களையும் வழங்கிறது.
[prelude]: ../../std/prelude/index.html

பாடம் 1 ல் பார்த்தபடி, `பிரதான` செயல்பாட்டின் நுழைவு புள்ளி
திட்டம்:

```rust,ignore
fn main() {
```

`Fn` தொடரியல் ஒரு புதிய செயல்பாட்டை அறிவிக்கிறது, அடைப்புக்குறிகள்,` () `,அளவுருக்கள் இல்லை என்பதை குறிக்கின்றன,
 மற்றும் சுருள் அடைப்புக்குறி, `{`, உடல் செயல்பாடும் தொடங்குகிறது.

நீங்கள் பாடம் 1 இல் கற்றுக்கொண்டது போல்,
  `println!` என்பது திரையில் ஒரு சரத்தை அச்சிடும் ஒரு மேக்ரோ ஆகும்:

```rust,ignore
println!("Guess the number!");

println!("Please input your guess.");
```

இந்த குறியீடானது என்ன விளையாட்டு என்பதைக் குறிப்பிடுவதோடு 
பயனரின் உள்ளீடு கோருகிறது.

### மாறிகள் கொண்ட மதிப்புகளை சேமித்தல்

அடுத்து, பயனர் உள்ளீட்டைச் சேமிப்பதற்கான இடத்தை உருவாக்குவோம்:

```rust,ignore
let mut guess = String::new();
```

இப்போது திட்டம் சுவாரசியமாக உள்ளது!
இந்த சிறிய வரியில் நிறைய நடக்கிறது.  இது ஒரு "let" அறிக்கையாகும், இது உருவாக்க ஒரு பயன்படுத்தப்படும்
* மாறி *. மற்றொரு உதாரணம்:

```rust,ignore
let foo = bar;
```

இந்த வரி `foo` என்ற புதிய மாறினை உருவாக்குகிறது மற்றும் மதிப்பு` பட்டியில்` பிணைக்கிறது.
ரஸ்ட், மாறிகள் இயல்புநிலையில் மாறாதவை.
பாடம் 3 இல் "மாறிகள் மற்றும் மாற்றத்தக்கது" பிரிவில் இந்த கருத்தை நாம் விரிவாக விவாதிப்போம்.
  ஒரு மாறி செய்ய மாறி பெயர் முன் `mut 'பயன்படுத்த 
எப்படி பின்வரும் உதாரணம் காட்டுகிறது

```rust,ignore
let foo = 5; // immutable
let mut bar = 5; // mutable
```

> குறிப்பு: `//` தொடரியல் வரி முடிவின் வரை தொடரும் ஒரு கருத்தை தொடங்குகிறது. 
>ரஸ்ட் கருத்துக்கள் எல்லாம் புறக்கணிக்கிறது,  பாடம் 3 ல்.
>இது இன்னும் விரிவாக விவாதிக்கப்படுகின்றன

யோசிக்கிற விளையாட்டு திட்டத்திற்கு திரும்புவோம்.
'let mut guess' என்ற பெயரில் ஒரு மாறும் மாறினை அறிமுகப்படுத்தும் என்று யூகிக்கிறேன். சமமான மற்ற பக்கத்தில்
அடையாளம் (`=`) என்பது 'யூகம்' என்று வரையறுக்கப்பட்ட மதிப்பு,
  இது `string :: new` என்று அழைப்பதன் விளைவாகும்,
ஒரு சார்பின் ஒரு புதிய நிகழ்விற்குத் திரும்பும் ஒரு சார்பு.
[String] [string] <! - ignore -> என்பது ஒரு வளரக்கூடிய, UTF-8 குறியிடப்பட்ட பிட் உரை நூலகம் வழங்கிய ஒரு சரம் வகை.

[string]: ../../std/string/struct.String.html

The `::` syntax in the `::new` line indicates that `new` is an *associated
function* of the `String` type. An associated function is implemented on a type,
in this case `String`, rather than on a particular instance of a `String`. Some
languages call this a *static method*.

இந்த `புதிய` செயல்பாடு ஒரு புதிய, வெற்று சரத்தை உருவாக்குகிறது.
பல வகைகளில் நீங்கள் `புதிய` செயல்பாட்டைக் காணலாம், 
ஏனென்றால் 
இது ஒரு புதிய பெயர், சில வகையான புதிய மதிப்பை உருவாக்குகிறது.

சுருக்கமாகச் சொல்ல, 'mute guess = string :: new () ஐ அனுமதிக்க வேண்டும், `variable` 
என்பது ஒரு variable variable ஐ உருவாக்கியுள்ளது. அந்தப்புரச்!

நாம் தரநிலையிலிருந்து உள்ளீடு / வெளியீடு செயல்பாடு சேர்க்கப்பட்டதை நினைவுபடுத்தவும்
நிரல் முதல் வரியில் `std :: io; கொண்ட நூலகம்.
இப்போது `io` இல் தொடர்புடைய செயல்பாடு,` stdin` என அழைக்கிறோம்:

```rust,ignore
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

நிரல் ஆரம்பத்தில் 'std :: io` வரியைப் பயன்படுத்தவில்லை எனில், இந்த செயல்பாட்டின் அழைப்புl' std :: io :: stdin` என எழுதப்பட்டிருக்கலாம்.
 'Stdin' செயல்பாடு [std :: io :: stdin`] [iostdin] <! - ignore -> இன்
 ஒரு எடுத்துக்காட்டை கொடுக்கிறது, இது உங்கள் முனையத்திற்கான நிலையான
 உள்ளீட்டிற்கு ஒரு கைப்பிடி என்பதை குறிக்கும் வகை.
அழைப்பு

[iostdin]: ../../std/io/struct.Stdin.html

குறியீடு அடுத்த பகுதியாக, `.read_line (&mud guess)`,
  [read_line`] [read_line] <! - புறக்கணிக்க -> என்று அழைக்கிறது
  பயனரிடம் இருந்து உள்ளீடு பெற தரமான உள்ளீடு முறை கையாள பெற வேண்டும்.
நாங்கள் ஒரு வாதத்தை `படிக்க_லைன்` என்று கடந்து செல்கிறோம்:` read_line``&mud guess`.

[read_line]: ../../std/io/struct.Stdin.html#method.read_line

`Read_line` இன் வேலை பயனர் வகைகளை ஒரு நிலையான string என்ற இடத்திற்கு கொண்டு செல்வதாகும்,
 எனவே அது ஒரு வாதமாக அந்த சரத்தை எடுக்கும். 
 பயனர் உள்ளீட்டைச் சேர்ப்பதன் மூலம், சரத்தின் உள்ளடக்கத்தை
 மாற்றுவதன் மூலம், சரம் வாதம் மாற்றத்தக்கதாக இருக்க வேண்டும்.

`` & `இந்த வாதம் ஒரு * குறிப்பு *
  இது உங்கள் குறியீட்டின் பல பகுதிகளை ஒரு தரவு மென்பொருளை நகலெடுக்க தேவையில்லாமலேயே 
  தரவுகளை ஒரு பகுதியை அணுக அனுமதிக்கிறது. குறிப்புகள் ஒரு சிக்கலான அம்சமாகும்,
மற்றும் ரஸ்ட் முக்கிய நன்மைகள் ஒன்று குறிப்புகள் பயன்படுத்த எவ்வளவு பாதுகாப்பான மற்றும் எளிது.
இந்த திட்டத்தை முடிக்க அந்த விவரங்கள் நிறைய உங்களுக்கு தெரியவேண்டியதில்லை.
இப்போது, நீங்கள் தெரிந்துகொள்ள வேண்டியது எல்லாம் மாறிகள் போல, 
குறிப்புகள் இயல்பாகவே மாறாதவை.
எனவே, நீங்கள் அதை மாற்றிக்கொள்ள செய்ய `& mud guess` & `guess` எழுத வேண்டும்.
 (அத்தியாயம் 4 மேலும் குறிப்புகள் இன்னும் விரிவாக விளக்குகிறது.)

### 'முடிவு' வகை கொண்ட சாத்தியமான தோல்விகளை கையாளுதல்

நாம் இந்த கோடு வரிசையுடன் மிகவும் செய்யவில்லை.
  நாம் இதுவரை விவாதிக்கப்படும் என்ன உரை ஒரு ஒற்றை வரி என்றாலும்,
  அது ஒற்றை logicalline குறியீட்டின் முதல் பகுதி தான். இரண்டாவது பகுதி இந்த முறையாகும்:

```rust,ignore
.expect("Failed to read line");
```

நீங்கள் `.foo ()` தொடரியுடன் ஒரு முறையை அழைக்கையில், நீண்ட வரிசையை உடைக்க உதவும் 
ஒரு புதியலைன் மற்றும் பிற இடைவெளிகளை அறிமுகப்படுத்துவது பெரும்பாலும் புத்திசாலி. 
நாம் இந்த குறியீட்டை இவ்வாறு எழுதியிருக்கலாம்:

```rust,ignore
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

எனினும், ஒரு நீண்ட வரி வாசிக்க கடினமாக உள்ளது, எனவே அதை பிரிப்பது சிறந்தது:
இரண்டு முறை அழைப்புகளுக்கு இரண்டு கோடுகள். இந்த வரி என்ன என்பதை இப்போது பார்க்கலாம்.

முன்னர் குறிப்பிட்டது போல்,
  நாம் அதை கடந்து செல்லும் சரம் மீது பயனர் வகைகளை `read_line` அளிக்கிறது, 
  ஆனால் இந்த வழக்கில் இது ஒரு மதிப்பை தருகிறது  [[io :: result`] [ioresult] <! - ignore ->.
இரஸ்ட் அதன் நிலையான நூலகத்தில் `முடிவு` என்ற வகைகளில் பல உள்ளது:
ஒரு பொதுவான [`Result'] [result] <! - ignore-> அதேபோல்` io :: முடிவு` போன்ற நீர்மூழ்கிகளுக்கான குறிப்பிட்ட பதிப்புகள்.

[ioresult]: ../../std/io/type.Result.html
[result]: ../../std/result/enum.Result.html

`முடிவு` வகைகள் [* enumerations*] [enums] <! - ignore ->,
பெரும்பாலும் * enums * என குறிப்பிடப்படுகிறது. ஒரு கணக்கீடு என்பது ஒரு நிலையான தொகுப்பு மதிப்புகள் இருக்கக்கூடிய ஒரு வகை,
மற்றும் அந்த மதிப்புகள் enum இன் * வகைகள் * என்று அழைக்கப்படுகின்றன.
அத்தியாயம் 6 மேலும் enums-ஐ விரிவாக விரிவுபடுத்தும்.

[enums]: ch06-00-enums.html

`முடிவு`, மாறுபாடுகள்` சரி` அல்லது `பிழை '.
`சரி` மாற்று அறுவை சிகிச்சை வெற்றிகரமாக இருப்பதைக் குறிக்கிறது, மற்றும் `சரி` உள்ளே வெற்றிகரமாக உருவாக்கப்பட்ட மதிப்பு.
'பிழை' மாற்று என்றால் அறுவை சிகிச்சை தோல்வியடைந்தது,
எப்படி, ஏன் அறுவை சிகிச்சை தோல்வியுற்றது என்பது பற்றிய தகவலை `Err` கொண்டுள்ளது.

The purpose of these `Result` types is to encode error-handling information.
Values of the `Result` type, like values of any type, have methods defined on them. An
instance of `io::Result` has an [`expect` method][expect]<!-- ignore --> that
you can call. If this instance of `io::Result` is an `Err` value, `expect` will
cause the program to crash and display the message that you passed as an
argument to `expect`. If the `read_line` method returns an `Err`, it would
likely be the result of an error coming from the underlying operating system.
If this instance of `io::Result` is an `Ok` value, `expect` will take the
return value that `Ok` is holding and return just that value to you so you
can use it. In this case, that value is the number of bytes in what the user
entered into standard input.

[expect]: ../../std/result/enum.Result.html#method.expect

நீங்கள் 'expect` என்று அழைக்கவில்லை என்றால், நிரல் ஒன்று தொகுக்க வேண்டும், ஆனால் நீங்கள் ஒரு எச்சரிக்கையைப் பெறுவீர்கள்:

```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `std::result::Result` which must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default
```

நிரல் சாத்தியமான பிழை கையாளப்படவில்லை என்பதைக் குறிக்கும்,
 `read_line` என்பதில் இருந்து` முடிவு`` மதிப்பை நீங்கள் பயன்படுத்தவில்லை என்று இரஸ்ட் எச்சரிக்கிறது.

எச்சரிக்கையை ஒடுக்குவதற்கான சரியான வழி உண்மையில் பிழைநீக்கத்தை எழுதுவதுதான்,
  ஆனால் இந்த பிரச்சனை ஏற்படும் போது நீங்கள் இந்த திட்டத்தை செயலிழக்க விரும்புவதால், நீங்கள் 'எதிர்பார்ப்பை' பயன்படுத்தலாம்.
  பாடம் 9 இல் பிழைகளை மீட்டெடுப்பது பற்றி அறிந்து கொள்ளலாம்.
  
### 'println!' பெட்டிகள் கொண்ட அச்சிடும் மதிப்புகள்

மூடு வளைவு அடைப்புக்களில் இருந்து தவிர, இதுவரை சேர்க்கப்பட்ட 
குறியீட்டில் விவாதிக்க ஒரே ஒரு வரி இருக்கிறது, இது பின்வருமாறு:

```rust,ignore
println!("You guessed: {}", guess);
```

இந்த வரி,பயனரின் உள்ளீட்டை சேமிக்கும் சரத்தை அச்சிடுகிறது
அடைப்புகள், `{}`, ஒரு ஒதுக்கிடம்:
  `` `சிறிய நண்டு pincers என ஒரு மதிப்பு அதன் இடத்தில் இருக்கும்.
நீங்கள் சுருள் அடைப்புகளை பயன்படுத்தி ஒன்றுக்கு மேற்பட்ட மதிப்பை அச்சிடலாம்:
முதல் வரிசையில் சுருள் அடைப்புக்குறிகள் சரம் வடிவமைக்கப்பட்ட பிறகு பட்டியலிடப்பட்டுள்ள முதல் மதிப்பு உள்ளது, இரண்டாவது தொகுப்பு இரண்டாவது மதிப்பு வைத்திருக்கிறது, மற்றும் பல மதிப்பு உள்ளது.
ஒரே ஒரு அழைப்பில் பல மதிப்புகளை அச்சிடுதல் `println!` என இருக்கும்:


```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

இந்த குறியீடு `x = 5 மற்றும் y = 10` என்று அச்சிடும்.

### முதல் பகுதி சோதனை

யூகிக்கிற விளையாட்டின் முதல் பகுதியை சோதிக்க வேண்டும். `cargo run` பயன்படுத்தி இந்த விளையாட்டை இயக்கவும்:

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

இந்த கட்டத்தில், விளையாட்டின் முதல் பகுதி செய்யப்படுகிறது:
  நாம் விசைப்பலகையில் இருந்து உள்ளீடு பெறுகிறது பின்னர் அச்சிடும்.

## இரகசிய எண்ணை உருவாக்குதல்

அடுத்து, பயனர் யூகிக்க முயற்சிக்கும் இரகசிய எண்ணை உருவாக்க வேண்டும்.
விளையாட்டு ஒவ்வொரு முறையும் வித்தியாசமாக இருக்க வேண்டும், எனவே விளையாட்டு ஒருமுறைக்கு மேல் விளையாட வேடிக்கையாக இருக்கும்.
  1 மற்றும் 100 க்கு இடையில் ஒரு சீரற்ற எண்ணைப் பயன்படுத்தலாம், அதனால் விளையாட்டு கூட இல்லை
கடினமான இருக்காது. இரஸ்ட் இன்னும் அதன் நிலையான நூலகத்தில் சீரற்ற எண் செயல்பாட்டை சேர்க்கவில்லை.
இருப்பினும், இரஸ்ட் குழு ஒரு [`rand` crate] [randcrate]யை வழங்கும்.

[randcrate]: https://crates.io/crates/rand

### crate பயன்படுத்தி மேலூம் செயல்திறனை பெறலாம்

கிரேட் என்பது இரஸ்ட் குறியீட்டின் ஒரு தொகுப்பு என்பதை நினைவில் கொள்ளுங்கள். 
*binary crate * என்ற ஒரு செயல் திட்டத்தை கட்டி வருகிறோம, இது இயங்கக்கூடியதாக இருக்கிறது.
`rand` crate என்பது * library crate * ஆகும், இது மற்ற திட்டங்களில் பயன்படுத்தப்படும் குறியீட்டை கொண்டுள்ளது.

வெளிப்புறக் கோடுகளில் கார்கோ பயன்படுத்துவது உண்மையிலேயே பிரகாசிக்கிறது.
`Rand` ஐ பயன்படுத்தும் குறியீட்டை எழுதுவதற்கு முன்,` rand` crate ஐ ஒரு சார்புடன் சேர்க்க * cargo.toml * கோப்பை மாற்ற வேண்டும்.
  இப்போது அந்த கோப்பைத் திறந்து, `[dependencies]` என்ற தலைப்பின் கீழ் கீழ்க்கண்ட கோட்டைச் சேர்க்கவும்.

<span class="filename">Filename: Cargo.toml</span>

```toml
[dependencies]

rand = "0.3.14"
```

In the *Cargo.toml* file, everything that follows a header is part of a section
that continues until another section starts. The `[dependencies]` section is
where you tell Cargo which external crates your project depends on and which
versions of those crates you require. In this case, we’ll specify the `rand`
crate with the semantic version specifier `0.3.14`. Cargo understands [Semantic
Versioning][semver]<!-- ignore --> (sometimes called *SemVer*), which is a
standard for writing version numbers. The number `0.3.14` is actually shorthand
for `^0.3.14`, which means “any version that has a public API compatible with
version 0.3.14.”

[semver]: http://semver.org

Now, without changing any of the code, let’s build the project, as shown in
Listing 2-2.

```text
$ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading rand v0.3.14
 Downloading libc v0.2.14
   Compiling libc v0.2.14
   Compiling rand v0.3.14
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

<span class = "caption"> பட்டியல் 2-2:
ரேண்ட் கிரேட்டை ஒரு சார்புடன் சேர்த்து 'cargo build' ஐ  வெளியீடுதல் <`span>

நீங்கள் பல்வேறு பதிப்பு எண்களைக் காணலாம் (ஆனால் அவர்கள் அனைவரும் குறியீட்டுடன் இணக்கமாக இருக்க வேண்டும் SemVer-க்கு நன்றி!), மற்றும் கோடுகள் வெவ்வேறு வரிசையில் இருக்கலாம்.
ஆனால் அவர்கள் அனைவரும் குறியீட்டுடன் இணக்கமாக இருக்க வேண்டும்!

இப்போது நாம் ஒரு வெளிப்புற சார்புடையதாக இருப்பதால், கார்கோ அனைத்து வகையான சமீபத்திய பதிப்பை *registry*-ல் இருந்து பெறுகிறது, 
இது தரவின் ஒரு நகலாகும் [Crates.io] [cratesio].
Crates.io என்பது இரஸ்ட் சுற்றுச்சூழலில் உள்ள மக்கள் தங்களது திறந்த மூல இரஸ்ட் திட்டங்களை பிறருக்கு பயன்படுத்துவதற்கு இடமளிக்கிறது.

[cratesio]: https://crates.io

பதிவேட்டை புதுப்பித்த பிறகு, `` சார்புநிலைகள்`` பகுதியைச் சரிபார்த்து, உங்களிடம் இதுவரை இல்லாத எந்தப் பெட்டிகளையும் பதிவிறக்கம் செய்கிறது.
 இந்த வழக்கில், ஒரு சார்பு என நாம் `ரேண்ட்டை 'பட்டியலிட்டிருந்தாலும், கார்கோ` libc` 
 இல் பணிபுரியும் `rand`` சார்ந்திருப்பதால்,` `libc`` என்ற நகலையும் கூட கார்கோ வாங்கின. 
கிரேட்சுகளைப் பதிவிறக்கியபின், ரஸ்ட் அவற்றை தொகுக்கிறது,
 பின்னர் அந்தக் கருவிகளைக் கொண்டிருக்கும் சார்புகளுடன் தொகுக்கிறது.

எந்த மாற்றத்தையும் செய்யாமல் உடனடியாக `cargo build`ஐ  நீங்கள் இயற்றினால்,` 
முடிக்கப்பட்ட` வரிசையில் இருந்து நீங்கள் எந்த வெளியீட்டையும் பெற மாட்டீர்கள். 
கார்கோ ஏற்கனவே அதை பதிவிறக்கம் செய்து தொகுத்துள்ளீர்கள்,
 * Cargo.toml * கோப்பில் எதையும் மாற்றவில்லை.
நீங்கள் உங்கள் குறியீட்டைப் பற்றி எதுவும் மாற்றவில்லை என்று கார்கோவும் அறிந்திருக்கின்றன, எனவே இது மறு ஒழுங்கமைக்காது.
செய்ய எதுவும் இல்லை என்றால், கார்கோ வெறுமனே வெளியேறும்.

* Src / main.rs * கோப்பை திறந்தால், ஒரு சிறிய மாற்றத்தை உருவாக்கவும்,
 பின்னர் அதை சேமித்து மீண்டும் உருவாக்கவும், மற்றும் நீங்கள் இரண்டு வெளியீட்டை மட்டுமே காண முடியும்:
 
```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

These lines show Cargo only updates the build with your tiny change to the
*src/main.rs* file. Your dependencies haven’t changed, so Cargo knows it can
reuse what it has already downloaded and compiled for those. It just rebuilds
your part of the code.

#### * Cargo.lock * கோப்பினைக் கொண்ட மறுகட்டமைக்கப்பட்ட கட்டங்களை உறுதிப்படுத்துகிறது

நீங்கள் அல்லது வேறு ஒருவர் உங்கள் குறியீட்டை உருவாக்குகின்ற ஒவ்வொரு முறையும்
 நீங்கள் அதே கலைஞரை மீண்டும் நாம் கட்டமைக்க முடியும் என்பதை உறுதிப்படுத்தும் ஒரு இயந்திரத்தை கார்கோ கையாளுகிறது. 
 நீங்கள் குறிப்பிடும் வரை நீங்கள் குறிப்பிட்டுள்ள சார்புகளின் பதிப்புகளை மட்டுமே கார்கோ பயன்படுத்தும். எடுத்துக்காட்டாக, 
 `rand` crate இன் அடுத்த வாரம் பதிப்பு 0.3.15 வெளியே வந்தால் என்ன நடக்கிறது என்பதும் மற்றும் ஒரு முக்கிய பிழைத்திருத்தம் உள்ளது என்பதை அறியலாம், 
ஆனால் உங்கள் குறியீடு உடைக்கப்படும் போது ஒரு பின்னடைவு உள்ளது?

இந்த சிக்கலுக்கு பதில் * Cargo.lock * கோப்பு,
நீங்கள் `சரக்குக் கட்டமைப்பை` இயக்கிய முதல் முறையாக இது உருவாக்கப்பட்டது,
 இப்போது உங்கள் * guessing_game * அடைவில் உள்ளது.
நீங்கள் முதல் முறையாக ஒரு திட்டத்தை உருவாக்கும் போது, கார்கோ தகுதிகள் பொருந்தும் சார்புகளின் அனைத்து பதிப்பைக் குறிப்பிடுகிறது,
 பின்னர் அவற்றை Cargo.lock * கோப்பில் எழுதுகிறது. 
 நீங்கள் எதிர்காலத்தில் உங்கள் திட்டத்தை உருவாக்கும்போது, * Cargo.lock * கோப்பு உள்ளது என்பதைக் காணலாம்,
 மேலும் பதிப்புகள் மீண்டும் கண்டறிவதற்கான அனைத்து வேலைகளையும் செய்வதற்கு பதிலாக குறிப்பிடப்பட்ட பதிப்புகளைப் பயன்படுத்தவும்.
  இது தானாகவே ஒரு மறுபயன்பாட்டு கட்டமைப்பை உருவாக்க உதவுகிறது. 
  வேறு வார்த்தைகளில் சொன்னால், நீங்கள் வெளிப்படையாக மேம்படுத்தும் வரை உங்கள் திட்டம் `0.3.14` ஆக இருக்கும்,
* Cargo.lock * கோப்பிற்கு நன்றி.

#### புதிய பதிப்பை பெறுவதற்கு ஒரு க்ரேட்டை மேம்படுத்துதல்

நீங்கள் * ஒரு crate ஐ புதுப்பிக்க விரும்பினால், சரக்கு மற்றொரு கட்டளையை வழங்குகிறது,
 'update', * Cargo.lock * -ல் கோப்பை புறக்கணிக்கிறது, * Cargo.toml * இல்
  உங்கள் குறிப்பீடுகளுக்கு பொருந்தும் அனைத்து சமீபத்திய பதிப்புகளையும் கண்டுபிடிக்கும்,
  இந்த வேலைகள் என்றால், கார்கோ இந்த பதிப்பை * Cargo.lock * கோப்பிற்கு எழுதும்.

ஆனால் முன்னிருப்பாக, `0.3.0` ஐ விட பெரிய மற்றும் `0.4.0` ஐ விட சிறியபதிப்புகளை மட்டுமே கார்கோ தேடுகிறது. 
`ரேண்ட்` crate இரண்டு புதிய பதிப்புகள் ` 0.3.15` மற்றும் `0.4.0` வெளியிட்டிருந்தால்,
'update cargo` இயங்கினால், பின்வருவதைக் காணலாம்:

```text
$ cargo update
    Updating registry `https://github.com/rust-lang/crates.io-index`
    Updating rand v0.3.14 -> v0.3.15
```

இந்த கட்டத்தில், உங்கள் * Cargo.lock * கோப்பில் மாற்றத்தை நீங்கள் கவனிப்பீர்கள், 
நீங்கள் இப்போது பயன்படுத்தும் `rand` crate இன் பதிப்பு` 0.3.15` என்று குறிப்பிடுகிறது.

`rand` பதிப்பு` 0.4.0` அல்லது 0.4.x` தொடரில் உள்ள எந்தவொரு பதிப்பையும் நீங்கள் பயன்படுத்த விரும்பினால், 
நீங்கள் அதற்கு பதிலாக * Cargo.toml * கோப்பு மேம்படுத்த முடியும்.

```toml
[dependencies]

rand = "0.4.0"
```

நீங்கள் அடுத்த முறை `cargo build`-யை இயக்கும் போது,
 சரக்குகள் தரவரிசைகளின் பதிவேட்டை புதுப்பித்து, நீங்கள் குறிப்பிட்டுள்ள
 புதிய பதிப்பிற்கு ஏற்ப உங்கள்` rand` தேவைகளை மறுபரிசீலனை செய்யும்.

[cargo] [doccargo] <! - ignore -> மற்றும் [its
ecosystem] [doccratesio] <! - ignore -> என்பதை பற்றி நாம் இது பாடம் 14- ல் விவாதிககலாம், 
ஆனால் இப்போது, நீங்கள் இதை பற்றி தெரிந்து கொள்ள வேண்டும்
.நூலகங்களை மீண்டும் பயன்படுத்துவது மிகவும் சுலபமாகிறது, 
எனவே பல பொதிகளில் இருந்து சேகரிக்கப்பட்ட சிறிய திட்டங்களை ரஸ்டாசன்கள் எழுத முடியும்.

[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html

### ரேண்டம் எண் உருவாக்குதல்

இப்போது நீங்கள் `ரேண்டட்` crate -ல் * Cargo.toml * ஐ சேர்ப்பதை,`ரேண்ட்` பயன்படுத்தி ஆரம்பிக்கலாம்.
அடுத்த படி * src / main.rs * புதுப்பிப்பது, பட்டியல் 2-3 -ல் காட்டப்பட்டுள்ளது.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

<span class="caption">Listing 2-3: Adding code to generate a random
number</span>

முதலாவதாக, நாம் ஒரு வரியை சேர்க்கிறோம், இது நாம் 'rand` crate ஐ வெளிப்புற சார்பாகப் 
பயன்படுத்துவோம் என்று தெரியும். இது 'use rand' என அழைக்கப்படுவதற்கு சமமானதாகும்,
எனவே இப்போது `rand ::` சேர்ப்பதற்கு முன்பு `rand` crate இல் எதையும் அழைக்க முடியும்.

அடுத்து, நாம் மற்றொரு `பயன்பாடு` வரிசையைச் சேர்க்கிறோம்:` use rand :: Rng`.
 'Rng` குணாம்சமானது சீரற்ற எண் ஜெனரேட்டர்கள் நடைமுறைப்படுத்தும் முறைகளை வரையறுக்கிறது. 
 அத்தியாயம் 10-ல் விரிவாக குணாம்சங்களைக் குறிக்கும்.

மேலும், நாம் நடுப்பகுதியில் இரண்டு கோடுகள் சேர்த்துள்ளோம்.
`Rand :: thread_rng` செயல்பாடு நமக்குத் தேவைப்படும் 
குறிப்பிட்ட ரேண்டம் எண் ஜெனரேட்டரை கொடுக்கும்: 
நடப்பு நூலை உள்ளூர் மற்றும் இயங்குதளத்தால் விதைத்த ஒரு இடம்.
  அடுத்து, நாம் சீரற்ற எண் ஜெனரேட்டர் -ல்  `gen_range` முறையை அழைக்கிறோம். இந்த முறை, `Rand :: RNG`
  அறிக்கையை கொண்டு வரக்கூடிய` Rng` பண்புடன் வரையறுக்கப்படுகிறது. 
  `Gen_range` முறை இரண்டு எண்களை வாதமாக எடுத்துக் கொள்கிறது மற்றும் அவற்றுக்கு இடையில் ஒரு சீரற்ற எண்ணை உருவாக்குகிறது. 
  இது குறைந்த எல்லைக்கு உட்பட்டது, ஆனால் மேல் வரம்பில் பிரத்தியேகமானது,
  எனவே 1 முதல் 100 வரையான இலக்கத்தை கோர, `1` மற்றும்` 101` ஐக் குறிப்பிட வேண்டும்.

> Note: You won’t just know which traits to use and which methods and functions
> to call from a crate. Instructions for using a crate are in each crate’s
> documentation. Another neat feature of Cargo is that you can run the `cargo
> doc --open` command, which will build documentation provided by all of your
> dependencies locally and open it in your browser. If you’re interested in
> other functionality in the `rand` crate, for example, run `cargo doc --open`
> and click `rand` in the sidebar on the left.

நாம் குறியீட்டில் சேர்க்கப்பட்ட இரண்டாவது வரி இரகசிய எண்ணை அச்சிடுகிறது. நிரலை சோதிக்க முடியும் 
என்று நாங்கள் திட்டத்தை வளர்க்கும் போது இது பயனுள்ளதாக இருக்கும்,
 ஆனால் இறுதி பதிப்பிலிருந்து அதை நீக்கி விடுவோம். 
 நிரல் ஆரம்பிக்கையில் உடனடியாக பதில் அச்சிடுவதால், இது ஒரு விளையாட்டாக இல்லை!
 
Try running the program a few times:

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4
$ cargo run
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

நீங்கள் வெவ்வேறு சீரற்ற எண்கள் பெற வேண்டும், 
அவர்கள் அனைவரும் 1 மற்றும் 100 க்கு இடையில் இருக்க வேண்டும். பெரிய வேலை!

## இரகசிய எண்ணுடன் யூகங்களை ஒப்பிடுக

இப்போது நாம் பயனர் உள்ளீடு மற்றும் ஒரு சீரற்ற எண் என்று, நாம் அவர்களை ஒப்பிட முடியும். 
அந்த படிநிலை 2-4 இல் பட்டியலிடப்பட்டுள்ளது. 
இந்த குறியீடானது இன்னும் விளக்கமளிக்காது என்பதை கவனத்தில் கொள்ளவும்.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // ---snip---

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

<span class="caption">Listing 2-4: Handling the possible return values of
comparing two numbers</span>

இங்கே முதல் புதிய பிட் என்பது மற்றொரு வகை 'பயன்பாட்டு` அறிக்கையாகும், இது ஒரு வகை என்று அழைக்கப்படுகிறது
நிலையான நூலகத்திலிருந்து நோக்கம்: STD :: cmp :: ஆர்டர் செய்தல்.
`முடிவு` போன்ற,` ஆர்டர்` மற்றொரு enum, ஆனால் `ஆர்டர்` வகைகள் 'குறைவாக',
கிரேட்டர், மற்றும் `சமம்`. நீங்கள் இரண்டு மதிப்புகள் ஒப்பிடும் போது சாத்தியம் என்று மூன்று விளைவுகள் ஆகும்.

`ordering` வகையைப் பயன்படுத்தும் கீழே புதிய ஐந்து வரிகளைச் சேர்க்கிறோம்.
`Cmp` முறையானது இரண்டு மதிப்புகளை ஒப்பிடுவதோடு ஒப்பிடலாம் என்று எதையும் அழைக்க முடியும்.
நீங்கள் அதை ஒப்பிட்டு என்ன வேண்டுமானாலும் எடுத்துக் கொள்ளுங்கள்: இங்கே அது தான்
`guess` ஒப்பிடுகையில் `secret_number ஒப்பிடுக என்கிறது. 
பின்னர் `ஆர்டிமிங்` enum இன் ஒரு மாறுபாட்டை` use` அறிக்கையில் கொண்டு வரப்படும்.
`order` என்ற மாறுபாட்டின் அடிப்படையில் 'cmp` க்கு' யூகம்` என்ற மதிப்புகள் மூலம்
 திரும்பியதன் அடிப்படையில் என்ன செய்ய வேண்டும் என்பதை முடிவு செய்வதற்கு
 [[match match]] [match] <match - ignore -> மற்றும் 'இரகசிய_உதவி` உதவுகிறது.

[match]: ch06-02-match.html

ஒரு 'match' வெளிப்பாடு * arms * உருவாக்குகிறது.
  ஒரு கையில் ஒரு மாதிரியை * கொண்டுள்ளது மற்றும் 'pattern' வெளிப்பாட்டின் தொடக்கத்திற்கு
  கொடுக்கப்பட்ட மதிப்பு அந்த கை வகைக்கு பொருந்தும் என்றால் அது இயங்க வேண்டும். ரஸ்ட் `match` 
  கொடுக்கப்பட்ட மதிப்பை எடுக்கவும், மேலும் ஒவ்வொரு கைமுறையினூடாகவும் தெரிகிறது.
`match` கட்டமைத்தல் மற்றும் வடிவங்கள் ஆகியவை சக்தி வாய்ந்த அம்சங்களாகும்.
 இது உங்கள் குறியீடுகளை எதிர்கொள்ளும் பல்வேறு சூழ்நிலைகளை வெளிப்படுத்தவும், அவற்றை அனைத்தையும் கையாளவும் உறுதிப்படுத்தவும் உதவுகிறது.
இந்த அம்சங்கள் முறையே பாடம் 6 மற்றும் பாடம் 18 ஆகியவற்றில் விவரிக்கப்பட்டுள்ளன.

'match' வெளிப்பாடு என்ன நடக்கும் என்பது ஒரு உதாரணம் மூலம் இங்கே பயன்படுத்தப்படுகிறது.
பயனர் 50 மற்றும் தோராயமாக உருவாக்கப்பட்ட இரகசிய எண் யூகிக்கும் போது 38 ஆகும்.
குறியீடு 50 முதல் 38 வரை ஒப்பிடும்போது, `cmp` முறை
மீண்டும் `ஆர்டர் செய்தல்: கிரேட்டர்`, ஏனென்றால் 50 என்பது 38 ஐ விட அதிகமாக உள்ளது.
`Match`expression` ordering :: greater` மதிப்பை பெறுகிறது மற்றும் ஒவ்வொரு கை வடிவத்தையும் சரிபார்க்கிறது.
அது முதல் கை வடிவத்தில் இருக்கிறது, `வரிசைப்படுத்தல் :: குறைவாக`, மற்றும் அதைப் பார்க்கிறது
மதிப்பீடு `ordering :: greater` பொருந்தவில்லை` ordering :: less`, எனவே அது புறக்கணிக்கிறது
அந்த கையில் உள்ள குறியீடு மற்றும் அடுத்த கைக்கு நகரும்.
அடுத்த கை வரிசை, `ordering :: greater`, *match*`ordering :: greater`! 
அந்த கையில் உள்ள தொடர்புடைய குறியீடானது
 திரைக்கு மிகப்பெரியதாக 'Too big!` இந்த சூழ்நிலையில் கடைசி கைக்குத் தேவை இல்லை என்பதால், 'match' வெளிப்பாடு முடிவடைகிறது.

இருப்பினும், 2-4 பட்டியலில் உள்ள குறியீட்டை இன்னும் தொகுக்க முடியாது. அதை முயற்சி செய்வோம்:

```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types
  --> src/main.rs:23:21
   |
23 |     match guess.cmp(&secret_number) {
   |                     ^^^^^^^^^^^^^^ expected struct `std::string::String`, found integral variable
   |
   = note: expected type `&std::string::String`
   = note:    found type `&{integer}`

error: aborting due to previous error
Could not compile `guessing_game`.
```

The core of the error states that there are *mismatched types*. Rust has a
strong, static type system. However, it also has type inference. When we wrote
`let guess = String::new()`, Rust was able to infer that `guess` should be a
`String` and didn’t make us write the type. The `secret_number`, on the other
hand, is a number type. A few number types can have a value between 1 and 100:
`i32`, a 32-bit number; `u32`, an unsigned 32-bit number; `i64`, a 64-bit
number; as well as others. Rust defaults to an `i32`, which is the type of
`secret_number` unless you add type information elsewhere that would cause Rust
to infer a different numerical type. The reason for the error is that Rust
cannot compare a string and a number type.

இறுதியில், நாம் "string" என்ற திட்டத்தை மாற்ற வேண்டுமென்றால் உள்ளீடு என நிரல் வாசிக்கிறது
  ஒரு உண்மையான எண்ணை வகைக்குள் எடுத்தால் அது எண்களோடு ஒப்பிடலாம்.
  `பிரதான` செயல்பாடு உடலுக்கு பின்வரும் இரண்டு வரிகளை சேர்ப்பதன் மூலம் இதைச் செய்யலாம்:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
// --snip--

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

The two new lines are:

```rust,ignore
let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");
```

நாம் 'யூகம்' என்ற மாறினை உருவாக்குவோம்.
  ஆனால் காத்திருங்கள், நிரல் ஏற்கனவே 'guess' என்ற மாறி கொண்டிருக்கிறதா?
அது செய்கிறது, ஆனால் இரஸ்ட் எங்களுக்கு * shadow * ஒரு புதிய ஒரு 'guess' முந்தைய மதிப்பு எங்களுக்கு அனுமதிக்கிறது.
  இந்த அம்சம் ஒரு சூழலில் இருந்து மற்றொரு வகைக்கு ஒரு மதிப்பை மாற்ற விரும்பும்
  சூழல்களில் பெரும்பாலும் பயன்படுத்தப்படுகிறது.   உதாரணமாக
  'guess_str` மற்றும் ' guess' போன்ற இரண்டு தனித்துவமான மாறிகள் உருவாக்க எங்களுக்கு கட்டாயப்படுத்தி
  விடவும், யூகங்களை மாற்றியப்பதற்கு பதிலாக, யூகிக்க உதவுகிறது. (அத்தியாம் 3-ல் மேலும் விரிவாக நிழலாடுகிறது.)

We bind `guess` to the expression `guess.trim().parse()`. The `guess` in the
expression refers to the original `guess` that was a `String` with the input in
it. The `trim` method on a `String` instance will eliminate any whitespace at
the beginning and end. Although `u32` can contain only numerical characters,
the user must press <span class="keystroke">enter</span> to satisfy
`read_line`. When the user presses <span class="keystroke">enter</span>, a
newline character is added to the string. For example, if the user types <span
class="keystroke">5</span> and presses <span class="keystroke">enter</span>,
`guess` looks like this: `5\n`. The `\n` represents “newline,” the result of
pressing <span class="keystroke">enter</span>. The `trim` method eliminates
`\n`, resulting in just `5`.

[`parse` method on strings][parse]<!-- ignore --> என்ற ஒரு சரத்தை,
சில வகையான எண்ணுக்கு மாற்ற. இந்த முறை எண் வகைகளை பல்வேறு
 வகைப்படுத்தலாம் என்பதால், நாம் `let guess: u32` ஐ  பயன்படுத்தி 
 நாம் விரும்பும் சரியான எண் வகையை இரஸ்ட்க்கு சொல்ல வேண்டும். பெருங்குடல் (`:`) பிறகு
'guess' இரஸ்ட்டின், மாறி வகை வகைகளை சொல்கிறது. இரஸ்ட்டில் ஒரு சில உள்ளமைக்கப்பட்ட எண் வகைகள் உள்ளன; இங்கே காணப்பட்ட 
`U32` என்பது கையொப்பமிடாத, 32-பிட் ஆகும் முழு. இது ஒரு சிறிய நேர்மறை எண்ணின் 
ஒரு நல்ல முன்னிருப்பு தேர்வு. அத்தியாம் 3-ல் உள்ள மற்ற எண் வகைகளைப் பற்றி நீங்கள் 
அறிந்துகொள்வீர்கள். கூடுதலாக, இந்த எடுத்துக்காட்டு நிரலில் `u32` சிறுகுறிப்பு மற்றும்` secret_number` 
உடன் ஒப்பிடுவது அந்த `secret_number` என்பது` u32` ஆக இருக்க வேண்டும் என்று கூறும்.
எனவே இப்போது ஒப்பீடுகையில் அதே வகை இரண்டு மதிப்புகள் இடையே இருக்கும்!

[parse]: ../../std/primitive.str.html#method.parse

`பாகுபடுத்தி` என்ற அழைப்பு எளிதாக பிழை ஏற்படலாம். உதாரணமாக, சரம்
`A👍 %` ஐ உள்ளடக்கியது, இது ஒரு எண்ணை மாற்றுவதற்கான வழி இல்லை.
  இது தோல்வியடைந்தால், `parse` முறை 'Result` வகையை வழங்குகிறது,
`read_line`  முறை (முன்னதாக "விளைபயன் வகைகளுடன் கூடிய திறன் 
தோல்வியுறையை கையாளுதல்" என்பது விவாதிக்கப்பட்டது). `எதிர்பார்ப்பு` முறையை
மீண்டும் பயன்படுத்துவதன் மூலம் இதேபோல் முடிவுக்கு-யை நாங்கள் ட்ரீட் செய்வோம்.
சரத்திலிருந்து ஒரு எண்ணை உருவாக்க முடியாது என்பதால், 'parse` ஒருவேளை `Err``Result` மாறுபாடு செய்தால், 
`எதிர்பார்ப்பு` அழைப்பு விளையாட்டை முறித்துக் கொண்டு, நாங்கள் கொடுக்கும் செய்தியை அச்சிடுவோம்.
`parse` வெற்றிகரமாக சரத்தை பல எண்ணாக மாற்றினால், அது` முடிவு` என்ற `Ok` மாறுபாட்டைத் தரும், 
மேலும்` Ok` மதிப்பில் இருந்து நாம் விரும்பும் எண்ணை மீண்டும் பெறுவோம்.

இப்போது நிரலை இயக்கலாம்!

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43 secs
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

நைஸ்! யூகங்களை முன் சேர்க்கப்பட்டாலும் கூட,
திட்டம் இன்னும் பயனர் யூகிப்பது 76 என்று கண்டுபிடிக்கப்பட்டது.
பல்வேறு வகையான உள்ளீடுகளுடன் வெவ்வேறு நடத்தை சரிபார்க்க சில நிரலை இயக்கவும்:
சரியாக எண்ணை யூகிக்கவும், அதிக எண்ணிக்கையிலான எண்ணை யூகிக்கவும், மிகவும் குறைவாக இருக்கும் எண்ணை யூகிக்கவும்.

இப்போது நாங்கள் விளையாடிய பெரும்பாலான விளையாட்டுக்களைக் கொண்டிருக்கிறோம், ஆனால் பயனர் ஒரு யூகம் மட்டுமே செய்ய முடியும்.
ஒரு வட்டத்தை சேர்ப்பதன் மூலம் அதை மாற்றலாம்!

## பல வழிகளை தேடுதலைக் கொண்டு அனுமதிக்கிறது

`loop` முக்கிய முடிவிலா சுழற்சியை உருவாக்குகிறது.  நாம்  இப்போது சேர்க்க வேண்டியது
பயனர்களுக்கு  எண்ணை யோசிக்கும்போது அதிக வாய்ப்புகள் கொடுக்க வேண்டும்:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
// --snip--

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // --snip--
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```

நீங்கள் பார்க்க முடியும் என,
யுகத்தின் அடிப்படையில் உள்ளீட்டைத்தை தொடர்ந்து, நாம் எல்லாவற்றையும் ஒரு வட்டத்திற்குள் நகர்த்தியுள்ளோம்.
லூப் உள்ளே உள்ள கோடுகளை ஒவ்வொன்றும் மற்றொரு நான்கு இடைவெளிகளை உள்ளிட்டு மீண்டும் நிரலை இயக்கவும்.
ஒரு புதிய சிக்கல் இருப்பதைக் கவனிக்கவும் ஏனென்றால் நிரல் சரியாகச் செய்ததை நாங்கள் 
செய்தோம்: மற்றொரு நிரூபணத்தை எப்போதும் கேட்கவும்! பயனர் வெளியேற முடியும் போல அது தெரியவில்லை!

பயனர் விசைப்பலகை குறுக்குவழியை பயன்படுத்தி எப்போதும் நிரலை நிறுத்த முடியும்
<span class="keystroke">ctrl-c</span>.
ஆனால் "Comparing the Guess to the Secret Number" 'parse` விவாதத்தில்
 குறிப்பிடப்பட்டுள்ளபடி, இந்த ஏமாற்ற முடியாத அசுரனை தப்பிக்க மற்றொரு வழியும்
 உள்ளது: பயனர் எண் அல்லாத பதிலில் நுழையும் போது, திட்டம் செயலிழக்கப்படும்.
இங்கே காட்டியுள்ளபடி, பயனீட்டாளர் அதைப் பயன்படுத்திக் கொள்ளலாம்:

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50 secs
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/libcore/result.rs:785
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/guess` (exit code: 101)
```

தட்டச்சு செய்தால் `quit` உண்மையில் விளையாட்டுகளை விட்டு விலகும், ஆனால் வேறு ஏதேனும் எண் அல்லாத உள்ளீடு.
இருப்பினும், இது குறைந்தபட்சம் சொல்லுவதற்கு துணைபுரியும்.
சரியான எண்ணிக்கை யூகிக்கப்படும் போது விளையாட்டு தானாகவே நிறுத்தப்பட வேண்டும் என்று நாங்கள் விரும்புகிறோம்.

### சரியான கெஸ் பிறகு வெளியேறவும்

பயனர் ஒரு 'முறிவு` அறிக்கையைச் சேர்ப்பதன் மூலம் வெற்றி பெறும் திட்டம்:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
// --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

`break 'வரிசையைச் சேர்த்தால்,`You win!` என்ற திட்டத்தை வட்டத்திற்கு வெளியே விடுகிறது, எப்போது என்றால்
பயனர் சரியாக இரகசிய எண்ணை யூகிக்கும்போது.
வளையத்திலிருந்து வெளியேறுவதால் அதாவது நிரல் வெளியேறும் என்பதால், வளையம் என்பது `main` கடைசி பகுதியாகும்.

### தவறான உள்ளீடு கையாளுதல்

விளையாட்டின் நடத்தையை மேலும் மேம்படுத்துவதற்கு பதிலாக, 
பயனர் உள்ளீடுகள் எண் அல்லாத போது நிரல் செயலிழந்து விடுகிறது, விளையாட்டு ஒரு எண்ணற்ற
 எண்ணை புறக்கணிப்போம் எனவே பயனர் ஊகித்து தொடரலாம்.  2-5 என்ற
 பட்டியலில் காட்டப்பட்டுள்ளபடி, `U32` என்ற ` String`-ல் `guess` மாற்றப்படும் வரிகளை மாற்றுவதன் மூலம் இதை செய்யலா
 
<span class="filename">Filename: src/main.rs</span>

```rust,ignore
// --snip--

io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

println!("You guessed: {}", guess);

// --snip--
```

<span class="caption">Listing 2-5: Ignoring a non-number guess and asking for
another guess instead of crashing the program</span>

`match` என்ற அழைப்பில் இருந்து` Expect` அழைப்பிலிருந்து மாறுவது என்பது பொதுவாக நீங்கள் எப்படி இருக்கிறீர்கள் என்ற
பிழையை கையாள்வதில் பிழை ஏற்பட்டது.
நினைவில் இருக்கட்டும் 'Parse` என்பது `Result`  திரும்ப பெறவும் `Result` 
என்பது ஒரு  enumத்தில்   `Ok` அல்லது `Err` உள்ளது. `Cmp` முறையின்` வரிசைப்படுத்தல்` 
முடிவுடன் நாங்கள் செய்ததைப் போல, இங்கே ஒரு 'பொருத்தம்` வெளிப்பாட்டை பயன்படுத்துகிறோம்.

சரத்தை ஒரு எண்ணாக `parse` வெற்றிகரமாக மாற்றினால்,
இதன் விளைவாக வரும் எண்ணைக் கொண்ட `Ok` மதிப்பை அது திரும்பப்பெறும்.
அந்த `Ok` மதிப்பு முதல் கை வடிவத்துடன் பொருந்தும், மற்றும் `பொருத்தம்` 
வெளிப்பாடு ` num` மதிப்பை மட்டும் திருப்பிவிடும் அந்த `parse` தயாரிக்கப்பட்டு, ` Ok` மதிப்புக்குள் வைக்கப்பட்டது.
நாம் உருவாக்கும் புதிய "guess" இல் நாம் விரும்பும் இடத்தில் அந்த எண் முடிவடையும்.

சரத்தை ஒரு எண்ணாக மாற்றுவதற்கு `parse`-ல்  * not *
  பிழையைப் பற்றிய கூடுதல் தகவல்களைக் கொண்டிருக்கும் ஒரு `Err 'மதிப்பை அது திரும்பப்பெறும்.
முதல் `போட்டியில் 'Ok(num)' எண்` பொருத்தத்தை `Err 'மதிப்பு பொருந்தவில்லை,
  ஆனால் அது இரண்டாவது கையில் உள்ள `Err (_)` மாதிரியுடன் பொருந்துகிறது. அடிக்கோடிட்டு, `_`, என்பது ஒரு
catchall மதிப்பு; இந்த எடுத்துக்காட்டில், நாங்கள் அனைத்து `பிழை` மதிப்புகளும் பொருந்த 
வேண்டும் என்று சொல்கிறோம்,  அவர்கள் உள்ளே எந்த ஒரு தகவலும் இல்லை.
எனவே நிரல் இரண்டாவது கை குறியீடு, `continue`, திட்டமானது செயல்படுத்துகிறது
 இந்த` loop` அடுத்த மறு செய்கை சென்று மற்றொரு யூகம் கேட்க திட்டம் சொல்கிறது.
எனவே திறம்பட,  நிரல் `parse` எதிர்கொள்ளும் அனைத்து பிழைகளையும் புறக்கணிக்கிறது!

இப்போது நிரலில் உள்ள அனைத்தும் எதிர்பார்த்தபடி வேலை செய்ய வேண்டும். அதை முயற்சி செய்வோம்:

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

அற்புதம்! ஒரு சிறிய இறுதி மாற்றங்களை கொண்டு, நாம் யோசிக்காமல் விளையாட்டு முடிக்க வேண்டும்.
திட்டம் இரகசிய எண்ணை இன்னும் அச்சிடுகிறதா என்பதை நினைவில் வையுங்கள்.
அது சோதனைக்கு நன்றாக வேலை செய்திருந்தது, ஆனால் அது விளையாட்டை அழித்தது.
'Println!' என்று இரகசிய எண்ணை வெளியிடுவோம். பட்டியல் 2-6 கடைசி குறியீட்டை காட்டுகிறது.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

<span class="caption">Listing 2-6: Complete guessing game code</span>

##  சுருக்கம்

இந்த கட்டத்தில், நீங்கள் வெற்றிகரமாக யோசிக்கிற விளையாட்டு உருவாக்கப்பட்டது. வாழ்த்துக்கள்!

பல புதிய ரஸ்ட் கருத்தாக்கங்களை அறிமுகப்படுத்துவதற்கு இந்த திட்டம் கைகொடுத்தது:
`let`,` match`, முறைகள், தொடர்புடைய செயல்பாடுகள், வெளிப்புற கோடுகள் மற்றும் பல.
அடுத்த சில அத்தியாயங்களில், நீங்கள் இந்த விஷயங்களைப் பற்றி மேலும் விரிவாக அறிந்து கொள்வீர்கள்.
அத்தியாயம் 3-ல் பெரும்பாலான நிரலாக்க மொழிகளான கருத்துகளை உள்ளடக்கியது அதாவது
மாறிகள், தரவு வகைகள், மற்றும் செயல்பாடுகளை, மற்றும் எப்படி ரஸ்ட் அவற்றை பயன்படுத்த காட்டுகிறது.
அத்தியாயம் 4-ல் உரிமையை ஆராயூகிறது, அம்சங்கள் இரஸ்ட்டின் மற்ற மொழிகளிருந்து மாறுபடும்
 என்பதை சொல்கிறது. அத்தியாயம் 5-ல், structs மற்றும் முறை தொடரியல் பற்றி விவாதிக்கிறது,
மற்றும் அத்தியாம் 6-ல்,  enumes  எப்படி வேலை செய்கிறது என்பதை விளக்குகிறது.