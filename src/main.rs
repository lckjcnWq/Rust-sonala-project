mod variables1;
mod baseType;

fn main() {
    // 调用 variables1 模块中的 main1 函数
    variables1::main1();
    
    baseType::main1();
    
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
