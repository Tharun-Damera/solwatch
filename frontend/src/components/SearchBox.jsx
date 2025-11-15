import { useState } from "react";

export default function SearchBox({ onSearch, loading }) {
  let [value, setValue] = useState("");

  function onSubmit(e) {
    e.preventDefault();
    if (!value) return;
    onSearch(value.trim());
  }
  return (
    <form className="search-box">
      <input
        type="text"
        placeholder="Enter a Solana Wallet Address"
        value={value}
        onChange={(e) => setValue(e.target.value)}
      />
      <button type="submit" onSubmit={onSubmit}>
        {loading ? "Searching" : "Search"}
      </button>
    </form>
  );
}
