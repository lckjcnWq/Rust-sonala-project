use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg
};

// 声明程序入口点
entrypoint!(process_instruction);

/// 程序入口点函数
/// 
/// # 参数
/// - `program_id` - 程序的公钥地址
/// - `accounts` - 指令所需的账户列表
/// - `instruction_data` - 序列化的指令数据
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    msg!("Hello, world!");
    msg!("程序 ID: {}", program_id);
    msg!("账户数量: {}", accounts.len());
    msg!("指令数据长度: {}", instruction_data.len());
    
    Ok(())
}
