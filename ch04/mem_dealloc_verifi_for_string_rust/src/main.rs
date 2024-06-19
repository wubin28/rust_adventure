// 使用 jemallocator 库中的 Jemalloc 内存分配器
use jemallocator::Jemalloc;

// 用属性(用于为代码的特定部分提供元信息的注释)定义一个全局的内存分配器，使用 Jemalloc 作为系统的全局内存分配器
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

// 主函数，从这里开始执行程序
fn main() {
    // 获取当前系统的初始内存使用情况
    let initial_memory = get_memory_usage();
    // 打印初始内存使用情况，单位是 KB
    println!("Initial memory usage: {} KB", initial_memory);

    {
        // 进入一个新的作用域，作用域是用大括号 `{}` 包围的代码块
        let memory_before = get_memory_usage();
        // 打印创建字符串前的内存使用情况
        println!("Memory before creating String: {} KB", memory_before);

        // 创建一个大字符串，占用大约 100 MB 内存
        let _large_string = create_large_string(100_000_000); // 100 MB

        // 获取创建大字符串后的内存使用情况
        let memory_after = get_memory_usage();
        // 打印创建大字符串后的内存使用情况
        println!("Memory after creating String: {} KB", memory_after);

        // 使用标准库的断言宏assert!，验证内存是否增加，否则中止程序，并打印错误信息
        assert!(memory_after > memory_before);
    } // 这里作用域结束，`_large_string` 变量自动销毁，内存应该被释放

    // 获取离开作用域后的内存使用情况
    let final_memory = get_memory_usage();
    // 打印离开作用域后的内存使用情况
    println!("Memory after String is out of scope: {} KB", final_memory);

    // 验证最终的内存使用是否接近初始值，允许有一些小波动
    assert!(final_memory <= initial_memory + 1_000); // 容许一点点波动
}
// The output after running 'cargo run' should be:
// Initial memory usage: 33 KB
// Memory before creating String: 43 KB
// Memory after creating String: 98347 KB
// Memory after String is out of scope: 43 KB

// 创建一个大的字符串函数
fn create_large_string(size: usize) -> String {
    // 创建一个具有预设容量的字符串，容量为 size
    let mut s = String::with_capacity(size);
    // 扩展字符串，填充 size 个 'A' 字符
    s.extend(std::iter::repeat('A').take(size));
    // 返回这个大字符串
    s
}

// 获取当前内存使用情况的函数
fn get_memory_usage() -> u64 {
    // 引入 jemalloc_ctl 库中的 epoch 和 stats 模块。Rust 可以在函数定义的内部使用 use 语句引入外部模块
    use jemalloc_ctl::{epoch, stats};
    // 获取 epoch 模块的 MIB（管理信息块）
    let e = epoch::mib().unwrap();
    // 获取 stats 模块的 allocated MIB
    let allocated = stats::allocated::mib().unwrap();

    // 刷新 jemalloc 的统计信息，使得获取的内存使用情况是最新的
    e.advance().unwrap();

    // 读取当前分配的内存量，单位是字节
    let allocated_bytes: u64 = (allocated.read().unwrap() / 1024).try_into().unwrap();
    // 将字节转换为 KB 并返回
    allocated_bytes
}
