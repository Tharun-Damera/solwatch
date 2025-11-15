import { useState } from "react";

import "./App.css";
import EmptyState from "./components/EmptyState";
import Navbar from "./components/NavBar";
import SearchBox from "./components/SearchBox";
import Account from "./components/Account";

import { account_index_status, account_data } from "./api/api";

export default function App() {
  let [address, setAddress] = useState("");
  let [loading, setLoading] = useState(false);
  let [account, setAccount] = useState(null);
  let [error, setError] = useState(null);

  async function handleSearch(value) {
    setAddress(value);

    setLoading(true);
    let result = await account_index_status(value);
    if (result.indexed) {
      let acc = await account_data(value);
      setAccount(acc);
    } else {
      //
    }

    setLoading(false);
  }

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

        {address && <Account data={account} />}

        {error && <div className="error">{error}</div>}
      </main>
    </>
  );
}
