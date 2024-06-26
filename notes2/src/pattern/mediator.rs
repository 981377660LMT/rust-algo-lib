// !中介者模式本质是图论中的"中心节点"或者"虚拟节点"，所有的节点都通过中心节点来通信，而不是直接通信。
// !让程序组件通过特殊的中介者对象进行间接沟通， 达到减少组件之间依赖关系的目的。
// 应用场景：rbac鉴权，组件之间的通信等。
//
// 自上而下的所有权方法允许在 Rust 中应用 Mediator，
// 因为它适合 Rust 的所有权模型，具有严格的借用检查器规则。
// 这不是实现 Mediator 的唯一方法，但它是一种基本方法。
