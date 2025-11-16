const BASE_URL = import.meta.env.VITE_API_URL;
console.log(`VITE_API_URL: ${BASE_URL}`);

export async function accountIndexStatus(address) {
  let res = await fetch(`${BASE_URL}/api/accounts/${address}/status`, {
    method: "GET",
  });
  let resp = await res.json();
  console.log(resp);

  return resp;
}

export async function accountData(address) {
  let res = await fetch(`${BASE_URL}/api/accounts/${address}`, {
    method: "GET",
  });
  let resp = await res.json();
  console.log(resp);

  return resp;
}

export async function transactionHistory(address) {
  let res = await fetch(
    `${BASE_URL}/api/accounts/${address}/signatures?skip=0&limit=10`,
    {
      method: "GET",
    }
  );
  let resp = await res.json();
  console.log(resp);

  return resp;
}
