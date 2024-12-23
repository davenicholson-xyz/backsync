export function makeSeed(length) {
  let result = '';
  const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  const charactersLength = characters.length;
  let counter = 0;
  while (counter < length) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength));
    counter += 1;
  }
  return result;
}

export function convertToBinary(obj) {
  let binary = Object.values(obj)
    .map((v) => (v ? '1' : '0'))
    .join('');

  return binary == "000" ? "100" : binary;
}

