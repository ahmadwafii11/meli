# MeliLang — Token Specification v0.1.0

**Status:** Revised Draft  
**Language:** MeliLang  
**Version:** 0.1.0  
**Component:** Lexer  
**Implementation Language:** Rust

---

## 1. Introduction

Token Specification mendefinisikan aturan lexical analysis pada MeliLang. Spesifikasi ini menjadi acuan bagi Lexer untuk mengubah source code menjadi urutan token yang akan diproses oleh Parser.

Setiap token merepresentasikan unit terkecil yang memiliki makna dalam struktur bahasa.

Lexer bertanggung jawab untuk:

1. Membaca source code secara berurutan.
2. Mengidentifikasi keyword, identifier, literal, operator, dan punctuation.
3. Mengidentifikasi struktur indentation dan newline.
4. Menyimpan informasi lokasi setiap token.
5. Menghasilkan token stream yang dapat diproses oleh Parser.
6. Menghasilkan lexical error apabila ditemukan karakter atau literal yang tidak valid.

Lexer tidak bertanggung jawab untuk menentukan apakah suatu program memiliki makna semantik yang benar.

## 2. Token Structure

Setiap token memiliki informasi berikut:

| Field | Description |
|---|---|
| `kind` | Jenis token |
| `lexeme` | Teks asli yang membentuk token |
| `span` | Lokasi token di dalam source code |

Representasi konseptual menggunakan Rust:

```rust
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub span: Span,
}

pub struct Span {
    pub start: Position,
    pub end: Position,
}

pub struct Position {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}
```

Ketentuan:

- `offset` menggunakan indeks byte UTF-8.
- `line` dan `column` digunakan untuk pelaporan error.
- Posisi baris dan kolom dimulai dari 1.
- `lexeme` menyimpan teks asli token, termasuk keyword dan operator berbentuk frasa yang masih dipecah menjadi beberapa token.

## 3. Token Categories

### 3.1. Keywords

Keywords merupakan kata-kata yang memiliki makna khusus dalam grammar MeliLang.

| Keyword | Token Kind | Purpose |
|---|---|---|
| `let` | `Let` | Deklarasi variabel immutable |
| `var` | `Var` | Deklarasi variabel mutable |
| `set` | `Set` | Mengubah nilai variabel |
| `define` | `Define` | Mendefinisikan konstanta atau fungsi |
| `function` | `Function` | Mendefinisikan fungsi |
| `returns` | `Returns` | Menentukan tipe kembalian fungsi |
| `when` | `When` | Memulai conditional |
| `otherwise` | `Otherwise` | Alternatif conditional |
| `return` | `Return` | Mengembalikan nilai |
| `show` | `Show` | Menampilkan output |
| `use` | `Use` | Mengimpor modul |
| `repeat` | `Repeat` | Memulai perulangan |
| `for` | `For` | Bagian dari perulangan |
| `each` | `Each` | Menunjukkan iterasi setiap elemen |
| `while` | `While` | Perulangan berdasarkan kondisi |
| `in` | `In` | Menentukan sumber iterasi |
| `be` | `Be` | Menghubungkan deklarasi dengan nilai |
| `to` | `To` | Menentukan nilai assignment |
| `as` | `As` | Penanda dalam deklarasi konstanta sesuai grammar |

### 3.2. Comparison Keywords

MeliLang menggunakan Controlled Natural-Language untuk operator perbandingan.

Kata-kata pembentuk operator perbandingan diproses sebagai token individual, bukan satu token gabungan.

| Keyword | Token Kind | Purpose |
|---|---|---|
| `is` | `Is` | Awalan operator perbandingan |
| `not` | `Not` | Negasi atau bagian operator perbandingan |
| `equal` | `Equal` | Bagian operator kesetaraan |
| `greater` | `Greater` | Bagian operator lebih besar |
| `less` | `Less` | Bagian operator lebih kecil |
| `than` | `Than` | Pelengkap operator perbandingan |
| `at` | `At` | Bagian operator batas minimum atau maksimum |
| `least` | `Least` | Pelengkap operator minimum |
| `most` | `Most` | Pelengkap operator maksimum |

Contoh pemetaan:

| Source Syntax | Token Sequence | AST Operator |
|---|---|---|
| `is equal to` | `Is`, `Equal`, `To` | `Equal` |
| `is not equal to` | `Is`, `Not`, `Equal`, `To` | `NotEqual` |
| `is greater than` | `Is`, `Greater`, `Than` | `GreaterThan` |
| `is less than` | `Is`, `Less`, `Than` | `LessThan` |
| `is at least` | `Is`, `At`, `Least` | `GreaterEqual` |
| `is at most` | `Is`, `At`, `Most` | `LessEqual` |

Lexer hanya menghasilkan token individual. Parser bertanggung jawab untuk mengenali urutan token tersebut sebagai operator perbandingan.

Contoh:

```meli
when accuracy is at least 0.90:
    show "Model is ready"
```

Token sequence bagian perbandingan:

```text
Identifier("accuracy")
Is
At
Least
FloatLiteral("0.90")
```

Parser menginterpretasikannya sebagai operator `GreaterEqual`.

### 3.3. Logical Keywords

| Keyword | Token Kind |
|---|---|
| `and` | `And` |
| `or` | `Or` |
| `not` | `Not` |

`not` juga digunakan dalam operator `is not equal to`.

### 3.4. Identifiers

Identifier digunakan untuk memberikan nama pada variabel, fungsi, parameter, modul, dan entitas lainnya.

Aturan identifier:

- Karakter pertama harus berupa huruf ASCII (`a-z`, `A-Z`) atau underscore (`_`).
- Karakter berikutnya dapat berupa huruf, angka, atau underscore.
- Identifier tidak boleh menggunakan reserved keyword.
- Identifier bersifat case-sensitive.

Contoh valid:

```meli
learning_rate
current_epoch
calculate_accuracy
IMAGE_SIZE
_model
```

Contoh tidak valid:

```meli
123epoch
learning-rate
learning rate
```

Token kind:

```rust
Identifier
```

### 3.5. Numeric Literals

MeliLang menyediakan literal numerik untuk integer dan floating-point.

| Category | Example |
|---|---|
| Integer | `100` |
| Decimal | `0.001` |
| Floating-point | `3.14159` |
| Scientific notation | `1.5e-4` |

Token kinds:

```rust
IntegerLiteral
FloatLiteral
```

Tanda negatif diproses sebagai unary operator `Minus`, bukan bagian dari literal numerik.

Contoh:

```meli
let temperature be -10
```

Token sequence:

```text
Let
Identifier("temperature")
Be
Minus
IntegerLiteral("10")
```

Lexer harus menolak format numerik yang tidak valid, misalnya `1.2.3` atau `1e`.

### 3.6. String and Character Literals

String literal menggunakan tanda kutip ganda.

```meli
let message be "Hello, MeliLang!"
```

Character literal menggunakan tanda kutip tunggal dan hanya boleh berisi satu Unicode scalar value.

```meli
let initial be 'M'
```

Token kinds:

```rust
StringLiteral
CharLiteral
```

Escape sequences yang direncanakan:

| Sequence | Meaning |
|---|---|
| `\n` | Newline |
| `\t` | Horizontal tab |
| `\r` | Carriage return |
| `\\` | Backslash |
| `\"` | Double quotation mark |
| `\'` | Single quotation mark |
| `\0` | Null character |

Lexer harus menghasilkan error apabila string atau character literal tidak ditutup dengan benar.

### 3.7. Boolean Literals

Boolean literal:

```meli
true
false
```

Token kinds:

```rust
True
False
```

### 3.8. Arithmetic Operators

| Operator | Token Kind |
|---|---|
| `+` | `Plus` |
| `-` | `Minus` |
| `*` | `Star` |
| `/` | `Slash` |
| `%` | `Percent` |
| `@` | `MatrixMultiply` |

Operator `@` direncanakan untuk operasi matrix multiplication.

Perlu dibedakan antara keyword `at` dan simbol `@`:

- `at` menghasilkan token `At`.
- `@` menghasilkan token `MatrixMultiply`.

Keduanya merupakan bentuk lexical yang berbeda.

### 3.9. Assignment Operator

| Syntax | Token Kind |
|---|---|
| `=` | `Assign` |

Pada versi awal, assignment statement menggunakan keyword `set` dan `to`.

```meli
set current_epoch to 10
```

Token `Assign` disediakan untuk kebutuhan grammar dan ekspresi yang dapat dikembangkan kemudian. Penggunaannya sebagai assignment statement belum menjadi bentuk kanonis v0.1.0.

### 3.10. Punctuation and Delimiters

| Symbol | Token Kind |
|---|---|
| `(` | `LeftParen` |
| `)` | `RightParen` |
| `[` | `LeftBracket` |
| `]` | `RightBracket` |
| `{` | `LeftBrace` |
| `}` | `RightBrace` |
| `:` | `Colon` |
| `,` | `Comma` |
| `.` | `Dot` |
| `..` | `Range` |
| `->` | `Arrow` |

### 3.11. Structural Tokens

MeliLang menggunakan indentation-based block structure dan newline sebagai pemisah logical statement.

| Token Kind | Purpose |
|---|---|
| `Newline` | Menandai akhir logical statement |
| `Indent` | Menandai awal blok dengan indentation lebih dalam |
| `Dedent` | Menandai akhir blok |
| `Eof` | Menandai akhir source code |

Newline wajib digunakan untuk memisahkan logical statement pada v0.1.0.

Contoh:

```meli
let learning_rate be 0.001
var current_epoch be 0

show "Training started"
```

Setiap deklarasi dan output statement diakhiri newline.

Contoh block:

```meli
when accuracy is at least 0.90:
    show "Excellent"
    show "Continue training"

show "Finished"
```

Token stream secara konseptual:

```text
When
Identifier("accuracy")
Is
At
Least
FloatLiteral("0.90")
Colon
Newline
Indent
Show
StringLiteral("Excellent")
Newline
Show
StringLiteral("Continue training")
Newline
Dedent
Show
StringLiteral("Finished")
Newline
Eof
```

Aturan structural tokens:

1. Setiap logical statement wajib diakhiri newline.
2. Header conditional, function, dan loop diakhiri colon dan newline sebelum block.
3. Satu block menggunakan tingkat indentation yang konsisten.
4. Tab dan spasi tidak boleh dicampur dalam satu file.
5. Blank lines tidak menghasilkan `Indent` atau `Dedent`.
6. Komentar tidak memengaruhi struktur indentation.
7. Dedent harus kembali ke tingkat indentation sebelumnya yang valid.
8. Akhir file harus menghasilkan seluruh `Dedent` yang masih terbuka sebelum `Eof`.
9. Implicit statement termination tidak didukung.
10. Implicit line continuation di luar aturan grammar yang secara eksplisit ditetapkan belum didukung pada v0.1.0.

Untuk menjaga aturan newline wajib tetap sederhana, deklarasi fungsi dengan parameter lebih dari satu baris belum menjadi format kanonis. Gunakan deklarasi dalam satu logical line:

```meli
define function calculate_accuracy(correct: i32, total: i32) returns f32:
    return correct / total
```

### 3.12. Comments

MeliLang menggunakan single-line comment dengan simbol `#`.

```meli
# Initialize training configuration

let learning_rate be 0.001
```

Comment diabaikan oleh Parser dan tidak menghasilkan token AST.

Dukungan multiline comment belum termasuk dalam spesifikasi awal.

## 4. Reserved Keywords

Daftar reserved keywords v0.1.0:

```text
let
var
set
define
function
returns
when
otherwise
return
show
use
repeat
for
each
while
in
be
to
as

is
not
equal
greater
less
than
at
least
most

and
or

true
false
```

Reserved keywords bersifat case-sensitive.

Kata-kata dalam daftar tersebut tidak dapat digunakan sebagai identifier biasa pada v0.1.0.

## 5. Lexical Rules

Lexer mengikuti aturan berikut:

1. Source code dibaca secara berurutan.
2. Whitespace biasa memisahkan token, tetapi tidak menghasilkan token tersendiri.
3. Newline diproses sebagai pemisah logical statement.
4. Indentation diproses untuk menghasilkan `Indent` dan `Dedent`.
5. Reserved keywords dikenali sebelum identifier ditetapkan.
6. Operator multi-character seperti `->` dan `..` dikenali sebelum operator yang lebih pendek.
7. Kata-kata pembentuk controlled natural-language operator dikenali sebagai token individual.
8. String dan character literal harus ditutup dengan delimiter yang sesuai.
9. Komentar diabaikan hingga akhir baris.
10. Setiap token memiliki source span.
11. Lexer menghasilkan lexical error untuk karakter atau literal yang tidak valid.
12. Keyword `at` dan simbol `@` harus menghasilkan token yang berbeda.

## 6. TokenKind Definition

Representasi awal menggunakan Rust:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Declaration and control-flow keywords
    Let,
    Var,
    Set,
    Define,
    Function,
    Returns,
    When,
    Otherwise,
    Return,
    Show,
    Use,
    Repeat,
    For,
    Each,
    While,
    In,
    Be,
    To,
    As,

    // Comparison keywords
    Is,
    Equal,
    Greater,
    Less,
    Than,
    At,
    Least,
    Most,

    // Logical keywords
    And,
    Or,
    Not,

    // Boolean literals
    True,
    False,

    // Identifiers and literals
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,

    // Arithmetic operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    MatrixMultiply,

    // Comparison operators
    EqualEqual,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,

    // Assignment
    Assign,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Colon,
    Comma,
    Dot,
    Range,
    Arrow,

    // Structural tokens
    Newline,
    Indent,
    Dedent,
    Eof,
}
```

`EqualEqual`, `NotEqual`, `GreaterThan`, `LessThan`, `GreaterEqual`, dan `LessEqual` merupakan representasi operator pada tahap AST/Parser. Token stream Lexer tetap berisi kata-kata pembentuk operator secara individual.

## 7. Lexer Error Handling

Lexer harus dapat mengidentifikasi kesalahan lexical.

Contoh:

| Error | Example |
|---|---|
| Invalid character | `let value be $100` |
| Unterminated string | `"Hello MeliLang` |
| Invalid character literal | `'AB'` |
| Invalid number | `1.2.3` |
| Invalid indentation | Indentation kembali ke tingkat yang tidak pernah digunakan |
| Invalid character escape | `"\x"` jika escape tersebut tidak didukung |

Setiap error harus menyertakan lokasi baris dan kolom.

## 8. Scope of Version 0.1.0

Fokus implementasi Lexer:

- Keywords dan identifiers.
- Comparison keywords sebagai token individual.
- Integer, float, string, character, dan boolean literals.
- Arithmetic dan comparison operators.
- Logical operators.
- Punctuation.
- Newline, indentation, dan dedentation.
- Single-line comments.
- Source span dan lexical error reporting.

Fitur seperti multiline strings, multiline comments, interpolation, dan advanced numeric formats belum menjadi prioritas.

## 9. Design Status

Spesifikasi ini merupakan revisi Token Specification MeliLang v0.1.0 berdasarkan keputusan desain:

1. Operator perbandingan menggunakan token individual.
2. Newline wajib menjadi pemisah logical statement.
3. Keyword `at` dibedakan dari simbol `@`.
4. Reserved keywords memiliki aturan case-sensitive.
5. Setiap token memiliki source span.

Implementasi final harus diselaraskan dengan Formal Syntax Design dan AST Structure.

---

**End of Token Specification v0.1.0**