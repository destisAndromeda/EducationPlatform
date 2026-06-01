import { API_CONFIG } from './config.js';
import { captureSessionFromCookies } from './session.js';

async function requestJson(path, options = {}) {
  const response = await fetch(`${API_CONFIG.baseUrl}${path}`, {
    credentials: 'include',
    headers: {
      Accept: 'application/json',
      ...options.headers,
    },
    ...options,
  });

  if (!response.ok) {
    const text = await response.text();
    throw new Error(text || `HTTP ${response.status}: ${response.statusText}`);
  }

  if (response.status === 204) {
    return null;
  }

  return response.json();
}

export async function fetchSubjects() {
  return requestJson(API_CONFIG.endpoints.subjects);
}

export async function fetchQuestionsBySubject(subject) {
  const questions = await requestJson(API_CONFIG.endpoints.questionsBySubject(subject));
  const sessionId = captureSessionFromCookies();

  return { questions, sessionId };
}

export async function submitAnswers(answers) {
  return requestJson(API_CONFIG.endpoints.submit, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(answers),
  });
}
