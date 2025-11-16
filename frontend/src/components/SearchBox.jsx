export default function SearchBox({ loading, address, onAddress, onSearch }) {
  function onSubmit(e) {
    e.preventDefault();
    onSearch(address);
  }
  return (
    <form className="search-box" onSubmit={onSubmit}>
      <input
        id="search-input"
        type="text"
        placeholder="Enter a Solana Wallet Address"
        value={address}
        onChange={(e) => onAddress(e.target.value.trim())}
      />
      <button type="submit" disabled={loading}>
        Search
      </button>
    </form>
  );
}
