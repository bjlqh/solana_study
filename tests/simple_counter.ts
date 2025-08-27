import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SimpleCounter } from "../target/types/simple_counter";
import { expect } from "chai";


describe("simple_counter", () => {

    //配置客户端使用本地集群Anchor.toml
    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace.SimpleCounter as Program<SimpleCounter>;
    const provider = anchor.getProvider();

    //生成一个新的秘钥对作为用户
    const user = anchor.web3.Keypair.generate();

    //计算PDA地址
    const [counterPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("counter"),
            user.publicKey.toBuffer()
        ],
        program.programId
    );


    before(async () => {
        //为用户账户重置SOL用于支付交易费用
        const signature = await provider.connection.requestAirdrop(
            user.publicKey,
            2 * anchor.web3.LAMPORTS_PER_SOL
        );
        //await provider.connection.confirmTransaction(signature);
        const latestBlockHash = await provider.connection.getLatestBlockhash();
        await provider.connection.confirmTransaction({
            signature,
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight
        });
    })

    //初始化测试，验证计数器是否初始化为0
    it("初始化计数器", async () => {
        const tx = await program.methods.initialize()
        .accounts({
            counter: counterPda,
            user: user.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        }).signers([user]).rpc();
        console.log("Initialize transaction signature:", tx);
        
        //获取计数器账户数据
        const counterAccount = await program.account.counter.fetch(counterPda);
    
        //验证计数器初始值为0
        expect(counterAccount.count.toNumber()).to.equal(0);
    })

    //增量测试，验证increment功能正常工作
    it("增加计数器", async () => {
        const tx = await program.methods.increment()
        .accounts({
            counter: counterPda,
            user: user.publicKey
        }).signers([user]).rpc();

        console.log("Increment transaction signature:", tx);

        //获取更新后的计数器账户数据
        const counterAccount = await program.account.counter.fetch(counterPda);

        //验证计数器值为1
        expect(counterAccount.count.toNumber()).to.equal(1);
    })


    //多次增量测试，验证连续调用increment函数
    it("多次增加计数器", async () => {
        //连续调用increment指令3次
        for(let i = 0; i < 3; i++){
            await program.methods.increment()
            .accounts({
                counter: counterPda,
                user: user.publicKey
            }).signers([user]).rpc();
        }

        const counterAccount = await program.account.counter.fetch(counterPda);
        // 验证计数器值为4 (初始0 + 第一次测试的1 + 这次的3)
        expect(counterAccount.count.toNumber()).to.equal(4);


        //再次初始化
        /**
        await program.methods.initialize()
        .accounts({
            counter: counterPda,
            user: user.publicKey
        }).signers([user]).rpc();

        const counterAccount2 = await program.account.counter.fetch(counterPda);
        expect(counterAccount2.count.toNumber()).to.equal(0);
        */
    });

    //PDA验证，确认程序派生地址计算正确
    it("验证PDA地址计算", async() => {
        const[expectedPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("counter"), user.publicKey.toBuffer()],
            program.programId
        );

        expect(expectedPda.toString()).to.equal(counterPda.toString());
        console.log("Counter PDA:",counterPda.toString());
        console.log("bump:",bump);
    })

    //多用户测试，验证不同用户有独立的计数器
    it("测试不同用户的独立计数器", async() => {
        const user2 = anchor.web3.Keypair.generate();

        //为user2账户重置SOL用于支付交易费用
        const signature = await provider.connection.requestAirdrop(
            user2.publicKey,
            2 * anchor.web3.LAMPORTS_PER_SOL
        );

        await provider.connection.confirmTransaction(signature);

        //计算user2的PDA地址
        const [user2Pda] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("counter"), user2.publicKey.toBuffer()],
            program.programId
        );

        //为第二个用户初始化计数器
        await program.methods.initialize()
        .accounts({
            counter: user2Pda,
            user: user2.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        }).signers([user2]).rpc();

        //验证user2的计数器初始值为0
        const user2Counter = await program.account.counter.fetch(user2Pda);
        expect(user2Counter.count.toNumber()).to.equal(0);

        //验证第一个用户的计数器仍是4
        const user1Counter = await program.account.counter.fetch(counterPda);
        expect(user1Counter.count.toNumber()).to.equal(4);
    })
})