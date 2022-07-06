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
    pub counter: u32,
}

// Declare and export the program's entrypoint
// this is related to line 13 in the cargo.toml file
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {

    // B - had to double check syntax to declare a string lol
    let name = String::from("Brendan");

    // B - does this string interpolation work?
    msg!("Entrypoint to {name} greeting");

    // Iterating accounts is safer than indexing
    // B - very interesting that the order of accounts input is important
    // i guess this makes sense but i could see a contract helper that
    // allows for unordered account input maybe
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {

        msg!("Greeted account does not have the correct program id");
        msg!("Account owner is {account.owner} but the program id is {program_id}")

        // B - I have a feeling it will take me a long time to become comfortable with
        // error handling
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted

    // B - hmmmmm is .borrow() a native rust function for handling the data pointer?
    // and is try_from_slice a function from borsch thats used by the GreetingAccount struct?
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;

    // B - here is the core logic -> later do something creative here
    greeting_account.counter += 1;

    // B - oh lord the mut mut borrow mut
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Greeted {} time(s)!", greeting_account.counter);

    Ok(())

    // B - damn, I cant wait to look back on this like how dumb I was
    // I hate that it takes me a little while to lern new syntax.. feel like an idiot
}

// Sanity tests

// B - how is this text code handled at deployment?
// do builds ignore all code under the `#[cfg(test)]` declaration?
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
                .counter,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );
    }
}
