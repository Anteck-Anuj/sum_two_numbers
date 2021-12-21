use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub a: u32,
    pub b: u32,
    pub res: u32,
}

impl GreetingAccount {
    fn total(&self) -> u32 {
        self.a + self.b
    }
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("+++++++++++++ Hello World Rust program entrypoint +++++++++++++ ");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;
    // let res = String::from_utf8(_instruction_data.to_vec());

    
    let coming_data = GreetingAccount::try_from_slice(_instruction_data)?;

    
    msg!("coming_data   => {:?}", coming_data.total());
    // let coming_data = serde_json::from_str(String::from_utf8(_instruction_data.to_vec()))
    msg!(" accounts ==>  {:?}", accounts);
    msg!("program_id ==> {:?}", program_id);

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted
    msg!("&account.data.borrow()  ==> {:?}", &account.data.borrow());
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    msg!("greeting_account ==> {:?}", greeting_account);
    greeting_account.a = coming_data.a;
    greeting_account.b = coming_data.b;
    greeting_account.res = greeting_account.total();

    msg!("greeting_account => {:?}", greeting_account);
    
    msg!("account data => {:?}", account);
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .a,
            0
        );
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .b,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .a,
            1
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .b,
            2
        );
    }
}
