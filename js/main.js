const QUOTE_URL = 'https://alanpi.alexwilkinson.co';

const currentQuote = '';

const getQuote = async () => {
  try {
    const response = await fetch (QUOTE_URL);
    return await response.json ();
  } catch (err) {
    return console.error (err);
  }
};
