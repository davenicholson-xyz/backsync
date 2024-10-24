
export async function load({ parent, fetch, params }) {
  const parentData = await parent();
  let settings = parentData.settings

  let url = `https://wallhaven.cc/api/v1/collections/fatnic/${params.id}?apikey=XXMiYVopgEjJlslkFOWxkmbdM1k4nGEi&page=1`
  let response = await fetch(`${settings.baseURL}/wallhaven/search`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ url })
  })
  let data = await response.json()
  let pages = data.meta.last_page
  console.log(pages)
  return { collection: data.data }
}
