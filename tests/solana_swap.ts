import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaSwap } from "../target/types/solana_swap";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  createAccount,
  mintTo,
  getAccount,
  getAssociatedTokenAddress,
  createAssociatedTokenAccount,
} from "@solana/spl-token";
import { assert } from "chai";
import { Keypair } from "@solana/web3.js";

describe("solana_swap", () => {
  // 配置Anchor提供者
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // 获取程序接口
  const program = anchor.workspace.SolanaSwap as Program<SolanaSwap>;

  // 定义测试变量
  let tokenAMint: anchor.web3.PublicKey = null;
  let tokenBMint: anchor.web3.PublicKey = null;
  let poolTokenA: anchor.web3.PublicKey = null;
  let poolTokenB: anchor.web3.PublicKey = null;
  let userTokenA: anchor.web3.PublicKey = null;
  let userTokenB: anchor.web3.PublicKey = null;
  let poolAuthority: anchor.web3.PublicKey = null;
  let poolAuthorityBump: number = null;

  // 在所有测试之前进行初始化设置
  before(async () => {
    console.log("正在设置测试环境...");

    try {
      // 创建代币A和代币B的铸币厂，使用随机生成的密钥对
      const mintAKeypair = Keypair.generate();
      const mintBKeypair = Keypair.generate();

      tokenAMint = await createMint(
        provider.connection,
        provider.wallet.payer,
        provider.wallet.publicKey,
        null,
        9,
        mintAKeypair
      );
      console.log(`创建代币A铸币厂: ${tokenAMint.toString()}`);

      tokenBMint = await createMint(
        provider.connection,
        provider.wallet.payer,
        provider.wallet.publicKey,
        null,
        9,
        mintBKeypair
      );
      console.log(`创建代币B铸币厂: ${tokenBMint.toString()}`);

      // 找到池子权限PDA
      [poolAuthority, poolAuthorityBump] =
        await anchor.web3.PublicKey.findProgramAddress(
          [Buffer.from("pool_authority")],
          program.programId
        );
      console.log(
        `池子权限PDA: ${poolAuthority.toString()}, Bump: ${poolAuthorityBump}`
      );

      // 为用户创建代币账户
      userTokenA = await createAccount(
        provider.connection,
        provider.wallet.payer,
        tokenAMint,
        provider.wallet.publicKey,
        Keypair.generate()
      );
      console.log(`用户代币A账户: ${userTokenA.toString()}`);

      userTokenB = await createAccount(
        provider.connection,
        provider.wallet.payer,
        tokenBMint,
        provider.wallet.publicKey,
        Keypair.generate()
      );
      console.log(`用户代币B账户: ${userTokenB.toString()}`);

      // 铸造一些代币给用户
      console.log("向用户铸造代币...");
      await mintTo(
        provider.connection,
        provider.wallet.payer,
        tokenAMint,
        userTokenA,
        provider.wallet.publicKey,
        1000000000
      );

      await mintTo(
        provider.connection,
        provider.wallet.payer,
        tokenBMint,
        userTokenB,
        provider.wallet.publicKey,
        1000000000
      );

      // 找到代币池子PDA
      [poolTokenA] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("token_pool_a"), tokenAMint.toBuffer()],
        program.programId
      );
      console.log(`代币A池子PDA: ${poolTokenA.toString()}`);

      [poolTokenB] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("token_pool_b"), tokenBMint.toBuffer()],
        program.programId
      );
      console.log(`代币B池子PDA: ${poolTokenB.toString()}`);
    } catch (err) {
      console.error("设置测试环境时出错:", err);
      throw err;
    }
  });

  it("初始化交换池", async () => {
    console.log("测试: 初始化交换池");
    try {
      await program.methods
        .initialize()
        .accounts({
          poolAuthority: poolAuthority,
          poolTokenA: poolTokenA,
          poolTokenB: poolTokenB,
          tokenAMint: tokenAMint,
          tokenBMint: tokenBMint,
          admin: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .rpc();

      console.log("交换池初始化成功");

      // 向池子注入流动性
      console.log("向交换池注入流动性...");
      await mintTo(
        provider.connection,
        provider.wallet.payer,
        tokenAMint,
        poolTokenA,
        provider.wallet.publicKey,
        1000000000
      );

      await mintTo(
        provider.connection,
        provider.wallet.payer,
        tokenBMint,
        poolTokenB,
        provider.wallet.publicKey,
        1000000000
      );

      console.log("交换池账户数据验证成功");
    } catch (err) {
      console.error("初始化交换池时出错:", err);
      throw err;
    }
  });

  it("交换代币A为代币B", async () => {
    console.log("测试: 交换代币A为代币B");
    const amountToSwap = new anchor.BN(100000000); // 交换1亿个代币单位

    try {
      // 获取交换前的余额
      const userTokenABefore = await getAccount(
        provider.connection,
        userTokenA
      );
      const userTokenBBefore = await getAccount(
        provider.connection,
        userTokenB
      );
      const poolTokenABefore = await getAccount(
        provider.connection,
        poolTokenA
      );
      const poolTokenBBefore = await getAccount(
        provider.connection,
        poolTokenB
      );

      console.log(`交换前用户代币A余额: ${userTokenABefore.amount}`);
      console.log(`交换前用户代币B余额: ${userTokenBBefore.amount}`);
      console.log(`交换前池子代币A余额: ${poolTokenABefore.amount}`);
      console.log(`交换前池子代币B余额: ${poolTokenBBefore.amount}`);

      // 执行交换
      await program.methods
        .swapAToB(amountToSwap)
        .accounts({
          poolAuthority: poolAuthority,
          poolTokenA: poolTokenA,
          poolTokenB: poolTokenB,
          userTokenA: userTokenA,
          userTokenB: userTokenB,
          tokenAMint: tokenAMint,
          tokenBMint: tokenBMint,
          user: provider.wallet.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();

      console.log(`成功交换 ${amountToSwap} 代币A为代币B`);

      // 获取交换后的余额
      const userTokenAAfter = await getAccount(provider.connection, userTokenA);
      const userTokenBAfter = await getAccount(provider.connection, userTokenB);
      const poolTokenAAfter = await getAccount(provider.connection, poolTokenA);
      const poolTokenBAfter = await getAccount(provider.connection, poolTokenB);

      console.log(`交换后用户代币A余额: ${userTokenAAfter.amount}`);
      console.log(`交换后用户代币B余额: ${userTokenBAfter.amount}`);
      console.log(`交换后池子代币A余额: ${poolTokenAAfter.amount}`);
      console.log(`交换后池子代币B余额: ${poolTokenBAfter.amount}`);

      // 验证余额变化
      assert.equal(
        BigInt(userTokenABefore.amount) - BigInt(userTokenAAfter.amount),
        BigInt(amountToSwap.toString())
      );
      assert.equal(
        BigInt(userTokenBAfter.amount) - BigInt(userTokenBBefore.amount),
        BigInt(amountToSwap.toString())
      );
      assert.equal(
        BigInt(poolTokenAAfter.amount) - BigInt(poolTokenABefore.amount),
        BigInt(amountToSwap.toString())
      );
      assert.equal(
        BigInt(poolTokenBBefore.amount) - BigInt(poolTokenBAfter.amount),
        BigInt(amountToSwap.toString())
      );

      console.log("代币A兑换代币B测试验证成功");
    } catch (err) {
      console.error("交换代币A为代币B时出错:", err);
      throw err;
    }
  });

  it("交换代币B为代币A", async () => {
    console.log("测试: 交换代币B为代币A");
    const amountToSwap = new anchor.BN(100000000); // 交换1亿个代币单位

    try {
      // 获取交换前的余额
      const userTokenABefore = await getAccount(
        provider.connection,
        userTokenA
      );
      const userTokenBBefore = await getAccount(
        provider.connection,
        userTokenB
      );
      const poolTokenABefore = await getAccount(
        provider.connection,
        poolTokenA
      );
      const poolTokenBBefore = await getAccount(
        provider.connection,
        poolTokenB
      );

      console.log(`交换前用户代币A余额: ${userTokenABefore.amount}`);
      console.log(`交换前用户代币B余额: ${userTokenBBefore.amount}`);
      console.log(`交换前池子代币A余额: ${poolTokenABefore.amount}`);
      console.log(`交换前池子代币B余额: ${poolTokenBBefore.amount}`);

      // 执行交换
      await program.methods
        .swapBToA(amountToSwap)
        .accounts({
          poolAuthority: poolAuthority,
          poolTokenA: poolTokenA,
          poolTokenB: poolTokenB,
          userTokenA: userTokenA,
          userTokenB: userTokenB,
          tokenAMint: tokenAMint,
          tokenBMint: tokenBMint,
          user: provider.wallet.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();

      console.log(`成功交换 ${amountToSwap} 代币B为代币A`);

      // 获取交换后的余额
      const userTokenAAfter = await getAccount(provider.connection, userTokenA);
      const userTokenBAfter = await getAccount(provider.connection, userTokenB);
      const poolTokenAAfter = await getAccount(provider.connection, poolTokenA);
      const poolTokenBAfter = await getAccount(provider.connection, poolTokenB);

      console.log(`交换后用户代币A余额: ${userTokenAAfter.amount}`);
      console.log(`交换后用户代币B余额: ${userTokenBAfter.amount}`);
      console.log(`交换后池子代币A余额: ${poolTokenAAfter.amount}`);
      console.log(`交换后池子代币B余额: ${poolTokenBAfter.amount}`);

      // 验证余额变化
      assert.equal(
        BigInt(userTokenBBefore.amount) - BigInt(userTokenBAfter.amount),
        BigInt(amountToSwap.toString())
      );
      assert.equal(
        BigInt(userTokenAAfter.amount) - BigInt(userTokenABefore.amount),
        BigInt(amountToSwap.toString())
      );
      assert.equal(
        BigInt(poolTokenBAfter.amount) - BigInt(poolTokenBBefore.amount),
        BigInt(amountToSwap.toString())
      );
      assert.equal(
        BigInt(poolTokenABefore.amount) - BigInt(poolTokenAAfter.amount),
        BigInt(amountToSwap.toString())
      );

      console.log("代币B兑换代币A测试验证成功");
    } catch (err) {
      console.error("交换代币B为代币A时出错:", err);
      throw err;
    }
  });
});
