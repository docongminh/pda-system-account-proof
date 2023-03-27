# pda-system-account-proof
Prove pda system account storage lamports really secure. Some people concern about pda account, that assigned for system program rather than itself program id is not secure. 
This 's a peace of code that prove that pda account assigned for system program always secure.

[Program](https://github.com/docongminh/pda-system-account-proof/tree/master/programs/pda) init pda account
```rust
   #[program]
   pub mod pda {
        use super::*;

        pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
            let bump = *ctx.bumps.get("escrow_account").unwrap();
            ctx.accounts.create_native_account_vault(bump)?;
            Ok(())
        }
   }
```

[Program](https://github.com/docongminh/pda-system-account-proof/tree/master/programs/hacker/programs/hacker) init hack pda account
```rust
    #[program]
    pub mod hacker {
        use super::*;

        pub fn drain(ctx: Context<Drain>) -> Result<()> {
            let bump = *ctx.bumps.get("escrow_account").unwrap();
            ctx.accounts.drain_sol(bump)?;
            Ok(())
        }
    }
``` 

## Step by Step

  - Deploy program:
    deploy pda:
    ```
      cd programs/pda
      anchor build && anchor deploy
    ```
  
  - Deploy hacker:
    ```
       cd prgrams/hacker
       anchor build && anchor deploy
    ```
  
  Notes: Update program in declare id in all program
  ```rust
      declare_id!("FcBBAczSDVtSwJ55RWfbGAwEQTyQ3Urh29UHi3qrFPJf");
  ```
  
  - Run client
    ```
      cd client && cargo build
    ```
    
    run test:
    ```bash
      /target/debug/client --pda-program-id programId --hacker-program-id programId
    ```
    
    Example:
    ```bash
      ./target/debug/client --pda-program-id DzSN8ZCEURUe3nXtUi23L4N6jCQT16B8GjfZQ1CkoQEh --hacker-program-id FcBBAczSDVtSwJ55RWfbGAwEQTyQ3Urh29UHi3qrFPJf
    ```
