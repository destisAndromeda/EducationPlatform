const SESSION_STORAGE_KEY = 'edu.sessionId';

const UUID_RE = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

export function getStoredSessionId() {
  return sessionStorage.getItem(SESSION_STORAGE_KEY);
}

export function setStoredSessionId(sessionId) {
  if (sessionId) {
    sessionStorage.setItem(SESSION_STORAGE_KEY, sessionId);
  } else {
    sessionStorage.removeItem(SESSION_STORAGE_KEY);
  }
}

export function captureSessionFromCookies() {
  const parts = document.cookie.split(';').map((part) => part.trim()).filter(Boolean);

  for (const part of parts) {
    const eqIndex = part.indexOf('=');

    if (eqIndex === -1) {
      if (UUID_RE.test(part)) {
        setStoredSessionId(part);
        return part;
      }
      continue;
    }

    const name = part.slice(0, eqIndex).trim();
    const value = part.slice(eqIndex + 1).trim();

    if (UUID_RE.test(name)) {
      setStoredSessionId(name);
      return name;
    }

    if (UUID_RE.test(value)) {
      setStoredSessionId(value);
      return value;
    }
  }

  return getStoredSessionId();
}
