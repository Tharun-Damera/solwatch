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
  let [error, setError] = useState(null);

  async function handleSearch(value) {
    setAddress(value);
    setLoading(true);
    await searchAddress(value, setAccount);
    setLoading(false);
  }

  useEffect(() => {
    const params = new URLSearchParams(window.location.search);
    const urlAddress = params.get("address");
    if (urlAddress) {
      setAddress(urlAddress);
      searchAddress(urlAddress, setAccount);
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
        {error && <div className="error">{error}</div>}
        <div class="horizontal-line"></div>

        {!account && <EmptyState />}

        {account && <Account data={account} />}

        {account && <div class="horizontal-line"></div>}

        {account && <TransactionHistory address={address} />}
      </main>
    </>
  );
}
