const BASE_URL = import.meta.env.VITE_API_URL;
console.log(`VITE_API_URL: ${BASE_URL}`);


export async function account_index_status(address) {
  let res = await fetch(`${BASE_URL}/api/accounts/${address}/status`, {
    method: "GET",
  });
  let resp = await res.json();
  console.log(resp);

  return resp;
}

export async function account_data(address) {
  let res = await fetch(`${BASE_URL}/api/accounts/${address}`, {
    method: "GET",
  });
  let resp = await res.json();
  console.log(resp);

  return resp;
}
