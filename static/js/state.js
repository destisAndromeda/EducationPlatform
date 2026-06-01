export const appState = {
  subjects: [],
  selectedSubject: null,
  questions: [],
  sessionId: sessionStorage.getItem('edu.sessionId'),
  theme: localStorage.getItem('edu.theme') || 'dark',
  wallet: {
    connected: false,
    address: null,
    provider: null,
  },
};

export function setTheme(theme) {
  appState.theme = theme;
  localStorage.setItem('edu.theme', theme);
}
