# MeliLang — AST Structure v0.1.0

**Status:** Revised Draft  
**Language:** MeliLang  
**Version:** 0.1.0  
**Component:** Parser / Abstract Syntax Tree  
**Implementation Language:** Rust

---

## 1. Introduction

Abstract Syntax Tree (AST) merupakan representasi struktural dari source code setelah melalui proses lexical analysis dan parsing.

AST digunakan untuk menggambarkan hubungan antarpernyataan dan ekspresi dalam sebuah program tanpa harus mempertahankan seluruh detail penulisan source code.

Pada MeliLang, AST menjadi input utama untuk tahap:

1. Semantic Analysis.
2. Type Checking.
3. Name Resolution.
4. Ownership and Borrowing Analysis.
5. Intermediate Representation (IR) Generation.

AST dirancang menggunakan Rust enums dan structs agar setiap jenis node dapat dikenali secara eksplisit dan aman secara tipe.

## 2. AST Design Principles

Perancangan AST MeliLang mengikuti prinsip berikut:

### 2.1. Typed Structure

Setiap node memiliki tipe yang jelas, seperti `VariableDeclaration`, `FunctionDefinition`, `BinaryExpression`, dan `Conditional`.

### 2.2. Separation of Statements and Expressions

Statement dan expression harus dibedakan.

- Statement digunakan untuk melakukan tindakan atau mendefinisikan struktur program.
- Expression digunakan untuk menghasilkan suatu nilai.

### 2.3. Source Location Tracking

Setiap node menyimpan informasi lokasi source code untuk membantu pelaporan error.

### 2.4. Extensibility

AST harus dapat dikembangkan untuk mendukung tensor operations, automatic differentiation, neural networks, dan fitur ML lainnya.

### 2.5. Compiler-Oriented Representation

AST tidak harus mempertahankan seluruh detail penulisan source code.

Contohnya, frasa `is at least` direpresentasikan sebagai operator `GreaterEqual` dalam AST.

### 2.6. Explicit and Inferred Types

Deklarasi variabel dapat menggunakan tipe eksplisit maupun type inference.

Kedua bentuk tersebut menghasilkan struktur AST yang sama, tetapi nilai `type_annotation` berbeda.

### 2.7. Grouping Preservation

Ekspresi yang menggunakan tanda kurung tetap direpresentasikan melalui `GroupingExpression`.

Hal ini membantu mempertahankan informasi struktur ekspresi dan source location selama tahap awal compiler.

## 3. AST Node Structure

### 3.1. Source Span

Setiap node AST perlu memiliki informasi lokasi sumber.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}
```

`Span` digunakan untuk menunjukkan bagian source code yang membentuk sebuah node.

### 3.2. Program

`Program` merupakan root node dari AST.

```rust
pub struct Program {
    pub statements: Vec<Statement>,
    pub span: Span,
}
```

Satu program dapat terdiri dari beberapa statement.

Contoh:

```meli
let learning_rate be 0.001
var current_epoch be 0
show "Training started"
```

Ketiga pernyataan tersebut akan menjadi elemen dalam `Program.statements`.

---

## 4. Statement Nodes

Statement merupakan instruksi atau struktur program yang tidak harus menghasilkan nilai.

### 4.1. Statement Enum

```rust
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    ConstantDeclaration(ConstantDeclaration),
    Assignment(Assignment),
    FunctionDefinition(FunctionDefinition),
    Conditional(Conditional),
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
    Return(ReturnStatement),
    Show(ShowStatement),
    Import(ImportStatement),
    Expression(ExpressionStatement),
}
```

### 4.2. Variable Declaration

Digunakan untuk mendeklarasikan variabel immutable atau mutable.

MeliLang mendukung dua bentuk deklarasi variabel:

**A. Implicit Type Inference**

```meli
let learning_rate be 0.001
var current_epoch be 0
```

Pada bentuk ini, programmer tidak perlu menuliskan tipe data secara eksplisit. Compiler akan menyimpulkan tipe berdasarkan nilai awal dan aturan type inference.

**B. Explicit Type Annotation**

```meli
let learning_rate: f32 be 0.001
var current_epoch: i32 be 0
```

Pada bentuk ini, programmer menentukan tipe data secara eksplisit menggunakan type annotation setelah nama variabel.

Struktur AST:

```rust
pub struct VariableDeclaration {
    pub name: String,
    pub mutable: bool,
    pub type_annotation: Option<TypeExpression>,
    pub value: Expression,
    pub span: Span,
}
```

Penjelasan atribut:

| Attribute | Description |
|---|---|
| `name` | Nama variabel |
| `mutable` | Menentukan apakah variabel dapat diubah |
| `type_annotation` | Tipe eksplisit atau `None` jika menggunakan inference |
| `value` | Nilai awal variabel |
| `span` | Lokasi deklarasi |

Contoh representasi:

Source:

```meli
let learning_rate be 0.001
```

AST:

```text
VariableDeclaration
├── name: "learning_rate"
├── mutable: false
├── type_annotation: None
└── value: Literal(Float(0.001))
```

Source:

```meli
let learning_rate: f32 be 0.001
```

AST:

```text
VariableDeclaration
├── name: "learning_rate"
├── mutable: false
├── type_annotation: Named("f32")
└── value: Literal(Float(0.001))
```

Parser hanya bertugas membentuk `type_annotation`. Validasi kesesuaian tipe antara annotation dan initial value dilakukan pada tahap semantic analysis dan type checking.

### 4.3. Constant Declaration

Digunakan untuk mendefinisikan konstanta.

Contoh:

```meli
define IMAGE_SIZE as 224
```

Struktur:

```rust
pub struct ConstantDeclaration {
    pub name: String,
    pub value: Expression,
    pub span: Span,
}
```

Konstanta tidak dapat diubah setelah dideklarasikan.

Tipe konstanta dapat disimpulkan berdasarkan initial value pada tahap type checking.

Dukungan explicit type annotation untuk konstanta belum menjadi bagian dari sintaks kanonis v0.1.0.

### 4.4. Assignment

Digunakan untuk mengubah nilai variabel mutable.

Contoh:

```meli
set current_epoch to 10
```

Struktur:

```rust
pub struct Assignment {
    pub target: Expression,
    pub value: Expression,
    pub span: Span,
}
```

Validasi apakah target merupakan variabel yang dapat diubah dilakukan pada tahap semantic analysis.

### 4.5. Function Definition

Digunakan untuk mendefinisikan fungsi.

Contoh:

```meli
define function calculate_accuracy(correct: i32, total: i32) returns f32:
    return correct / total
```

Struktur:

```rust
pub struct FunctionDefinition {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeExpression>,
    pub body: Block,
    pub span: Span,
}

pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<TypeExpression>,
    pub span: Span,
}
```

Ketentuan:

- `name` menyimpan nama fungsi.
- `parameters` menyimpan parameter fungsi.
- `return_type` menyimpan tipe kembalian apabila ditentukan.
- `body` menyimpan statement dalam fungsi.
- `span` menyimpan lokasi deklarasi.

Return type dapat berupa `None` apabila tidak ditentukan secara eksplisit dan sistem type inference dapat menyimpulkannya.

Untuk v0.1.0, parameter dalam satu deklarasi fungsi ditulis pada satu logical line.

### 4.6. Conditional

Digunakan untuk merepresentasikan percabangan program.

Contoh:

```meli
when accuracy is at least 0.90:
    show "Excellent"
otherwise when accuracy is at least 0.75:
    show "Good"
otherwise:
    show "Needs improvement"
```

Struktur:

```rust
pub struct Conditional {
    pub branches: Vec<ConditionalBranch>,
    pub else_branch: Option<Block>,
    pub span: Span,
}

pub struct ConditionalBranch {
    pub condition: Expression,
    pub body: Block,
    pub span: Span,
}
```

Setiap `when` dan `otherwise when` direpresentasikan sebagai `ConditionalBranch`.

Sementara itu, `otherwise` direpresentasikan sebagai `else_branch`.

### 4.7. For Loop

Digunakan untuk melakukan iterasi terhadap sekumpulan elemen atau range.

Contoh:

```meli
repeat for each epoch in 0..100:
    show epoch
```

Struktur:

```rust
pub struct ForLoop {
    pub variable: String,
    pub iterable: Expression,
    pub body: Block,
    pub span: Span,
}
```

### 4.8. While Loop

Digunakan untuk melakukan iterasi selama kondisi bernilai true.

Contoh:

```meli
repeat while current_epoch is less than 100:
    set current_epoch to current_epoch + 1
```

Struktur:

```rust
pub struct WhileLoop {
    pub condition: Expression,
    pub body: Block,
    pub span: Span,
}
```

### 4.9. Return Statement

Digunakan untuk mengembalikan nilai dari fungsi.

Contoh:

```meli
return accuracy
```

Struktur:

```rust
pub struct ReturnStatement {
    pub value: Option<Expression>,
    pub span: Span,
}
```

### 4.10. Show Statement

Digunakan untuk menampilkan informasi ke output.

Contoh:

```meli
show "Training started"
show learning_rate
```

Struktur:

```rust
pub struct ShowStatement {
    pub value: Expression,
    pub span: Span,
}
```

### 4.11. Import Statement

Digunakan untuk mengimpor modul.

Contoh:

```meli
use ml
use tensor
```

Struktur:

```rust
pub struct ImportStatement {
    pub module: Vec<String>,
    pub span: Span,
}
```

### 4.12. Expression Statement

Digunakan ketika suatu expression berdiri sendiri sebagai statement.

```rust
pub struct ExpressionStatement {
    pub expression: Expression,
    pub span: Span,
}
```

---

## 5. Expression Nodes

Expression merupakan struktur sintaksis yang menghasilkan suatu nilai.

### 5.1. Expression Enum

```rust
pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    FunctionCall(FunctionCall),
    Grouping(GroupingExpression),
    Range(RangeExpression),
    Array(ArrayExpression),
}
```

Daftar ini merupakan cakupan awal. Expression untuk tensor dan operasi ML dapat ditambahkan secara bertahap.

### 5.2. Literal

```rust
pub enum Literal {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Char(char),
}
```

Contoh:

```meli
let learning_rate be 0.001
let epochs be 100
let enabled be true
let message be "Hello"
```

### 5.3. Identifier

Merepresentasikan nama variabel, fungsi, atau simbol lainnya.

```rust
pub struct Identifier {
    pub name: String,
    pub span: Span,
}
```

### 5.4. Unary Expression

Digunakan untuk operasi terhadap satu operand.

Contoh:

```meli
let negative_value be -10
let enabled be not false
```

Struktur:

```rust
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
    pub span: Span,
}

pub enum UnaryOperator {
    Negate,
    Not,
}
```

`not` mengikat lebih kuat daripada operator perbandingan. Source `not x is equal to y` menghasilkan `BinaryExpression(Unary(Not, x), Equal, y)`. Untuk meniadakan hasil perbandingan, pengguna menulis `not (x is equal to y)`, yang menghasilkan `Unary(Not, Grouping(BinaryExpression(x, Equal, y)))`.

### 5.5. Binary Expression

Digunakan untuk operasi terhadap dua operand.

Contoh:

```meli
let result be 10 + 20
let is_valid be accuracy is at least 0.90
```

Struktur:

```rust
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
    pub span: Span,
}
```

Operator:

```rust
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    MatrixMultiply,

    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,

    And,
    Or,
}
```

Contoh representasi:

Source:

```meli
accuracy is at least 0.90
```

AST:

```text
BinaryExpression
├── left: Identifier("accuracy")
├── operator: GreaterEqual
└── right: Literal(Float(0.90))
```

Dengan struktur ini, frasa natural-language tidak perlu dipertahankan sebagai teks dalam AST. Parser mengubah urutan token individual menjadi satu operator yang sesuai.

### 5.6. Function Call

Digunakan untuk memanggil fungsi.

Contoh:

```meli
calculate_accuracy(90, 100)
```

Struktur:

```rust
pub struct FunctionCall {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
    pub span: Span,
}
```

### 5.7. Grouping Expression

Digunakan untuk merepresentasikan ekspresi yang dikelompokkan menggunakan tanda kurung.

Contoh:

```meli
let result be (10 + 20) * 2
```

Struktur:

```rust
pub struct GroupingExpression {
    pub expression: Box<Expression>,
    pub span: Span,
}
```

GroupingExpression wajib dipertahankan pada AST v0.1.0.

Contoh representasi:

```text
BinaryExpression
├── left
│   └── GroupingExpression
│       └── BinaryExpression
│           ├── left: Literal(Integer(10))
│           ├── operator: Add
│           └── right: Literal(Integer(20))
├── operator: Multiply
└── right: Literal(Integer(2))
```

Node grouping membantu mempertahankan struktur source code dan informasi lokasi tanda kurung. Tahap optimisasi dapat memprosesnya kemudian tanpa mengharuskan Parser menghilangkan node tersebut.

### 5.8. Range Expression

Digunakan untuk merepresentasikan rentang nilai.

Contoh:

```meli
0..100
```

Struktur:

```rust
pub struct RangeExpression {
    pub start: Box<Expression>,
    pub end: Box<Expression>,
    pub inclusive: bool,
    pub span: Span,
}
```

Operator `..` pada rancangan awal merepresentasikan range dengan batas akhir eksklusif, sehingga Parser mengisi `inclusive: false`.

Batas `start` dan `end` dapat berupa ekspresi aritmetika tanpa tanda kurung karena `..` memiliki precedence lebih rendah daripada `+`, `-`, `*`, `/`, `%`, dan `@`.

Contoh:

```meli
repeat for each i in 0..count - 1:
    show i
```

Pada contoh tersebut, `iterable` pada `ForLoop` adalah:

```text
RangeExpression
├── start: Literal(Integer(0))
├── end: BinaryExpression
│   ├── left: Identifier("count")
│   ├── operator: Subtract
│   └── right: Literal(Integer(1))
└── inclusive: false
```

Range tidak dapat dirangkai (`1..5..10` ditolak sebagai syntax error). Batas berupa perbandingan atau operator logika harus dikelompokkan dengan tanda kurung.

### 5.9. Array Expression

Digunakan untuk mendefinisikan kumpulan nilai.

Contoh:

```meli
let values be [10, 20, 30]
```

Struktur:

```rust
pub struct ArrayExpression {
    pub elements: Vec<Expression>,
    pub span: Span,
}
```

---

## 6. Type Expression

Type Expression merepresentasikan tipe data yang ditulis secara eksplisit dalam source code.

Struktur awal:

```rust
pub enum TypeExpression {
    Named(String),

    Generic {
        name: String,
        arguments: Vec<TypeArgument>,
    },
}

pub enum TypeArgument {
    Type(TypeExpression),
    Integer(usize),
}
```

`TypeArgument` memungkinkan argumen generic berupa tipe maupun dimensi integer. Tidak ada varian `Array` khusus: `Array` adalah nama generic biasa, sama seperti `List`, `Option`, `Result`, dan `Tensor`.

Contoh tipe yang direncanakan:

```meli
i32
f32
String
List[i32]
Option[f32]
Result[String, i32]
Array[f32, 224, 224]
Tensor[f32, 224, 224, 3]
```

Contoh representasi `Tensor[f32, 224, 224, 3]`:

```text
Generic
├── name: "Tensor"
└── arguments
    ├── Type(Named("f32"))
    ├── Integer(224)
    ├── Integer(224)
    └── Integer(3)
```

Contoh penggunaannya dalam deklarasi:

```meli
let learning_rate: f32 be 0.001
var current_epoch: i32 be 0
```

Parser membentuk `TypeExpression` dan menyimpannya dalam `VariableDeclaration.type_annotation`.

Untuk deklarasi tanpa annotation:

```meli
let learning_rate be 0.001
```

Parser menetapkan:

```rust
type_annotation: None
```

Penentuan tipe aktual dilakukan pada tahap type checking.

Catatan: struktur `Tensor` di atas masih berupa rancangan awal. Representasi generic dan dimensi tensor perlu difinalisasi ketika sistem tipe MeliLang dikembangkan.

---

## 7. Block Structure

Block merupakan kumpulan statement yang berada dalam satu struktur kontrol.

Contoh:

```meli
when accuracy is at least 0.90:
    show "Excellent"
    show accuracy
```

Struktur:

```rust
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: Span,
}
```

Parser membentuk `Block` berdasarkan token `Indent` dan `Dedent` yang dihasilkan Lexer.

Setiap statement di dalam block tetap mengikuti aturan newline wajib.

---

## 8. Newline and Statement Parsing

Newline merupakan bagian penting dari grammar MeliLang v0.1.0.

Aturan parsing:

1. Setiap logical statement harus diakhiri token `Newline`.
2. Parser tidak boleh menganggap akhir baris sebagai akhir statement apabila token `Newline` tidak ditemukan.
3. Blank lines dapat diabaikan oleh Parser.
4. Header block harus diikuti `Colon`, `Newline`, dan `Indent`.
5. Akhir block ditandai oleh `Dedent`.
6. `Eof` mengakhiri program setelah seluruh block ditutup.

Contoh valid:

```meli
let learning_rate be 0.001
var current_epoch be 0
show "Training started"
```

Contoh tidak valid:

```meli
let learning_rate be 0.001 var current_epoch be 0
```

Parser harus menghasilkan syntax error karena tidak terdapat pemisah logical statement yang diwajibkan.

---

## 9. Complete AST Example

Source code:

```meli
let learning_rate: f32 be 0.001
var current_epoch: i32 be 0

when learning_rate is at least 0.001:
    show "Training started"
otherwise:
    show "Invalid learning rate"
```

Representasi AST secara konseptual:

```text
Program
│
├── VariableDeclaration
│   ├── name: "learning_rate"
│   ├── mutable: false
│   ├── type_annotation: Named("f32")
│   └── value: Literal(Float(0.001))
│
├── VariableDeclaration
│   ├── name: "current_epoch"
│   ├── mutable: true
│   ├── type_annotation: Named("i32")
│   └── value: Literal(Integer(0))
│
└── Conditional
    │
    ├── branches
    │   └── ConditionalBranch
    │       ├── condition: BinaryExpression
    │       │   ├── left: Identifier("learning_rate")
    │       │   ├── operator: GreaterEqual
    │       │   └── right: Literal(Float(0.001))
    │       │
    │       └── body: Block
    │           └── ShowStatement
    │               └── Literal(String("Training started"))
    │
    └── else_branch: Block
        └── ShowStatement
            └── Literal(String("Invalid learning rate"))
```

Contoh di atas memperlihatkan bahwa:

- Explicit type annotation disimpan sebagai `Named("f32")` dan `Named("i32")`.
- Operator `is at least` menjadi `GreaterEqual`.
- Statement dalam conditional disimpan di dalam `Block`.
- Newline, indentation, dan dedentation digunakan oleh Parser untuk menentukan struktur statement dan block, bukan sebagai node AST tersendiri.

---

## 10. Parser Design Considerations

Parser MeliLang bertanggung jawab untuk:

1. Membaca token stream dari Lexer.
2. Memeriksa kesesuaian urutan token dengan grammar.
3. Membentuk statement dan expression nodes.
4. Menentukan operator berdasarkan precedence dan associativity.
5. Mengelola block menggunakan `Indent` dan `Dedent`.
6. Memastikan setiap logical statement diakhiri `Newline`.
7. Membentuk explicit type annotation jika tersedia.
8. Menghasilkan AST.
9. Melaporkan syntax error dengan informasi source span.

Untuk expression parsing, pendekatan **Pratt Parser** direkomendasikan karena mendukung pengaturan operator precedence secara eksplisit dan relatif mudah dikembangkan.

Rancangan precedence (Priority 1 = terikat paling kuat). Tabel ini harus identik dengan Formal Syntax Design Bagian 5.1:

| Priority | Operators | Associativity |
|---|---|---|
| 1 | Function call, grouping | Left |
| 2 | Unary `-`, `not` | Prefix |
| 3 | `*`, `/`, `%`, `@` | Left |
| 4 | `+`, `-` | Left |
| 5 | `..` | Non-associative |
| 6 | Comparison operators | Non-associative |
| 7 | `and` | Left |
| 8 | `or` | Left |

Comparison chaining belum didukung dalam versi awal.

Contoh berikut harus ditolak:

```meli
1 is less than x is less than 10
```

Programmer harus menggunakan dua perbandingan yang digabungkan dengan `and`:

```meli
1 is less than x and x is less than 10
```

---

## 11. Error Handling

Parser harus dapat mengidentifikasi:

- Unexpected token.
- Missing expression.
- Missing colon.
- Missing indentation block.
- Invalid function declaration.
- Invalid conditional structure.
- Missing closing delimiter.
- Invalid operator sequence.
- Missing newline between statements.
- Invalid type annotation.

Contoh:

```meli
when accuracy is at least:
    show "Excellent"
```

Parser menghasilkan syntax error karena operand kanan operator perbandingan tidak tersedia.

Contoh lainnya:

```meli
let learning_rate: be 0.001
```

Parser menghasilkan syntax error karena type annotation tidak memiliki tipe setelah tanda `:`.

Setiap error harus memuat:

- Error category.
- Error message.
- Source span.
- Expected token atau expression apabila relevan.

---

## 12. Future AST Extensions

AST MeliLang dirancang agar dapat dikembangkan untuk kebutuhan ML.

Beberapa node yang berpotensi ditambahkan:

| Feature | Proposed AST Node |
|---|---|
| Tensor declaration | `TensorExpression` |
| Tensor operation | `TensorOperation` |
| Model definition | `ModelDefinition` |
| Layer definition | `LayerDefinition` |
| Automatic differentiation | `GradientExpression` |
| Device selection | `DeviceExpression` |
| Training pipeline | `TrainingPipeline` |

Node tersebut belum menjadi bagian dari cakupan wajib AST v0.1.0.

---

## 13. Implementation Priority

Urutan implementasi Parser dan AST:

1. Program dan Statement.
2. Variable dan Constant Declaration.
3. Literal dan Identifier.
4. Explicit type annotation dan implicit type inference representation.
5. Arithmetic Expression.
6. Comparison Expression.
7. Grouping Expression.
8. Conditional.
9. Function Definition dan Function Call.
10. For Loop dan While Loop.
11. Return, Show, dan Import.
12. Array dan Range Expression.
13. Type Expression.
14. Parser Error Recovery.

---

## 14. Design Status

AST Structure v0.1.0 merupakan revisi rancangan AST berdasarkan keputusan desain:

1. Operator perbandingan diproses dari token individual dan direpresentasikan sebagai operator AST.
2. Newline wajib menjadi pemisah logical statement.
3. `GroupingExpression` dipertahankan dalam AST.
4. Deklarasi variabel mendukung explicit type annotation dan implicit type inference.
5. Parser bertanggung jawab membentuk struktur sintaksis, sedangkan validasi tipe dilakukan pada tahap type checking.

Implementasi Rust harus tetap mengikuti Formal Syntax Design, Token Specification, dan grammar MeliLang yang telah ditetapkan.

---

**End of AST Structure v0.1.0**