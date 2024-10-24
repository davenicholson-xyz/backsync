
export async function load({ parent, params }) {
  const parentData = await parent();
  let settings = parentData.settings
  return { settings, id: params.id }
}
