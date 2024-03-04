import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaOnChainPortfolio } from "../target/types/solana_on_chain_portfolio";

import { createHash } from "crypto";


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
        const tx = await provider.connection.requestAirdrop(payer.publicKey, 10000 * LAMPORTS_PER_SOL)

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
        const tx = await provider.connection.requestAirdrop(payer.publicKey, 10000 * LAMPORTS_PER_SOL)

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


  it("Initialize Developer Profile", async () => {

    const payer = users[0]

    const [authority] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("authority")],
      program.programId,
    )

    const [devProfile] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("profile"),
        authority.toBuffer(),
        payer.keypair.publicKey.toBuffer()
      ],
      program.programId,
    )

    const [profileHeader] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("profile-header"),
        devProfile.toBuffer()
      ],
      program.programId,
    )


    const tx = await program.methods
      .initializeProfile()
      .accounts({
        authority: payer.keypair.publicKey,
        programAuthority: authority,
        newProfile: devProfile,
        newProfileHeader: profileHeader,
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


  it("Initialize Employer Profile", async () => {

    const payer = users[1]

    const [authority] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("authority")],
      program.programId,
    )

    const [employerProfile] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("profile"),
        authority.toBuffer(),
        payer.keypair.publicKey.toBuffer()
      ],
      program.programId,
    )

    const [profileHeader] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("profile-header"),
        employerProfile.toBuffer()
      ],
      program.programId,
    )


    const tx = await program.methods
      .initializeProfile()
      .accounts({
        authority: payer.keypair.publicKey,
        programAuthority: authority,
        newProfile: employerProfile,
        newProfileHeader: profileHeader,
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


  it("Initialize Developer Portfolio", async () => {

    const payer = users[0]

    const [authority] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("authority")],
      program.programId,
    )

    const [devProjectHeader] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("project-header"),
        authority.toBuffer(),
        payer.keypair.publicKey.toBuffer()
      ],
      program.programId,
    )

    const [devProfile] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("profile"),
        authority.toBuffer(),
        payer.keypair.publicKey.toBuffer()
      ],
      program.programId,
    )

    const [profileHeader] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("profile-header"),
        devProfile.toBuffer()
      ],
      program.programId,
    )


    const tx = await program.methods
      .initializePortfolio()
      .accounts({
        authority: payer.keypair.publicKey,
        programAuthority: authority,
        profileHeader: profileHeader,
        newProjectHeader: devProjectHeader,
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


  it('Update Developer Profile', async () => {

    const payer = users[0]

    const [authority] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("authority")],
      program.programId,
    )


    const [devProfile] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("profile"),
        authority.toBuffer(),
        payer.keypair.publicKey.toBuffer()
      ],
      program.programId,
    )


    const tx = await program.methods
      .updateProfile([
        {
          social: {
            data: [
              { append: { content: { field: "twitter", data: "@freedom.pk" } } },
              { append: { content: { field: "linkedin", data: "@freedom.pk" } } },
            ]
          }
        },

        {
          content: {
            data: [
              { append: { content: { field: "description", data: "I'm all about creating" } } },
              { append: { content: { field: "moto", data: "mutiny" } } },
            ]
          }
        },

        { picture: { data: "picture_url.com" } }
      ])
      .accounts({
        authority: payer.keypair.publicKey,
        profile: devProfile,
      })
      .signers([payer.keypair])
      .rpc()

    const blockhash = await provider.connection.getLatestBlockhash()

    await provider.connection.confirmTransaction({
      ...blockhash,
      signature: tx
    }, 'confirmed')
  })


  it('add project to developer portfolio', async () => {

    const payer = users[0]

    const [authority] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("authority")],
      program.programId,
    )


    const [devProjectHeader] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("project-header"),
        authority.toBuffer(),
        payer.keypair.publicKey.toBuffer()
      ],
      program.programId,
    )

    const account = await program.account.projectHeader.fetch(devProjectHeader)

    const data = Buffer.from([0, 0, 0, 0, 0, 0, 0, 0, ...account.nonce.toBuffer()])
    const nonce = Buffer.alloc(8)
    data.copy(nonce, 0, 8, 16)

    const [project] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("project"),
        authority.toBuffer(),
        payer.keypair.publicKey.toBuffer(),
        nonce
      ],
      program.programId,
    )


    const tx = await program.methods
      .addProject(
        "project_url.com",
        "this is a description of the project"
      )
      .accounts({
        authority: payer.keypair.publicKey,
        programAuthority: authority,
        projectHeader: devProjectHeader,
        newProject: project,
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


  it('update project', async () => {

    const payer = users[0]

    const [authority] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("authority")],
      program.programId,
    )


    const [devProjectHeader] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("project-header"),
        authority.toBuffer(),
        payer.keypair.publicKey.toBuffer()
      ],
      program.programId,
    )

    const account = await program.account.projectHeader.fetch(devProjectHeader)

    // const data = Buffer.from([0, 0, 0, 0, 0, 0, 0, 0, ...account.nonce.toBuffer()])
    const data = Buffer.from([0, 0, 0, 0, 0, 0, 0, 0, 0])

    const nonce = Buffer.alloc(8)
    data.copy(nonce, 0, 8, 16)

    const [project] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("project"),
        authority.toBuffer(),
        payer.keypair.publicKey.toBuffer(),
        nonce
      ],
      program.programId,
    )


    const tx = await program.methods
      .updateProject(
        "project_url.com",
        "this is a change in description of the project"
      )
      .accounts({
        authority: payer.keypair.publicKey,
        project: project,
      })
      .signers([payer.keypair])
      .rpc()

    const blockhash = await provider.connection.getLatestBlockhash()

    await provider.connection.confirmTransaction({
      ...blockhash,
      signature: tx
    }, 'confirmed')
  })


  it('employer init message header', async () => {

    const payer = users[1]
    const recipient = users[0]

    const [authority] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("authority")],
      program.programId,
    )

    const hash = createHash('sha256');

    if (payer.keypair.publicKey < recipient.keypair.publicKey) {
      hash.update(payer.keypair.publicKey.toBuffer())
      hash.update(recipient.keypair.publicKey.toBuffer())
    } else {
      hash.update(recipient.keypair.publicKey.toBuffer())
      hash.update(payer.keypair.publicKey.toBuffer())
    }

    const data = hash.digest()
    console.log(data)

    console.log(Buffer.from([...data]))

    const [messageHeader] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("message-header"),
        data
      ],
      program.programId,
    )


    const tx = await program.methods
      .initializeMessageHeader()
      .accounts({
        sender: payer.keypair.publicKey,
        recipient: recipient.keypair.publicKey,
        newMessageHeader: messageHeader,
        programAuthority: authority,
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


  it('Post Message', async () => {

    const payer = users[1]
    const recipient = users[0]

    const [authority] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("authority")],
      program.programId,
    )


    const hash = createHash('sha256');

    if (payer.keypair.publicKey < recipient.keypair.publicKey) {
      hash.update(payer.keypair.publicKey.toBuffer())
      hash.update(recipient.keypair.publicKey.toBuffer())
    } else {
      hash.update(recipient.keypair.publicKey.toBuffer())
      hash.update(payer.keypair.publicKey.toBuffer())
    }


    const [messageHeader] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("message-header"),
        Buffer.from([...hash.digest()])
      ],
      program.programId,
    )

    const [message] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("message"),
        messageHeader.toBuffer(),
        Buffer.from([0, 0, 0, 0, 0, 0, 0, 0])
      ],
      program.programId,
    )

    const [devProfile] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("profile"),
        authority.toBuffer(),
        recipient.keypair.publicKey.toBuffer(),
      ],
      program.programId,
    )

    const [devProfileHeader] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("profile-header"),
        devProfile.toBuffer()
      ],
      program.programId,
    )

    const tx = await program.methods
      .postMessage(
        { append: {} },
        "this is a message. BOOM!!"
      )
      .accounts({
        sender: payer.keypair.publicKey,
        recipient: recipient.keypair.publicKey,
        messageHeader: messageHeader,
        profileHeader: devProfileHeader,
        message: message,
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