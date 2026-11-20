# MeliLang — Formal Syntax Design v0.1.0

**Status:** Revised Draft — Final Grammar Proposal  
**Language:** MeliLang  
**Version:** 0.1.0  
**Component:** Language Design / Parser  
**Implementation Language:** Rust  
**Grammar Notation:** Extended Backus–Naur Form (EBNF)

---

## 1. Introduction

Formal Syntax Design mendefinisikan aturan formal mengenai bagaimana source code MeliLang disusun dan bagaimana token-token yang dihasilkan oleh Lexer dapat dikombinasikan menjadi struktur program yang valid.

Dokumen ini menjadi acuan utama bagi implementasi Parser MeliLang.

Tujuan utama penyusunan grammar:

1. Menentukan struktur formal setiap statement dan expression.
2. Mendefinisikan aturan deklarasi variabel, konstanta, dan fungsi.
3. Mendefinisikan conditional dan loop structures.
4. Menentukan operator precedence dan associativity.
5. Menentukan penggunaan indentation dan newline.
6. Memastikan grammar dapat diterjemahkan menjadi AST Structure v0.1.0.
7. Menghindari ambiguitas dalam proses parsing.

Grammar MeliLang menggunakan pendekatan Controlled Natural-Language Syntax. Artinya, sintaks dirancang agar mudah dibaca oleh pemula, tetapi tetap mengikuti aturan formal yang deterministik.

MeliLang tidak menggunakan free-form natural language. Setiap konstruksi bahasa harus mengikuti pola grammar yang telah ditentukan.

## 2. Grammar Conventions

### 2.1. EBNF Notation

Grammar menggunakan notasi berikut:

| Notation | Meaning |
|---|---|
| `=` | Definition |
| `;` | End of production |
| `\|` | Alternative |
| `{ ... }` | Zero or more repetitions |
| `[ ... ]` | Optional element |
| `( ... )` | Grouping |
| `"..."` | Literal terminal |
| `(* ... *)` | Grammar comment |

Contoh:

```ebnf
identifier = letter, { letter | digit | "_" } ;
```

Artinya, identifier diawali huruf dan dapat diikuti huruf, angka, atau underscore.

### 2.2. Terminal Symbols

Terminal symbols merupakan token yang dihasilkan oleh Lexer.

Grammar menggunakan nama token dari Token Specification v0.1.0, bukan teks source code secara langsung, kecuali untuk memperjelas contoh.

Contoh:

```ebnf
variable_declaration =
    Let, Identifier, Be, expression, Newline ;
```

`Let`, `Identifier`, `Be`, dan `Newline` merupakan token.

### 2.3. Lexer-Generated Structural Tokens

Lexer bertanggung jawab menghasilkan:

- `Newline`
- `Indent`
- `Dedent`
- `Eof`

Parser menggunakan token tersebut untuk menentukan batas statement dan block.

Grammar tidak menghitung jumlah spasi secara langsung. Pengelolaan indentation dilakukan oleh Lexer.

---

## 3. Program Structure

Program merupakan root structure dari seluruh source code MeliLang.

```ebnf
program =
    { Newline },
    { statement },
    Eof ;
```

Satu program dapat berisi nol atau lebih statement.

Contoh:

```meli
let learning_rate be 0.001
var current_epoch be 0

show "Training started"
```

Setiap statement diakhiri `Newline`.

Blank lines dapat muncul di antara statement dan diabaikan oleh Parser.

---

## 4. Statement Grammar

Statement merupakan instruksi atau struktur program yang tidak harus menghasilkan suatu nilai.

### 4.1. Statement

```ebnf
statement =
      variable_declaration
    | constant_declaration
    | assignment_statement
    | function_definition
    | conditional_statement
    | for_statement
    | while_statement
    | return_statement
    | show_statement
    | import_statement
    | expression_statement ;
```

### 4.2. Variable Declaration

Deklarasi variabel mendukung dua bentuk:

1. Implicit type inference.
2. Explicit type annotation.

Grammar:

```ebnf
variable_declaration =
      immutable_declaration
    | mutable_declaration ;

immutable_declaration =
    Let, Identifier,
    [ Colon, type_expression ],
    Be, expression,
    Newline ;

mutable_declaration =
    Var, Identifier,
    [ Colon, type_expression ],
    Be, expression,
    Newline ;
```

Contoh implicit type inference:

```meli
let learning_rate be 0.001
var current_epoch be 0
```

Contoh explicit type annotation:

```meli
let learning_rate: f32 be 0.001
var current_epoch: i32 be 0
```

Ketentuan:

- `let` mendefinisikan variabel immutable.
- `var` mendefinisikan variabel mutable.
- Type annotation bersifat opsional.
- Initial value wajib tersedia pada v0.1.0.
- Tipe aktual ditentukan oleh type checker.
- Ketidaksesuaian antara tipe annotation dan initial value merupakan semantic error, bukan syntax error.

### 4.3. Constant Declaration

Grammar:

```ebnf
constant_declaration =
    Define, Identifier,
    As, expression,
    Newline ;
```

Contoh:

```meli
define IMAGE_SIZE as 224
define MAX_EPOCHS as 100
```

Konstanta tidak dapat diubah setelah dideklarasikan.

Explicit type annotation pada konstanta belum didukung dalam grammar kanonis v0.1.0.

### 4.4. Assignment Statement

Grammar:

```ebnf
assignment_statement =
    Set, assignment_target,
    To, expression,
    Newline ;

assignment_target =
    Identifier ;
```

Contoh:

```meli
set current_epoch to 10
```

Pada v0.1.0, assignment hanya diperbolehkan terhadap identifier.

Validasi apakah identifier telah dideklarasikan dan bersifat mutable dilakukan pada semantic analysis.

### 4.5. Function Definition

Grammar:

```ebnf
function_definition =
    Define, Function, Identifier,
    LeftParen,
    [ parameter_list ],
    RightParen,
    [ Returns, type_expression ],
    Colon,
    Newline,
    Indent,
    block_body,
    Dedent ;
```

Parameter:

```ebnf
parameter_list =
    parameter,
    { Comma, parameter } ;

parameter =
    Identifier,
    [ Colon, type_expression ] ;
```

Contoh:

```meli
define function calculate_accuracy(correct: i32, total: i32) returns f32:
    return correct / total
```

Ketentuan:

- Nama fungsi wajib ditentukan.
- Parameter dapat memiliki type annotation.
- Return type bersifat opsional.
- Function body menggunakan indentation-based block.
- Parameter wajib dipisahkan dengan koma.
- Trailing comma belum didukung pada v0.1.0.
- Deklarasi fungsi ditulis dalam satu logical line sebelum block.

### 4.6. Block Structure

Grammar:

```ebnf
block =
    Newline,
    Indent,
    block_body,
    Dedent ;

block_body =
    { Newline },
    statement,
    { statement },
    { Newline } ;
```

Block berisi satu atau lebih statement.

Contoh:

```meli
when accuracy is at least 0.90:
    show "Excellent"
    show accuracy
```

Parser membentuk `Block` yang berisi dua `ShowStatement`.

Block kosong belum didukung pada v0.1.0.

### 4.7. Conditional Statement

Grammar:

```ebnf
conditional_statement =
    When, expression,
    Colon,
    block,
    { otherwise_when_clause },
    [ otherwise_clause ] ;

otherwise_when_clause =
    Otherwise, When,
    expression,
    Colon,
    block ;

otherwise_clause =
    Otherwise,
    Colon,
    block ;
```

Contoh:

```meli
when accuracy is at least 0.90:
    show "Excellent"
otherwise when accuracy is at least 0.75:
    show "Good"
otherwise:
    show "Needs improvement"
```

Ketentuan:

- Conditional utama harus diawali `when`.
- `otherwise when` dapat digunakan berulang kali.
- `otherwise` hanya boleh muncul satu kali.
- `otherwise` harus menjadi cabang terakhir.
- Setiap kondisi harus menghasilkan nilai boolean.
- Setiap cabang harus memiliki block yang valid.

### 4.8. For Loop

Grammar:

```ebnf
for_statement =
    Repeat, For, Each,
    Identifier,
    In,
    expression,
    Colon,
    block ;
```

Contoh:

```meli
repeat for each epoch in 0..100:
    show epoch
```

Ketentuan:

- Variabel iterasi wajib ditentukan.
- Iterable dapat berupa range atau collection.
- Block menggunakan indentation.
- Validasi tipe iterable dilakukan pada semantic analysis.

### 4.9. While Loop

Grammar:

```ebnf
while_statement =
    Repeat, While,
    expression,
    Colon,
    block ;
```

Contoh:

```meli
repeat while current_epoch is less than 100:
    set current_epoch to current_epoch + 1
```

Kondisi while harus menghasilkan boolean.

### 4.10. Return Statement

Grammar:

```ebnf
return_statement =
    Return,
    [ expression ],
    Newline ;
```

Contoh:

```meli
return accuracy
```

Return tanpa nilai juga diperbolehkan:

```meli
return
```

Validasi apakah return berada di dalam fungsi dilakukan pada semantic analysis.

### 4.11. Show Statement

Grammar:

```ebnf
show_statement =
    Show,
    expression,
    Newline ;
```

Contoh:

```meli
show "Training started"
show learning_rate
show current_epoch + 1
```

### 4.12. Import Statement

Grammar:

```ebnf
import_statement =
    Use,
    module_path,
    Newline ;

module_path =
    Identifier,
    { Dot, Identifier } ;
```

Contoh:

```meli
use ml
use tensor
use ml.training
```

Import resolution dilakukan pada tahap semantic analysis atau module resolution.

### 4.13. Expression Statement

Grammar:

```ebnf
expression_statement =
    expression,
    Newline ;
```

Expression statement harus menghasilkan bentuk sintaksis expression yang valid.

Apakah expression tersebut boleh berdiri sendiri tanpa menghasilkan efek yang berguna merupakan persoalan semantic analysis.

---

## 5. Expression Grammar

Expression merupakan konstruksi sintaksis yang menghasilkan suatu nilai.

Grammar expression menggunakan precedence hierarchy untuk memastikan interpretasi operator tidak ambigu.

### 5.1. Expression Entry Point

```ebnf
expression =
    logical_or_expression ;
```

### 5.2. Logical OR

```ebnf
logical_or_expression =
    logical_and_expression,
    { Or, logical_and_expression } ;
```

Operator `or` memiliki precedence terendah.

### 5.3. Logical AND

```ebnf
logical_and_expression =
    comparison_expression,
    { And, comparison_expression } ;
```

Operator `and` memiliki precedence lebih tinggi daripada `or`.

### 5.4. Comparison Expression

```ebnf
comparison_expression =
    additive_expression,
    [ comparison_operator, additive_expression ] ;
```

Grammar hanya memperbolehkan satu comparison operator pada satu comparison expression.

Comparison operator:

```ebnf
comparison_operator =
      EqualToOperator
    | NotEqualToOperator
    | GreaterThanOperator
    | LessThanOperator
    | GreaterEqualOperator
    | LessEqualOperator ;
```

Definisi operator:

```ebnf
EqualToOperator =
    Is, Equal, To ;

NotEqualToOperator =
    Is, Not, Equal, To ;

GreaterThanOperator =
    Is, Greater, Than ;

LessThanOperator =
    Is, Less, Than ;

GreaterEqualOperator =
    Is, At, Least ;

LessEqualOperator =
    Is, At, Most ;
```

Contoh:

```meli
accuracy is equal to 0.90
accuracy is not equal to 0.50
accuracy is greater than 0.75
accuracy is less than 0.95
accuracy is at least 0.90
accuracy is at most 1.00
```

Lexer menghasilkan token individual untuk setiap keyword.

Contoh token sequence:

```text
Identifier("accuracy")
Is
At
Least
FloatLiteral("0.90")
```

Parser mengenali pola `Is At Least` dan mengubahnya menjadi operator AST `GreaterEqual`.

Pemetaan operator:

| Grammar Pattern | AST Operator |
|---|---|
| `Is Equal To` | `Equal` |
| `Is Not Equal To` | `NotEqual` |
| `Is Greater Than` | `Greater` |
| `Is Less Than` | `Less` |
| `Is At Least` | `GreaterEqual` |
| `Is At Most` | `LessEqual` |

Comparison chaining tidak didukung.

Contoh tidak valid:

```meli
1 is less than x is less than 10
```

Gunakan:

```meli
1 is less than x and x is less than 10
```

### 5.5. Additive Expression

```ebnf
additive_expression =
    multiplicative_expression,
    { ( Plus | Minus ), multiplicative_expression } ;
```

Contoh:

```meli
let result be 10 + 20 - 5
```

Operator `+` dan `-` memiliki left associativity.

### 5.6. Multiplicative Expression

```ebnf
multiplicative_expression =
    unary_expression,
    { ( Star | Slash | Percent | MatrixMultiply ), unary_expression } ;
```

Contoh:

```meli
let result be 10 * 5
let quotient be 20 / 4
```

Matrix multiplication:

```meli
let result be matrix_a @ matrix_b
```

Operator `MatrixMultiply` memiliki precedence yang sama dengan operator multiplicative pada v0.1.0.

Validasi operand matrix dilakukan pada tahap type checking.

### 5.7. Unary Expression

```ebnf
unary_expression =
      unary_operator, unary_expression
    | range_expression ;

unary_operator =
      Minus
    | Not ;
```

Contoh:

```meli
let negative_value be -10
let enabled be not false
```

Unary operators menggunakan right-to-left nesting melalui recursive grammar.

### 5.8. Range Expression

```ebnf
range_expression =
    postfix_expression,
    [ Range, postfix_expression ] ;
```

Contoh:

```meli
let epochs be 0..100
```

Operator `..` menghasilkan `RangeExpression`.

Ketentuan:

- Range memiliki batas akhir eksklusif.
- Range tidak dapat dirangkai secara langsung.
- Kedua batas harus berupa expression yang valid.
- Validasi kesesuaian tipe batas dilakukan pada semantic analysis.

### 5.9. Postfix Expression

```ebnf
postfix_expression =
    primary_expression,
    { function_call_suffix } ;

function_call_suffix =
    LeftParen,
    [ argument_list ],
    RightParen ;

argument_list =
    expression,
    { Comma, expression } ;
```

Contoh:

```meli
calculate_accuracy(90, 100)
```

Function call menghasilkan `FunctionCall` dalam AST.

Pada v0.1.0, method call, member access, dan indexing belum menjadi bagian dari grammar postfix.

### 5.10. Primary Expression

```ebnf
primary_expression =
      literal
    | Identifier
    | grouping_expression
    | array_expression ;
```

### 5.11. Grouping Expression

```ebnf
grouping_expression =
    LeftParen,
    expression,
    RightParen ;
```

Contoh:

```meli
let result be (10 + 20) * 2
```

Parser wajib mempertahankan struktur grouping sebagai `GroupingExpression` dalam AST.

### 5.12. Array Expression

```ebnf
array_expression =
    LeftBracket,
    [ array_elements ],
    RightBracket ;

array_elements =
    expression,
    { Comma, expression } ;
```

Contoh:

```meli
let values be [10, 20, 30]
```

Array kosong juga diperbolehkan:

```meli
let values be []
```

Penentuan tipe elemen array dilakukan pada type checking.

---

## 6. Literal Grammar

Literal merupakan nilai tetap yang ditulis secara langsung dalam source code.

```ebnf
literal =
      IntegerLiteral
    | FloatLiteral
    | StringLiteral
    | CharLiteral
    | True
    | False ;
```

Contoh:

```meli
let epochs be 100
let learning_rate be 0.001
let message be "Hello MeliLang"
let initial be 'M'
let enabled be true
```

Representasi AST:

| Token | AST Literal |
|---|---|
| `IntegerLiteral` | `Literal::Integer` |
| `FloatLiteral` | `Literal::Float` |
| `StringLiteral` | `Literal::String` |
| `CharLiteral` | `Literal::Char` |
| `True` | `Literal::Boolean(true)` |
| `False` | `Literal::Boolean(false)` |

---

## 7. Type Expression Grammar

Type expression digunakan untuk menentukan tipe data secara eksplisit.

Grammar:

```ebnf
type_expression =
      named_type
    | generic_type
    | array_type ;

named_type =
    Identifier ;

generic_type =
    Identifier,
    LeftBracket,
    type_argument_list,
    RightBracket ;

type_argument_list =
    type_argument,
    { Comma, type_argument } ;

type_argument =
      type_expression
    | IntegerLiteral ;

array_type =
    Array,
    LeftBracket,
    type_expression,
    Comma,
    IntegerLiteral,
    { Comma, IntegerLiteral },
    RightBracket ;
```

Contoh:

```meli
i32
f32
String
List[i32]
Option[f32]
Result[String, i32]
Tensor[f32, 224, 224, 3]
Array[f32, 224, 224]
```

Contoh penggunaan:

```meli
let learning_rate: f32 be 0.001
let epochs: i32 be 100
```

Ketentuan:

- Named type menggunakan identifier.
- Generic type menggunakan tanda kurung siku.
- Generic type dapat menerima type argument dan integer dimension.
- Array type dapat menerima tipe elemen dan dimensi.
- Validitas nama tipe dan jumlah argumen diperiksa oleh type checker.

Catatan implementasi: AST TypeExpression perlu mendukung integer generic arguments agar `Tensor[f32, 224, 224, 3]` dapat direpresentasikan secara benar. Detail penyesuaian AST dijelaskan pada bagian 12.

---

## 8. Lexical and Layout Constraints

Grammar ini bergantung pada aturan lexical analysis yang telah didefinisikan dalam Token Specification v0.1.0.

### 8.1. Newline Rules

Setiap logical statement wajib diakhiri `Newline`.

Contoh valid:

```meli
let first_value be 10
let