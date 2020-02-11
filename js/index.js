const QUOTE_URL = 'https://alanpi.alexwilkinson.co';

let currentQuote = '';

const getQuote = async () => {
  try {
    const response = await fetch (QUOTE_URL);
    return await response.json ();
  } catch (err) {
    return console.error (err);
  }
};

const setQuote = async () => {
  const {quote} = await getQuote ();
  currentQuote = quote;
  document.querySelector ('#quote-text').innerText = quote;
};

(async () => {
  try {
    await setQuote ();
  } catch (err) {
    console.error (err);
  }
}) ();
