import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaOnChainPortfolio } from "../target/types/solana_on_chain_portfolio";

describe("solana_on_chain_portfolio", () => {
  const {
    LAMPORTS_PER_SOL
  } = anchor.web3
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaOnChainPortfolio as Program<SolanaOnChainPortfolio>;

  const provider = anchor.getProvider()
  const users = []

  before(async () => {

    await (async () => {
      {
        const payer = anchor.web3.Keypair.generate()
        const blockhash = await provider.connection.getLatestBlockhash()
        const tx = await provider.connection.requestAirdrop(payer.publicKey, 1000 * LAMPORTS_PER_SOL)

        await provider.connection.confirmTransaction({
          ...blockhash,
          signature: tx
        }, 'confirmed')

        users.push({ keypair: payer, type: 'developer' })
      }
    })()

    await (async () => {
      {
        const payer = anchor.web3.Keypair.generate()
        const blockhash = await provider.connection.getLatestBlockhash()
        const tx = await provider.connection.requestAirdrop(payer.publicKey, 1000 * LAMPORTS_PER_SOL)

        await provider.connection.confirmTransaction({
          ...blockhash,
          signature: tx
        }, 'confirmed')

        users.push({ keypair: payer, type: 'employer' })
      }
    })()

  })

  it("Is initialized!", async () => {

    const payer = users[0]

    const [authority] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("authority")],
      program.programId,
    )


    const tx = await program.methods
      .initialize()
      .accounts({
        payer: payer.keypair.publicKey,
        newProgramHeader: authority,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([payer.keypair])
      .rpc()

    const blockhash = await provider.connection.getLatestBlockhash()

    await provider.connection.confirmTransaction({
      ...blockhash,
      signature: tx
    }, 'confirmed')

  })
})
