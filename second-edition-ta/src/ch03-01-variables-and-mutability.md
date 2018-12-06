## Variables and Mutability

As mentioned in Chapter 2, by default variables are immutable. This is one of
many nudges Rust gives you to write your code in a way that takes advantage of
the safety and easy concurrency that Rust offers. However, you still have the
option to make your variables mutable. Let’s explore how and why Rust
encourages you to favor immutability and why sometimes you might want to opt
out.

ஒரு மாறி மாறுபடாத போது, ஒரு மதிப்பு ஒரு பெயருடன் பிணைக்கப்படும் போது,
  நீங்கள் அந்த மதிப்பை மாற்ற முடியாது.
இதை விளக்குவதற்கு, `cargo new --bin variables` பயன்படுத்தி உங்கள் * projects * அடைவில்  *  variables *  என்ற புதிய திட்டத்தை உருவாக்கலாம்.

பின்னர், உங்கள் புதிய * variables * அடைவு, திறக்க * src / main.rs * மற்றும்
அதன் குறியீட்டை பின்வரும் குறியீடாக மாற்றுங்கள், இது இன்னும் தொகுக்காது:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

`சரக்கு ரன்` -யை பயன்படுத்தி திட்டத்தை சேமித்து இயக்கவும்.
  இந்த வெளியீட்டில் காட்டப்பட்டுள்ளபடி நீங்கள் ஒரு பிழை செய்தியைப் பெற வேண்டும்:

```text
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
```

உங்கள் நிரல்களில் பிழைகள் கண்டறிய எவ்வாறு தொகுப்பி உதவுகிறது என்பதை இந்த உதாரணம் காட்டுகிறது.
கம்பைலர் பிழைகள் பிழையாக இருந்தாலும், உங்கள் திட்டம் பாதுகாப்பாக இல்லை என்றால்
 இன்னும் என்ன செய்ய வேண்டும் என்று நீங்கள் விரும்புகிறீர்கள் என்று சொல்கிறது. அதனால் நீங்கள் 
 ஒரு நல்ல புரோகிராமர் இல்லை *not* என்று அர்த்தம் இல்லை! அனுபவம் வாய்ந்த ராட்சான்கள் இன்னும் தொகுப்பி பிழைகளை பெறுகின்றனர்.

பிழைக்கான காரணத்தைக, பிழை செய்தி குறிக்கிறது,இது
  நீங்கள் மாறக்கூடிய மாறி x க்கு இரண்டு முறை ஒதுக்க முடியாது என்கிறது.
  ஏனெனில் நீங்கள் மாறாத `x` மாறிக்கு இரண்டாவது மதிப்பை வழங்க முயற்சி செய்தீர்கள்.

நாம் தொகுக்க-நேர பிழைகள் பெற வேண்டியது அவசியம்
நாம் ஒரு மதிப்பை மாற்ற முயற்சிக்கும் போது
இந்த நிலைமை பிழைகள் வழிவகுக்கும் என்பதால் முன்னதாகவே முன்மாதிரியாக நாங்கள் நியமிக்கப்பட்டோம்.
எங்கள் குறியீட்டின் ஒரு பகுதியானது உத்தேசத்தில் செயல்படுகிறது என்றால்
 எங்கள் குறியீடு மாற்றத்தின் மற்றொரு பகுதி மதிப்புஒருபோதும் மாறாது.
குறியிட்டின் ஒரு பகுதி, முன்பு வடிவமைக்கப்பட்டுள்ளதை ஒன்றும் செய்யாது.இந்த வகையான பிழையின் காரணம் உண்மையைக் கண்டுபிடிப்பதற்கு கடினமாக இருக்கலாம்,
குறிப்பாக இரண்டாவது குறியீட்டின் மதிப்பு *sometimes * மாறும் போது.

இரஸ்ட், கம்பைலர் உத்தரவாதம் என்று ஒரு மதிப்பு மாறாது என்று நீங்கள் குறிப்பிடுகையில்,
அது உண்மையில் மாறாது.
அதாவது நீங்கள் குறியீட்டை படித்து மற்றும்  எழுதும்போது, 
எப்படி ஒரு மதிப்பு மாறக்கூடும் என்பதை கண்காணிக்க வேண்டியதில்லை.

ஆனால் மாற்றமடைதல் மிகவும் பயனுள்ளதாக இருக்கும். மாறிகள் இயல்பாகவே மாறாதவை;
அத்தியாயம் 2 ல் நீங்கள் செய்ததைப் போலவே, நீங்கள் மாறியின் பெயர் முன்வைக்கப்படுவதன் மூலம், 'mut` ஐ சேர்ப்பதன் மூலம் அவற்றை மாற்ற முடியும்.
  இந்த மதிப்பை மாற்ற அனுமதிக்கும் கூடுதலாக,
குறியீட்டின் பிற பகுதிகளானது இந்த மாறி மதிப்பு மாறும் என்பதைக் குறிக்கும் வகையில் 
குறியீட்டின் வருங்கால வாசகர்களுக்கு `mut` என்ற எண்ணம் உள்ளது.

உதாரணமாக, பின்வருமாறு * src / main.rs * ஐ மாற்றலாம்:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

இப்போது நிரலை இயக்கினால், இதைப் பெறுவோம்:

```text
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
     Running `target/debug/variables`
The value of x is: 5
The value of x is: 6
```

`x` ஐ `5 இருந்து `6` வரை` என்ற மதிப்பை மாற்ற அனுமதிக்கும் போது, `mut` பயன்படுத்தப்தபடுகிறது.
  சில சந்தர்ப்பங்களில், நீங்கள் ஒரு மாறி mutable செய்ய வேண்டும்
  அது மாறக்கூடிய மாறிகள் கொண்டிருப்பதைக் காட்டிலும் குறியீடாக எழுத மிகவும் வசதியானது.
  
தடுப்பு முகாம்களுக்கு கூடுதலாக கருத்தில் கொள்ள பல வர்த்தக வாய்ப்புகள் உள்ளன.
  எடுத்துக்காட்டாக, நீங்கள் பெரிய தரவு கட்டமைப்புகளைப் பயன்படுத்துகிறீர்கள்,
  புதிதாக ஒதுக்கப்பட்ட நிகழ்வுகளை நகலெடுப்பதிலும் திரும்புவதிலும் வேறொரு இடத்தில் மாற்றம் ஏற்படுவதாகும். 
  சிறிய தரவு கட்டமைப்புகள் மூலம், புதிய நிகழ்வுகளை உருவாக்குதல் மற்றும் மேலும் செயல்பாட்டு நிரலாக்க பாணியில் எழுதுவது எளிதாக இருக்கும்,
அதனால் குறைந்த செயல்திறன் அந்த தெளிவை பெற ஒரு பயனுள்ளது தண்டனை இருக்கலாம்.

### மாறிகள் மற்றும் மாறிலிகள் இடையே வேறுபாடுகள்

ஒரு மாறியின் மதிப்பை மாற்ற இயலாமல் இருக்கலாம்
மற்ற நிரலாக்க கருத்துக்கள் பெரும்பாலும் பிற மொழிகளில் உள்ளன: * consonants *.
 மாறாத மாறிகள் போல, மாறிலிகள் ஒரு பெயருடன் பிணைந்துள்ள மதிப்புகள் ஆகும்,
 மாற்றுவதற்கு அனுமதி இல்லை, ஆனால் மாறிலிகள் மற்றும் சில வேறுபாடுகள் உள்ளன
மாறிகள்.

முதலாவதாக, மாறிலிகளுடன் `mut` பயன்படுத்த உங்களுக்கு அனுமதி இல்லை.
மாறிலிகள் இயல்பாகவே மாறாதவை அல்ல--எப்போதும் மாறக்கூடியவை.

நீங்கள் `let` முக்கிய வார்த்தைக்கு பதிலாக 'const` keyword ஐப் பயன்படுத்தி மாறிலிகளை அறிவிக்கிறீர்கள்,
மற்றும் மதிப்பின் வகையை * must* குறிக்க வேண்டும் .
நாம் அடுத்த வகை, "தரவு வகைகள்" என்ற வகைகளை வகைப்படுத்தவும், பகுப்புகளை வகைப்படுத்தவும் போகிறோம், எனவே விவரங்களைப் பற்றி கவலைப்படாதீர்கள்.
நீங்கள் எப்போதையும் வகைப்படுத்த வேண்டும் என்று உங்களுக்குத் தெரியும்.

கான்ஸ்டன்ட்கள் உலகளாவிய நோக்கம் உள்ளிட்ட எந்தவொரு நோக்கிலும் அறிவிக்கப்படலாம், 
இது குறியீடுகளின் பல பகுதிகளை அறிந்து கொள்ள வேண்டிய மதிப்புகளுக்கு  உதவுகிறது.

கடைசி வேறுபாடு மாறிலிகள்,  ஒரு நிலையான வெளிப்பாட்டை மட்டுமே அமைக்க முடியும்,
ஒரு செயல்பாடு அழைப்பு அல்லது இயக்கத்தில் கணக்கிடப்
படக்கூடிய எந்த வேறு மதிப்பின் விளைவு அல்ல.

இங்கே ஒரு நிலையான அறிவிப்பு ஒரு உதாரணம் எங்கே நிலையான பெயர்
`MAX_POINTS` மற்றும் அதன் மதிப்பு 100,000 ஆக அமைக்கப்பட்டுள்ளது.
(மாறாலிக்கான இரஸ்டின் பெயரிடும் மாநாடு அனைத்து பெரியெளுத்துக்களுக்குடன் அடிக்கோடிட்டு இடையே வார்த்தைகளுடன் பயன்படுத்தப்படுகிறது):

```rust
const MAX_POINTS: u32 = 100_000;
```

Constants are valid for the entire time a program runs, within the scope they
were declared in, making them a useful choice for values in your application
domain that multiple parts of the program might need to know about, such as the
maximum number of points any player of a game is allowed to earn or the speed
of light.

உங்கள் நிரல் முழுவதும் பயன்படுத்தப்பட்டுள்ள கடினப்படுத்தப்பட்ட குறியீட்டு மதிப்புகள் மாறிலியாக இருக்கும்
குறியீட்டின் எதிர்கால பராமரிப்பாளர்களுக்கு அந்த மதிப்பின் பொருள் வெளிப்படுத்துகிறது.
  இது உங்கள் குறியீட்டில் ஒரே ஒரு இடத்தைப் பெற உதவுகிறது என்றால், 
 மேம்படுத்தப்பட வேண்டிய கடினமான மதிப்பை எதிர்காலத்தில் நீங்கள் மாற்ற வேண்டும்.

###   நிழலிடல்

நீங்கள் கற்பனை விளையாட்டு பயிற்சி கண்டது போல் "Comparing the Guess to the Secret Number " 
பாடம் 2-ம் பிரிவில்,  நீங்கள் ஒரு முந்தைய மாறி அதே பெயரில் ஒரு புதிய மாறி அறிவிக்க முடியும்,
  மற்றும் புதிய மாறி நிழலை முந்தைய மாறியில் அறிவிக்க முடியும்.
Rustaceans முதல் மாறி  * shadowed * -ஐ இரண்டாவது மாறியில்,
அதாவது மதிப்பு உபயோகப்படுத்தும் போது இரண்டாவது மாறியின் மதிப்பு என்ன தோன்றும் என்பதை சொல்கிறது.
 அதே மாறிப் பெயரைப் பயன்படுத்துவதன் மூலம் ஒரு மாறியை நிழலிடலாம் மற்றும்
  பின்வருமாறு `let` முக்கிய சொல்லைப் பயன்படுத்துதலாம்:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```

இந்த திட்டம் முதலில்  x` -ல் `5` என்ற மதிப்பை பிணைக்கிறது. பின்னர் `x` 
-ன் நிழல்கள்  மறுபடியும் `let x` செய்வதன் மூலம், அசல் மதிப்பை எடுத்து 
`1` -ஐ சேர்த்தால்,` x`-ன் மதிப்பு  `6` ஆகும்.  மூன்றாவது `let` அறிக்கையும் கூட நிழல்கள் ` x` -ல் ,
முந்தைய மதிப்பை `2`-ல் பெருக்கி,`x`-ன் இறுதி மதிப்பு ` 12` ஆககொடுக்கிறது.
நாம் இந்த திட்டத்தை இயக்கும்போது, பின்வருவனவற்றை வெளியிடுவோம்:

```text
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/variables`
The value of x is: 12
```

நிழல்கள், `mut` என்ற ஒரு மாறியை குறிக்கும் போது வேறுபட்டது,
  ஏனென்றால் நாம் `compile-time error` ஐப் பெறுவோம் என்றால் நாம் இந்த மாறிக்கு`
  `let`key-ஐப் பயன்படுத்தாமல் தட்டச்சு செய்ய முயற்சிக்கிறோம். `Let` ஐப் பயன்படுத்துவதன் மூலம்,
  நாம் ஒரு மதிப்பில், ஒரு சில மாற்றங்களை செய்ய முடியும் ஆனால் 
  அந்த மாற்றங்கள் முடிந்ததும் மாறி மாறக்கூடியதாக இருக்க வேண்டும்.

The other difference between `mut` and shadowing is that because we’re
effectively creating a new variable when we use the `let` keyword again, we can
change the type of the value but reuse the same name. For example, say our
program asks a user to show how many spaces they want between some text by
inputting space characters, but we really want to store that input as a number:

```rust
let spaces = "   ";
let spaces = spaces.len();
```

This construct is allowed because the first `spaces` variable is a string type
and the second `spaces` variable, which is a brand-new variable that happens to
have the same name as the first one, is a number type. Shadowing thus spares us
from having to come up with different names, such as `spaces_str` and
`spaces_num`; instead, we can reuse the simpler `spaces` name. However, if we
try to use `mut` for this, as shown here, we’ll get a compile-time error:

```rust,ignore
let mut spaces = "   ";
spaces = spaces.len();
```

The error says we’re not allowed to mutate a variable’s type:

```text
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected &str, found usize
  |
  = note: expected type `&str`
             found type `usize`
```

Now that we’ve explored how variables work, let’s look at more data types they
can have.
