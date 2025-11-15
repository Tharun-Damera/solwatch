export default function SearchBox({ loading, address, onAddress, onSearch }) {
  return (
    <div className="search-box">
      <input
        id="search-input"
        type="text"
        placeholder="Enter a Solana Wallet Address"
        value={address}
        onChange={(e) => onAddress(e.target.value.trim())}
      />
      <button onClick={() => onSearch(address)} disabled={loading}>
        Search
      </button>
    </div>
  );
}
