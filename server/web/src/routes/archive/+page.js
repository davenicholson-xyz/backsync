export async function load({ parent }) {
  const parentData = await parent();
  let settings = parentData.settings
  return { settings }
}

