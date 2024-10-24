
export async function load({ parent, fetch }) {
  const parentData = await parent();
  let settings = parentData.settings
  let url = `https://wallhaven.cc/api/v1/collections?apikey=XXMiYVopgEjJlslkFOWxkmbdM1k4nGEi`
  let response = await fetch(`${settings.baseURL}/wallhaven/search`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ url })
  })
  let data = await response.json()
  return { collections: data.data }
}

