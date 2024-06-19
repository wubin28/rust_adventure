use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    // 获取初始内存使用情况
    let initial_memory = get_memory_usage();
    println!("Initial memory usage: {} KB", initial_memory);

    {
        // 进入一个新的作用域
        let memory_before = get_memory_usage();
        println!("Memory before creating String: {} KB", memory_before);

        // 创建一个大的字符串
        let _large_string = create_large_string(100_000_000); // 100 MB

        // 获取创建大字符串后的内存使用情况
        let memory_after = get_memory_usage();
        println!("Memory after creating String: {} KB", memory_after);

        // 验证内存增加情况
        assert!(memory_after > memory_before);
    } // 离开作用域，`large_string` 应该被释放

    // 获取离开作用域后的内存使用情况
    let final_memory = get_memory_usage();
    println!("Memory after String is out of scope: {} KB", final_memory);

    // 验证内存减少情况
    assert!(final_memory <= initial_memory + 1_000); // 容许一点点波动
}
// The output after running 'cargo run' should be:
// Initial memory usage: 33 KB
// Memory before creating String: 43 KB
// Memory after creating String: 98347 KB
// Memory after String is out of scope: 43 KB

// 创建一个大的字符串
fn create_large_string(size: usize) -> String {
    let mut s = String::with_capacity(size);
    s.extend(std::iter::repeat('A').take(size));
    s
}

// 获取当前内存使用情况
fn get_memory_usage() -> u64 {
    use jemalloc_ctl::{epoch, stats};
    let e = epoch::mib().unwrap();
    let allocated = stats::allocated::mib().unwrap();

    // 刷新 jemalloc 统计信息
    e.advance().unwrap();

    // 获取分配的内存量并转换为 KB
    let allocated_bytes: u64 = (allocated.read().unwrap() / 1024).try_into().unwrap();
    allocated_bytes
}
