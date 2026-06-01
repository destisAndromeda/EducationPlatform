/** Czech UI copy — user-facing strings only. */
export const UI = {
  noSubject: 'Bez předmětu',
  noSection: 'Bez oddílu',

  loadingSubjects: 'Načítám předměty...',
  subjectsNotFound: 'Předměty nenalezeny. Zkontroluj databázi a API.',
  subjectsFound: (count) => `Nalezeno předmětů: ${count}`,
  selectSubjectFirst: 'Nejdřív vyber předmět.',
  generatingVariant: 'Generuji variantu...',
  variantReady: (count, sessionId) => {
    const base = `Varianta připravena. Načteno otázek: ${count}.`;
    if (!sessionId) return base;
    return `${base} ID relace: ${sessionId.slice(0, 8)}...`;
  },

  noQuestionsToSubmit: 'Nejsou žádné otázky k odeslání.',
  unansweredWarning: (count) => `Upozornění: ${count} otázek bez odpovědi. Odesílám...`,
  sessionCookieMissing: 'Session cookie nebyla zachycena, odesílám přes credentials prohlížeče...',
  submittingAnswers: 'Odesílám odpovědi na server...',
  answersSubmitted: (count) => `Odpovědi odeslány (${count} záznamů).`,
  submitFailed: (message) => `Chyba odeslání: ${message}`,

  walletConnect: 'Připojit peněženku',
  walletDisconnected: 'Peněženka odpojena.',
  walletConnected: (address) => `Peněženka: ${address}`,
  walletReady: 'Peněženka připojena. Lze přidat web3 akce.',
  walletConnectFailed: 'Nepodařilo se připojit peněženku.',
  walletNotFound: 'Phantom peněženka nenalezena. Nainstaluj rozšíření nebo připoj jiný adaptér.',
  walletUnknownAddress: 'neznámá',

  loadSubjectsFailed: (message) => `Chyba načítání předmětů: ${message}`,
  generateFailed: (message) => `Chyba generování: ${message}`,
  startupFailed: (message) => `Chyba startu: ${message}`,
  startupGenerateFailed: (message) => `Chyba generování při startu: ${message}`,
};
