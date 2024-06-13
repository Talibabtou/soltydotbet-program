import BN from "bn.js";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { TestClient } from "./test_client";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import * as spl from "@solana/spl-token";
import { expect, assert } from "chai";
import BN from "bn.js";
import type { Error } from "../target/types/error";

describe("perpetuals", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Error as anchor.Program<Error>;
  
  let tc = new TestClient();
  tc.printErrors = true;
  let oracleConfig;
  let pricing;
  let permissions;
  let fees;
  let borrowRate;
  let ratios;
  let isStable;
  let isVirtual;
  let perpetualsExpected;
  let multisigExpected;
  let tokenExpected;
  let positionExpected;

  it("init", async () => {
    await tc.initFixture();
    await tc.init();

    let err = await tc.ensureFails(tc.init());
    assert(err.logs[3].includes("already in use"));

    perpetualsExpected = {
      permissions: {
        allowSwap: true,
        allowAddLiquidity: true,
        allowRemoveLiquidity: true,
        allowOpenPosition: true,
        allowClosePosition: true,
        allowPnlWithdrawal: true,
        allowCollateralWithdrawal: true,
        allowSizeChange: true,
      },
      pools: [],
      transferAuthorityBump: tc.authority.bump,
      perpetualsBump: tc.perpetuals.bump,
      inceptionTime: new BN(0),
    };

    multisigExpected = {
      numSigners: 2,
      numSigned: 0,
      minSignatures: 2,
      instructionAccountsLen: 0,
      instructionDataLen: 0,
      instructionHash: new anchor.BN(0),
      signers: [
        tc.admins[0].publicKey,
        tc.admins[1].publicKey,
        PublicKey.default,
        PublicKey.default,
        PublicKey.default,
        PublicKey.default,
      ],
      signed: [0, 0, 0, 0, 0, 0],
      bump: tc.multisig.bump,
    };

    let multisig = await tc.program.account.multisig.fetch(
      tc.multisig.publicKey
    );
    expect(JSON.stringify(multisig)).to.equal(JSON.stringify(multisigExpected));

    let perpetuals = await tc.program.account.perpetuals.fetch(
      tc.perpetuals.publicKey
    );
    expect(JSON.stringify(perpetuals)).to.equal(
      JSON.stringify(perpetualsExpected)
    );
  });

  it("setAdminSigners", async () => {
    await tc.setAdminSigners(1);

    let multisig = await tc.program.account.multisig.fetch(
      tc.multisig.publicKey
    );
    multisigExpected.minSignatures = 1;
    expect(JSON.stringify(multisig)).to.equal(JSON.stringify(multisigExpected));
  });

  it("setPermissions", async () => {
    perpetualsExpected.permissions = {
      allowSwap: true,
      allowAddLiquidity: true,
      allowRemoveLiquidity: true,
      allowOpenPosition: true,
      allowClosePosition: true,
      allowPnlWithdrawal: true,
      allowCollateralWithdrawal: true,
      allowSizeChange: true,
    };
    await tc.setPermissions(perpetualsExpected.permissions);

    let perpetuals = await tc.program.account.perpetuals.fetch(
      tc.perpetuals.publicKey
    );
    expect(JSON.stringify(perpetuals)).to.equal(
      JSON.stringify(perpetualsExpected)
    );
  });

  it("addAndRemovePool", async () => {
    await tc.addPool("test pool");

    let pool = await tc.program.account.pool.fetch(tc.pool.publicKey);
    let poolExpected = {
      name: "test pool",
      custodies: [],
      ratios: [],
      aumUsd: new BN(0),
      bump: tc.pool.bump,
      lpTokenBump: pool.lpTokenBump,
      inceptionTime: new BN(0),
    };
    expect(JSON.stringify(pool)).to.equal(JSON.stringify(poolExpected));

    await tc.removePool();
    await tc.ensureFails(tc.program.account.pool.fetch(tc.pool.publicKey));

    await tc.addPool("test pool");
  });

  it("addAndRemoveCustody", async () => {
    oracleConfig = {
      maxPriceError: new BN(10000),
      maxPriceAgeSec: 60,
      oracleType: { custom: {} },
      oracleAccount: tc.custodies[0].oracleAccount,
      oracleAuthority: tc.oracleAuthority.publicKey,
    };
    pricing = {
      useEma: true,
      useUnrealizedPnlInAum: true,
      tradeSpreadLong: new BN(100),
      tradeSpreadShort: new BN(100),
      swapSpread: new BN(200),
      minInitialLeverage: new BN(10000),
      maxInitialLeverage: new BN(1000000),
      maxLeverage: new BN(1000000),
      maxPayoffMult: new BN(10000),
      maxUtilization: new BN(10000),
      maxPositionLockedUsd: new BN(1000000000),
      maxTotalLockedUsd: new BN(1000000000),
    };
    permissions = {
      allowSwap: true,
      allowAddLiquidity: true,
      allowRemoveLiquidity: true,
      allowOpenPosition: true,
      allowClosePosition: true,
      allowPnlWithdrawal: true,
      allowCollateralWithdrawal: true,
      allowSizeChange: true,
    };
    fees = {
      mode: { linear: {} },
      ratioMult: new BN(20000),
      utilizationMult: new BN(20000),
      swapIn: new BN(100),
      swapOut: new BN(100),
      stableSwapIn: new BN(100),
      stableSwapOut: new BN(100),
      addLiquidity: new BN(100),
      removeLiquidity: new BN(100),
      openPosition: new BN(100),
      closePosition: new BN(100),
      liquidation: new BN(100),
      protocolShare: new BN(10),
      feeMax: new BN(250),
      feeOptimal: new BN(10),
    };
    borrowRate = {
      baseRate: new BN(0),
      slope1: new BN(80000),
      slope2: new BN(120000),
      optimalUtilization: new BN(800000000),
    };
    ratios = [
      {
        target: new BN(5000),
        min: new BN(10),
        max: new BN(10000),
      },
      {
        target: new BN(5000),
        min: new BN(10),
        max: new BN(10000),
      },
    ];
    let ratios1 = [
      {
        target: new BN(10000),
        min: new BN(10),
        max: new BN(10000),
      },
    ];
    isStable = false;
    isVirtual = false;
    await tc.addCustody(
      tc.custodies[0],
      isStable,
      isVirtual,
      oracleConfig,
      pricing,
      permissions,
      fees,
      borrowRate,
      ratios1
    );

    let token = await tc.program.account.custody.fetch(tc.custodies[0].custody);
    tokenExpected = {
      pool: tc.pool.publicKey,
      mint: tc.custodies[0].mint.publicKey,
      tokenAccount: tc.custodies[0].tokenAccount,
      decimals: 9,
      isStable,
      isVirtual,
      oracle: {
        oracleAccount: tc.custodies[0].oracleAccount,
        oracleType: { custom: {} },
        oracleAuthority: tc.oracleAuthority.publicKey,
        maxPriceError: "10000",
        maxPriceAgeSec: 60,
      },
      pricing: {
        useEma: true,
        useUnrealizedPnlInAum: true,
        tradeSpreadLong: "100",
        tradeSpreadShort: "100",
        swapSpread: "200",
        minInitialLeverage: "10000",
        maxInitialLeverage: "1000000",
        maxLeverage: "1000000",
        maxPayoffMult: "10000",
        maxUtilization: "10000",
        maxPositionLockedUsd: "1000000000",
        maxTotalLockedUsd: "1000000000",
      },
      permissions: {
        allowSwap: true,
        allowAddLiquidity: true,
        allowRemoveLiquidity: true,
        allowOpenPosition: true,
        allowClosePosition: true,
        allowPnlWithdrawal: true,
        allowCollateralWithdrawal: true,
        allowSizeChange: true,
      },
      fees: {
        mode: { linear: {} },
        ratioMult: "20000",
        utilizationMult: "20000",
        swapIn: "100",
        swapOut: "100",
        stableSwapIn: "100",
        stableSwapOut: "100",
        addLiquidity: "100",
        removeLiquidity: "100",
        openPosition: "100",
        closePosition: "100",
        liquidation: "100",
        protocolShare: "10",
        feeMax: "250",
        feeOptimal: "10",
      },
      borrowRate: {
        baseRate: "0",
        slope1: "80000",
        slope2: "120000",
        optimalUtilization: "800000000",
      },
      assets: {
        collateral: "0",
        protocolFees: "0",
        owned: "0",
        locked: "0",
      },
      collectedFees: {
        swapUsd: "0",
        addLiquidityUsd: "0",
        removeLiquidityUsd: "0",
        openPositionUsd: "0",
        closePositionUsd: "0",
        liquidationUsd: "0",
      },
      volumeStats: {
        swapUsd: "0",
        addLiquidityUsd: "0",
        removeLiquidityUsd: "0",
        openPositionUsd: "0",
        closePositionUsd: "0",
        liquidationUsd: "0",
      },
      tradeStats: {
        profitUsd: "0",
        lossUsd: "0",
        oiLongUsd: "0",
        oiShortUsd: "0",
      },
      longPositions: {
        openPositions: "0",
        collateralUsd: "0",
        sizeUsd: "0",
        borrowSizeUsd: "0",
        lockedAmount: "0",
        weightedPrice: "0",
        totalQuantity: "0",
        cumulativeInterestUsd: "0",
        cumulativeInterestSnapshot: "0",
      },
      shortPositions: {
        openPositions: "0",
        collateralUsd: "0",
        sizeUsd: "0",
        borrowSizeUsd: "0",
        lockedAmount: "0",
        weightedPrice: "0",
        totalQuantity: "0",
        cumulativeInterestUsd: "0",
        cumulativeInterestSnapshot: "0",
      },
      borrowRateState: {
        currentRate: "0",
        cumulativeInterest: "0",
        lastUpdate: "0",
      },
      bump: token.bump,
      tokenAccountBump: token.tokenAccountBump,
    };
    expect(JSON.stringify(token)).to.equal(JSON.stringify(tokenExpected));

    let oracleConfig2 = Object.assign({}, oracleConfig);
    oracleConfig2.oracleAccount = tc.custodies[1].oracleAccount;
    await tc.addCustody(
      tc.custodies[1],
      isStable,
      isVirtual,
      oracleConfig2,
      pricing,
      permissions,
      fees,
      borrowRate,
      ratios
    );

    await tc.removeCustody(tc.custodies[1], ratios1);
    await tc.ensureFails(
      tc.program.account.custody.fetch(tc.custodies[1].custody)
    );

    await tc.addCustody(
      tc.custodies[1],
      isStable,
      isVirtual,
      oracleConfig2,
      pricing,
      permissions,
      fees,
      borrowRate,
      ratios
    );
  });

  it("setCustodyConfig", async () => {
    oracleConfig.maxPriceAgeSec = 90;
    permissions.allowPnlWithdrawal = false;
    fees.liquidation = new BN(200);
    ratios[0].min = new BN(90);
    await tc.setCustodyConfig(
      tc.custodies[0],
      isStable,
      isVirtual,
      oracleConfig,
      pricing,
      permissions,
      fees,
      borrowRate,
      ratios
    );

    let token = await tc.program.account.custody.fetch(tc.custodies[0].custody);
    tokenExpected.oracle.maxPriceAgeSec = 90;
    tokenExpected.permissions.allowPnlWithdrawal = false;
    tokenExpected.fees.liquidation = "200";
    expect(JSON.stringify(token)).to.equal(JSON.stringify(tokenExpected));
  });

  it("setCustomOraclePrice", async () => {
    await tc.setCustomOraclePrice(123, tc.custodies[0]);
    await tc.setCustomOraclePrice(200, tc.custodies[1]);

    let oracle = await tc.program.account.customOracle.fetch(
      tc.custodies[0].oracleAccount
    );
    let oracleExpected = {
      price: new BN(123000),
      expo: -3,
      conf: new BN(0),
      ema: new BN(123000),
      publishTime: oracle.publishTime,
    };
    expect(JSON.stringify(oracle)).to.equal(JSON.stringify(oracleExpected));
  });

  it("setCustomOraclePricePermissionless", async () => {
    await tc.setCustomOraclePricePermissionless(
      tc.oracleAuthority,
      500,
      tc.custodies[0]
    );

    let oracle = await tc.program.account.customOracle.fetch(
      tc.custodies[0].oracleAccount
    );
    let oracleExpected = {
      price: new BN(500000),
      expo: -3,
      conf: new BN(10),
      ema: new BN(500000),
      publishTime: oracle.publishTime,
    };
    expect(JSON.stringify(oracle)).to.equal(JSON.stringify(oracleExpected));

    // Updating the permissionless price oracle with an older publish time should no-op.
    await tc.setCustomOraclePricePermissionless(
      tc.oracleAuthority,
      400,
      tc.custodies[0],
      tc.getTime() - 20
    );
    oracle = await tc.program.account.customOracle.fetch(
      tc.custodies[0].oracleAccount
    );
    // Oracle's value is still 500 instead of the attempted 400.
    expect(JSON.stringify(oracle)).to.equal(JSON.stringify(oracleExpected));

    // Try permissionless oracle update with increased & priority compute.
    await tc.setCustomOraclePricePermissionless(
      tc.oracleAuthority,
      1000,
      tc.custodies[0],
      tc.getTime() + 10,
      null,
      null,
      true
    );
    oracle = await tc.program.account.customOracle.fetch(
      tc.custodies[0].oracleAccount
    );
    expect(JSON.stringify(oracle)).to.equal(
      JSON.stringify({
        ...oracleExpected,
        price: new BN(1000000),
        ema: new BN(1000000),
        publishTime: oracle.publishTime,
      })
    );

    // after test, set price back to the expected for other test cases.
    await tc.setCustomOraclePricePermissionless(
      tc.oracleAuthority,
      123,
      tc.custodies[0],
      tc.getTime() + 20
    );
  });

  it("setCustomOraclePricePermissionless Errors", async () => {
    // Attempting to update with a payload signed by a bogus key should fail.
    let bogusKeypair = Keypair.generate();
    await tc.ensureFails(
      tc.setCustomOraclePricePermissionless(bogusKeypair, 100, tc.custodies[1])
    );

    // Sending the permissionless update without signature verification should fail.
    await tc.ensureFails(
      tc.setCustomOraclePricePermissionless(
        tc.oracleAuthority,
        100,
        tc.custodies[1],
        null,
        true
      )
    );

    // Sending the permissionless update with malformed message should fail.
    let randomMessage = Buffer.alloc(68);
    randomMessage.fill(0xab);
    await tc.ensureFails(
      tc.setCustomOraclePricePermissionless(
        tc.oracleAuthority,
        100,
        tc.custodies[1],
        null,
        null,
        randomMessage
      )
    );
  });

  it("setTestTime", async () => {
    await tc.setTestTime(111);

    let perpetuals = await tc.program.account.perpetuals.fetch(
      tc.perpetuals.publicKey
    );
    expect(JSON.stringify(perpetuals.inceptionTime)).to.equal(
      JSON.stringify(new BN(111))
    );
  });

  it("addLiquidity", async () => {
    await tc.addLiquidity(
      tc.toTokenAmount(10, tc.custodies[0].decimals),
      new BN(1),
      tc.users[0],
      tc.users[0].tokenAccounts[0],
      tc.custodies[0]
    );
    await tc.addLiquidity(
      tc.toTokenAmount(10, tc.custodies[1].decimals),
      new BN(1),
      tc.users[1],
      tc.users[1].tokenAccounts[1],
      tc.custodies[1]
    );
  });

  it("swap", async () => {
    await tc.swap(
      tc.toTokenAmount(1, tc.custodies[0].decimals),
      new BN(1),
      tc.users[0],
      tc.users[0].tokenAccounts[0],
      tc.users[0].tokenAccounts[1],
      tc.custodies[0],
      tc.custodies[1]
    );
  });

  it("removeLiquidity", async () => {
    await tc.removeLiquidity(
      tc.toTokenAmount(1, 6),
      new BN(1),
      tc.users[0],
      tc.users[0].tokenAccounts[0],
      tc.custodies[0]
    );
    await tc.removeLiquidity(
      tc.toTokenAmount(1, 6),
      new BN(1),
      tc.users[1],
      tc.users[1].tokenAccounts[1],
      tc.custodies[1]
    );
  });

  it("openPosition", async () => {
    await tc.openPosition(
      125,
      tc.toTokenAmount(1, tc.custodies[0].decimals),
      tc.toTokenAmount(7, tc.custodies[0].decimals),
      "long",
      tc.users[0],
      tc.users[0].tokenAccounts[0],
      tc.users[0].positionAccountsLong[0],
      tc.custodies[0]
    );

    let position = await tc.program.account.position.fetch(
      tc.users[0].positionAccountsLong[0]
    );
    positionExpected = {
      owner: tc.users[0].wallet.publicKey.toBase58(),
      pool: tc.pool.publicKey.toBase58(),
      custody: tc.custodies[0].custody.toBase58(),
      collateralCustody: tc.custodies[0].custody.toBase58(),
      openTime: "111",
      updateTime: "0",
      side: { long: {} },
      price: "124230000",
      sizeUsd: "869610000",
      borrowSizeUsd: "869610000",
      collateralUsd: "123000000",
      unrealizedProfitUsd: "0",
      unrealizedLossUsd: "0",
      cumulativeInterestSnapshot: "0",
      lockedAmount: "7000000000",
      collateralAmount: "1000000000",
      bump: position.bump,
    };

    expect(JSON.stringify(position)).to.equal(JSON.stringify(positionExpected));
  });

  it("addCollateral", async () => {
    await tc.addCollateral(
      tc.toTokenAmount(1, tc.custodies[0].decimals),
      tc.users[0],
      tc.users[0].tokenAccounts[0],
      tc.users[0].positionAccountsLong[0],
      tc.custodies[0]
    );
  });

  it("removeCollateral", async () => {
    await tc.removeCollateral(
      tc.toTokenAmount(1, 6),
      tc.users[0],
      tc.users[0].tokenAccounts[0],
      tc.users[0].positionAccountsLong[0],
      tc.custodies[0]
    );
  });

  it("closePosition", async () => {
    await tc.closePosition(
      1,
      tc.users[0],
      tc.users[0].tokenAccounts[0],
      tc.users[0].positionAccountsLong[0],
      tc.custodies[0]
    );
    await tc.ensureFails(
      tc.program.account.position.fetch(tc.users[0].positionAccountsLong[0])
    );
  });

  it("liquidate", async () => {
    await tc.openPosition(
      125,
      tc.toTokenAmount(1, tc.custodies[0].decimals),
      tc.toTokenAmount(7, tc.custodies[0].decimals),
      "long",
      tc.users[0],
      tc.users[0].tokenAccounts[0],
      tc.users[0].positionAccountsLong[0],
      tc.custodies[0]
    );
    await tc.setCustomOraclePrice(80, tc.custodies[0]);
    await tc.liquidate(
      tc.users[0],
      tc.users[0].tokenAccounts[0],
      tc.users[0].positionAccountsLong[0],
      tc.custodies[0]
    );
    await tc.ensureFails(
      tc.program.account.position.fetch(tc.users[0].positionAccountsLong[0])
    );
  });
});