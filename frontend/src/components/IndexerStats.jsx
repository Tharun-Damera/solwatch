import { useEffect, useRef, useState } from "react";
import { Card, CardHeader, CardBody } from "./Card";

import { BASE_URL } from "../utils/env";

export default function IndexerStats({
  address,
  indexed,
  setAccount,
  setTxnsIndexed,
  setError,
}) {
  const [state, setState] = useState("⏸️ Idle");
  const [accountFetched, setAccountFetched] = useState(false);
  const [signatureStats, setSignatureStats] = useState({ total: 0 });
  const [txnStats, setTxnStats] = useState({ total: 0 });
  const [loading, setLoading] = useState(false);

  const sseRef = useRef(null);

  async function startSSE(url) {
    if (sseRef.current) sseRef.current.close();

    setLoading(true);
    const sse = new EventSource(url);
    sseRef.current = sse;

    sse.addEventListener("indexing", () => {
      setState("▶️ Running");
      setAccountFetched(false);
    });

    sse.addEventListener("syncing", () => {
      setState("🔁 Syncing");
      setAccountFetched(false);
    });

    sse.addEventListener("account-data", (e) => {
      const data = JSON.parse(e.data);
      setAccountFetched(true);
      setAccount(data);
    });

    sse.addEventListener("signatures-fetched", (e) => {
      setSignatureStats(JSON.parse(e.data));
    });

    sse.addEventListener("transactions-fetched", (e) => {
      const data = JSON.parse(e.data);
      setTxnsIndexed(data.total);
      setTxnStats(data);
    });

    sse.addEventListener("error", (e) => {
      sse.close();
      setLoading(false);
      if (e.data) {
        setState("🟥 Error");
        setError(e.data);
      }
    });

    sse.addEventListener("close", () => {
      sse.close();
      setLoading(false);
      setState("✅ Completed");
    });
  }

  async function getIdleStats() {
    setLoading(true);
    try {
      const res = await fetch(
        `${BASE_URL}/api/accounts/${address}/indexer/stats`
      );
      const data = await res.json();
      setAccountFetched(data.account_exists);
      setSignatureStats({ total: data.signatures });
      setTxnStats({ total: data.transactions });
    } catch (err) {
      setError(err.message);
    }
    setLoading(false);
  }

  async function onRefresh() {
    startSSE(`${BASE_URL}/api/accounts/${address}/refresh/sse`);
  }

  useEffect(() => {
    if (!address) return;

    function run() {
      if (indexed) {
        getIdleStats();
      } else {
        startSSE(`${BASE_URL}/api/accounts/${address}/index/sse`);
      }
    }

    run();
    return () => {
      if (sseRef.current) sseRef.current.close();
    };
  }, [address, indexed]);

  return (
    <>
      <Card>
        <CardHeader>
          <div className="stats-header-div">
            <span>Indexer Stats</span>
            <button className="refresh" disabled={loading} onClick={onRefresh}>
              <img src="/refresh-icon.svg" />
              Refresh
            </button>
          </div>
        </CardHeader>
        <CardBody>
          <table>
            <tbody>
              <tr>
                <td>State</td>
                <td className="responsive-td">{state}</td>
              </tr>
              <tr>
                <td>Account Fetched</td>
                <td className="responsive-td">
                  {accountFetched ? "✅" : "❌"}
                </td>
              </tr>
              <tr>
                <td>Signatures</td>
                <td className="responsive-td">
                  <span>
                    {signatureStats.total}{" "}
                    {signatureStats.fetched > 0 && (
                      <span> ({signatureStats.fetched}⬆)</span>
                    )}
                  </span>
                </td>
              </tr>
              <tr>
                <td>Transactions</td>
                <td className="responsive-td">
                  <span>
                    {txnStats.total}{" "}
                    {txnStats.fetched > 0 && (
                      <span> ({txnStats.fetched}⬆)</span>
                    )}
                  </span>
                </td>
              </tr>
            </tbody>
          </table>
        </CardBody>
      </Card>
      <div className="horizontal-line"></div>
    </>
  );
}
