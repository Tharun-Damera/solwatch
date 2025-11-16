import { useEffect, useState } from "react";

import "./App.css";
import EmptyState from "./components/EmptyState";
import Navbar from "./components/NavBar";
import SearchBox from "./components/SearchBox";
import Account from "./components/Account";

import TransactionHistory from "./components/TransactionHistory";
import { searchAddress } from "./utils/searchData";

export default function App() {
  let [address, setAddress] = useState("");
  let [loading, setLoading] = useState(false);
  let [account, setAccount] = useState(null);
  let [transactions, setTransactions] = useState(null);
  let [error, setError] = useState(null);

  async function handleSearch(value) {
    setAddress(value);
    setLoading(true);
    await searchAddress(value, setAccount, setTransactions);
    setLoading(false);
  }

  useEffect(() => {
    const params = new URLSearchParams(window.location.search);
    const urlAddress = params.get("address");
    if (urlAddress) {
      searchAddress(urlAddress, setAccount, setTransactions);
    }
  }, []);

  return (
    <>
      <Navbar />
      <main className="container">
        <SearchBox
          loading={loading}
          address={address}
          onAddress={setAddress}
          onSearch={handleSearch}
        />
        <div class="horizontal-line"></div>

        {!account && <EmptyState />}

        {account && <Account data={account} />}

        {account && <div class="horizontal-line"></div>}

        {transactions && <TransactionHistory transactions={transactions} />}

        {error && <div className="error">{error}</div>}
      </main>
    </>
  );
}
