//Import statement
use anchor_lang::prelude::*;

//Rust Macro - Takes in 1 argument 
declare_id!("6Nda2quCikwxyC61gjjNtonsAkoHEAwXupnyXERepw5m");
//Original key copied from code- 7FBBdRPMWb7eE5pRdpi3GRCNjxwFNwv33BGz3PNHb4bt

#[program]
pub mod counter {
    use super::*;
//Below are the structs for the ins tructions
//Init function allows to set up the structure of the app - eg = Accounts
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        //Get reference to the counter account
        let counter = &mut ctx.accounts.counter;
        counter.bump = ctx.bumps.counter; // store bump seed in `Counter` account
        msg!("Counter account created! Current count: {}", counter.count);
        msg!("Counter bump: {}", counter.bump);
        Ok(()) // Returns a 200 status
    }
//Increments the value of the counter
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        //Mut reference to change the value stored in the counter account
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count); //Log current value
        counter.count = counter.count.checked_add(1).unwrap();  // increment an unsigned integer
        msg!("Counter incremented! Current count: {}", counter.count);
        Ok(()) //Returns a 200 status
    }
}

//Structs for the initialize instruction
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // Account who will sign the transaction

    // Create and initialize `Counter` account using a PDA as the address
    //Below is an anchor attribute that generates more code
    //Anchor management that creates, manages and allocate the space of the accounts
    //Generate a PDA
    #[account(
        init,//Init account
        seeds = [b"counter"], // optional seeds for pda
        bump,                 // bump seed for pda
        payer = user,//User is paying the account
        space = 8 + Counter::INIT_SPACE //Space being taken by the account - Usage based cost
    )]
    //Data type definition
    pub counter: Account<'info, Counter>,//Specify account is 'Counter' type
    pub system_program: Program<'info, System>, // Specify account must be system program
}


//Structs for the Increment instruction
#[derive(Accounts)]
pub struct Increment<'info> {
    // The address of the `Counter` account must be a PDA derived with the specified `seeds`
    //Generate a PDA
    #[account(mut, /*We tell anchor that we will pass in a mutable account because we are updating its data*/ 
        seeds = [b"counter"], // optional seeds for pda
        bump = counter.bump,  // bump seed for pda stored in `Counter` account
    )]
    pub counter: Account<'info, Counter>, // Specify account being a 'Counter' type
}

//Account that will store the data of our account
#[account] // Rust attribute that tells anchor that this is an account
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64, // 8 bytes -  define count value type as u64 *Stores the data for our counter*
    pub bump: u8,   // 1 byte - A number used to go from a key pair to a PDA
}