const Decimal = require('decimal.js');
const moment = require('moment');
const Binance = require('binance-api-node').default
const { ApiPromise, WsProvider } = require('@polkadot/api');
const testKeyring = require('@polkadot/keyring/testing');
const BN = require('bn.js');
const [Alice, Charlie, BOB] = ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y", "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"]
const [Dave] = ["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"];
const snooze = ms => new Promise(resolve => setTimeout(resolve, ms));

async function main() {
  const provider = new WsProvider('ws://127.0.0.1:9944');
  let options = {
    provider,
    types: {
      Status: "Enum",
      DboxStatus: "Enum",
      "DboxOf": {
        "id": "H256",
        "create_position": "u64",
        "status": "DboxStatus",
        "value": "Balance",
        "version": "u64",
        "invitor": "Option<AccountId>",
        "open_position": "u64",
        "bonus_per_dbox": "Balance",
        "bonus_position": "u64"
      },
      "PlayerStatus": "Enum",
      "PlayerOf": {
        "total_bonus": "Balance",
        "total_prize": "Balance",
        "total_commission": "Balance",
        "status": "u8"
      },
      "PriceReport": {
        "reporter": "AccountId",
        "price": "Price"
      },
      "PendingRequest": {
        "id": "Hash",
        "expired_at": "BlockNumber"
      },
      "Price": "u128",
      "OracleStatus": "Enum",
      "OracleInfo": {
        "total_jobs": "u64",
        "total_missed_jobs": "u64",
        "total_witnessed_jobs": "u64",
        "total_reward": "Balance",
        "withdrawable_reward": "Balance",
        "total_slash": "Balance",
        "status": "u8"
      },
      "JobOf": {
        "from": "AccountId",
        "meta": "Vec<u8>",
        "created_at": "BlockNumber",
        "expired_at": "BlockNumber",
        "oracle": "AccountId",
        "reward": "Balance",
        "nonce": "u64"
      },
      "LedgerOf": {
        "locked": "Balance",
        "staked": "Balance",
        "unbonds": "Vec<Unbond>"
      },
      "Unbond": {
        "amount": "Balance",
        "until": "BlockNumber"
      }
    }
  };

  const api = await ApiPromise.create(options);
  const keyring = testKeyring.default();
  let key = keyring.getPair(Dave);
  let last_reported = null

  console.log("request price firstly");

  // Request price firstly
  const oracle = Dave; 
  const action = api.tx.price.requestPrice(oracle);
  let aliceKey = keyring.getPair(Alice);
  let rawNonce = await api.query.system.accountNonce(aliceKey.address);
  let nonce = new BN(rawNonce.toString());
  await action.signAndSend(aliceKey, { nonce }, ({ events = [], status }) => {
    console.log("requested price", oracle.toString(), status.toString());
  });

  const client = Binance();
  client.ws.ticker('BTCUSDT', async data => {
    let now = moment()
    console.log(moment.duration(now.diff(last_reported)).seconds(), data.eventType)

    if (data.eventType === "24hrTicker" && (last_reported === null || moment.duration(now.diff(last_reported)).seconds() > 30)) {
      console.log("pushing price", data.curDayClose.toString(), data)
      let price = new BN(new Decimal(data.curDayClose.toString()).mul(10000).round().toString())
      console.log("pushing price--", price.toString())
      // TODO: use parameter to hold hash value
      let hash = "0x11f41ca0ae166f08ae0e1059696c5e8161b0ab072ef7950c01d9440ff90c7ed5";
      
      const action = api.tx.price.reportPrice(price, hash);
      let rawNonce = await api.query.system.accountNonce(key.address); 
      nonce = new BN(rawNonce.toString());
      await action.signAndSend(key, { nonce }, ({ events = [], status }) => {
          console.log("pushed price", price.toString(), status.toString(), status.toString())
          nonce = nonce.add(new BN(1));
      });

      last_reported = now;
      process.exit(0);
    }

  });
  while (true) {
    await snooze(100000)
  }
}


main().catch(console.error).finally(() => process.exit());
