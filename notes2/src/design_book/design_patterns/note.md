设计模式是用来描述一门编程语言文化的好标准。设计模式与编程语言息息相关，一门语言中的模式可能在另一种语言中没什么必要，因为语言可能自身特性就能解决问题。或者可能在另一门语言中由于缺少某些特性，压根就实现不了。
设计模式如果滥用，那将会增加程序不必要的复杂性。不过设计模式倒可以用来分享关于一门语言深层次和进阶水平的知识。

- YAGNI
  YAGNI 是不过早添加功能的缩写（You Aren't Going to Need It）
  我曾写过的最好的代码是我没写过的代码

## 命令模式

如果我们的命令很小，可以定义成函数，或作为闭包传递，那么使用函数指针可能更好， 因为它不需要动态分发。但如果我们的命令是个完整的结构， 有一堆函数和变量被分别定义为独立的模块，那么使用 trait 对象会更合适

## 建造者

因为 Rust 缺少重载功能，所以这种模式在 Rust 里比其他语言更常见

## 分解结构体(Compose Structs)

有时候一个很大的结构体会在借用的时候产生问题——`当有多个可变借用`（每个只改变其中一部分字段）的时候会相互冲突。解决方法是将这个大结构体分解成更小的结构体，然后再把这些小结构组装成大结构体，这样结构体中的每个部分都可以单独的借用。

这通常在其他方面带来更好的设计：用这种模式可以展露出更小的功能模块。

在没有借用检查器的语言里中是不需要这种模式的，所以它是 Rust 独有的设计模式。不过，将功能分解成更小的单元是很多有名的软件设计原则中都赞同的，这一点与语言无关。

这种模式依赖于 `Rust 的借用检查器能够分清结构体内部的字段`。在上面的例子中，借用检查器知道 a.b 和 a.c 是相互独立的，就不会尝试去借用整个 a。

## newType

如果我们出于安全考虑想要创建一个 String 的自定义的 Display 实现（例如密码）。 这种情况我们可以用新类型模式提供类型安全和封装。
用带有单独字段的结构来创建一个类型的不透明包装器。这将创建一个新类型，而不是类型的别名。
**通过使用新类型而不是将实现作为 API 的一部分公开出去，它支持你向后兼容地更改实现。**

```rs
struct Password(String);

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Password(****)")
    }
}
```

作用：`隐藏内部接口`

## RAII 守卫

RAII 是个糟糕的名字，代表“`资源获取即初始化`”。
该模式的本质是，`资源的初始化在对象的构造函数中完成`，以及确定性析构器。通过使用一个 RAII 对象作为一些资源的守卫，并且依赖类型系统确保访问始终要通过守卫对象，以此在 Rust 中扩展这种模式。

防止使用未初始化资源和销毁后资源的错误。

## 偏爱更小的库(Prefer Smaller Crates)

## 策略模式

策略模式是支持关注点分离的一门技术。 它还支持通过 `依赖倒置`来分离软件模块。

策略模式背后的基本思想是，给定一个解决特定问题的算法，我们仅在抽象层次上定义算法的框架，并将指定的算法实现分成不同的部分。

这样，使用该算法的客户端可以选择特定的实现，而通用的算法工作流可以保持不变。换句话说，`类的抽象规范不依赖于派生类的具体实现，而是具体实现必须遵循抽象规范。这就是我们为什么叫它“依赖倒置”。`

## 将不安全置于小模块中