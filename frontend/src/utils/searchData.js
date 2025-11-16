import {
  accountData,
  accountIndexStatus,
  transactionHistory,
} from "../api/api";

export async function searchAddress(address, setAccount, setTransactions) {
  let result = await accountIndexStatus(address);
  if (result.indexed) {
    window.history.replaceState({}, "", `/?address=${address}`);

    let acc = await accountData(address);
    setAccount(acc);
    let txns = await transactionHistory(address);
    setTransactions(txns);
  } else {
    //
  }
}
