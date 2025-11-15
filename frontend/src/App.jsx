import "./App.css";
import EmptyState from "./components/EmptyState";
import Navbar from "./components/NavBar";
import SearchBox from "./components/SearchBox";

import { useState } from "react";

export default function App() {
  let [loading, setLoading] = useState(false);

  function handleSearch(value) {
    console.log(value);
    setLoading(true);
  }

  return (
    <>
      <Navbar />
      <main className="container">
        <SearchBox search={handleSearch} loading={loading} />
        <EmptyState />
      </main>
    </>
  );
}
