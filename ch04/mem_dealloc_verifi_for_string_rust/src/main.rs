// 使用 jemallocator 库中的 Jemalloc 内存分配器
use jemallocator::Jemalloc;

// 用属性(用于为代码的特定部分提供元信息的注释)定义一个全局的内存分配器，使用 Jemalloc 作为系统的全局内存分配器
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    {
        // 进入一个新的作用域，作用域是用大括号 `{}` 包围的代码块

        // 创建一个包含 100M 大字符串的自定义结构体
        let _large_string_owner = LargeStringOwner::new(100_000_000); // 100 MB
                                                                      // 打印创建大字符串后消息
        println!("Large string created.");
    } // 这里作用域结束，`large_string_owner` 变量自动销毁，`drop` 函数被调用

    // 打印离开作用域后的消息
    println!("Large string scope ended.");
}
// The output of this program after running 'cargo run' would be:
// Large string created.
// Dropping LargeStringOwner, releasing large string memory.
// Large string scope ended.

// 自定义一个包含大字符串的结构体，并实现 Drop trait
struct LargeStringOwner {
    #[allow(dead_code)]
    content: String,
}

impl LargeStringOwner {
    fn new(size: usize) -> Self {
        // 创建一个大的字符串并初始化结构体
        LargeStringOwner {
            content: create_large_string(size),
        }
    }
}

// 实现 Drop trait，添加销毁时的消息打印
impl Drop for LargeStringOwner {
    fn drop(&mut self) {
        println!("Dropping LargeStringOwner, releasing large string memory.");
    }
}

// 创建一个大的字符串函数
fn create_large_string(size: usize) -> String {
    // 创建一个具有预设容量的字符串，容量为 size
    let mut s = String::with_capacity(size);
    // 扩展字符串，填充 size 个 'A' 字符
    s.extend(std::iter::repeat('A').take(size));
    // 返回这个大字符串
    s
}
