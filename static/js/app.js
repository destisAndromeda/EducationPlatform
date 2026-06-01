import { fetchQuestionsBySubject, fetchSubjects, submitAnswers } from './api.js';
import { UI } from './i18n/cs.js';
import { appState, setTheme } from './state.js';
import {
  collectAnswersFromDom,
  renderQuestions,
  renderSubjects,
  setStatus,
  applyAssessment,
} from './ui.js';
import { connectSolanaWallet, disconnectSolanaWallet } from './wallet/solanaWallet.js';

const ui = {
  subjectSelect: document.getElementById('subject-select'),
  countInput: document.getElementById('questions-count'),
  generateBtn: document.getElementById('generate-btn'),
  refreshBtn: document.getElementById('refresh-btn'),
  submitBtn: document.getElementById('submit-btn'),
  walletBtn: document.getElementById('wallet-connect-btn'),
  themeBtn: document.getElementById('theme-toggle-btn'),
  statusLine: document.getElementById('status-line'),
  questionsList: document.getElementById('questions-list'),
  questionsCounter: document.getElementById('questions-counter'),
  questionTemplate: document.getElementById('question-card-template'),
};

function applyTheme(theme) {
  document.body.classList.toggle('light', theme === 'light');
}

function updateCounter() {
  ui.questionsCounter.textContent = String(appState.questions.length);
  ui.submitBtn.disabled = appState.questions.length === 0;
}

async function loadSubjects() {
  setStatus(ui.statusLine, UI.loadingSubjects);

  const subjects = await fetchSubjects();
  appState.subjects = subjects;
  appState.selectedSubject = subjects[0] ?? null;

  renderSubjects(ui.subjectSelect, subjects, appState.selectedSubject);

  if (!subjects.length) {
    setStatus(ui.statusLine, UI.subjectsNotFound, true);
    return;
  }

  setStatus(ui.statusLine, UI.subjectsFound(subjects.length));
}

async function generateTest() {
  if (!appState.selectedSubject) {
    setStatus(ui.statusLine, UI.selectSubjectFirst, true);
    return;
  }

  setStatus(ui.statusLine, UI.generatingVariant);
  const { questions: allQuestions, sessionId } = await fetchQuestionsBySubject(appState.selectedSubject);
  appState.sessionId = sessionId;

  const limit = Number(ui.countInput.value);
  appState.questions = Number.isFinite(limit) && limit > 0
    ? allQuestions.slice(0, limit)
    : allQuestions;

  renderQuestions(ui.questionsList, ui.questionTemplate, appState.questions);
  updateCounter();
  setStatus(ui.statusLine, UI.variantReady(appState.questions.length, sessionId));
}

async function sendAnswers() {
  if (!appState.questions.length) {
    setStatus(ui.statusLine, UI.noQuestionsToSubmit, true);
    return;
  }

  const payload = collectAnswersFromDom(ui.questionsList, appState.questions);
  const unanswered = payload.filter((item) => item.answer.length === 0).length;

  if (unanswered > 0) {
    setStatus(ui.statusLine, UI.unansweredWarning(unanswered));
  } else if (!appState.sessionId) {
    setStatus(ui.statusLine, UI.sessionCookieMissing);
  } else {
    setStatus(ui.statusLine, UI.submittingAnswers);
  }

  ui.submitBtn.disabled = true;

  try {
    const result = await submitAnswers(payload);
    // If server returned an array of assessments, apply visual flags
    if (Array.isArray(result)) {
      applyAssessment(ui.questionsList, result);
      const count = result.length;
      setStatus(ui.statusLine, UI.answersSubmitted(count));
    } else {
      const count = result?.count ?? payload.length;
      setStatus(ui.statusLine, UI.answersSubmitted(count));
    }
  } catch (error) {
    setStatus(ui.statusLine, UI.submitFailed(error.message), true);
  } finally {
    ui.submitBtn.disabled = appState.questions.length === 0;
  }
}

async function toggleWalletConnection() {
  try {
    if (appState.wallet.connected) {
      await disconnectSolanaWallet(appState.wallet.provider);
      appState.wallet.connected = false;
      appState.wallet.address = null;
      appState.wallet.provider = null;
      ui.walletBtn.textContent = UI.walletConnect;
      setStatus(ui.statusLine, UI.walletDisconnected);
      return;
    }

    const connected = await connectSolanaWallet();
    appState.wallet.connected = true;
    appState.wallet.address = connected.address;
    appState.wallet.provider = connected.provider;

    const shortAddress = connected.address
      ? `${connected.address.slice(0, 4)}...${connected.address.slice(-4)}`
      : UI.walletUnknownAddress;

    ui.walletBtn.textContent = UI.walletConnected(shortAddress);
    setStatus(ui.statusLine, UI.walletReady);
  } catch (error) {
    setStatus(ui.statusLine, error.message || UI.walletConnectFailed, true);
  }
}

function setupEvents() {
  ui.subjectSelect.addEventListener('change', (event) => {
    appState.selectedSubject = event.target.value;
  });

  ui.refreshBtn.addEventListener('click', async () => {
    try {
      await loadSubjects();
    } catch (error) {
      setStatus(ui.statusLine, UI.loadSubjectsFailed(error.message), true);
    }
  });

  ui.generateBtn.addEventListener('click', async () => {
    try {
      await generateTest();
    } catch (error) {
      setStatus(ui.statusLine, UI.generateFailed(error.message), true);
    }
  });

  ui.submitBtn.addEventListener('click', sendAnswers);

  ui.walletBtn.addEventListener('click', toggleWalletConnection);

  ui.themeBtn.addEventListener('click', () => {
    const nextTheme = appState.theme === 'dark' ? 'light' : 'dark';
    setTheme(nextTheme);
    applyTheme(nextTheme);
  });
}

async function bootstrap() {
  applyTheme(appState.theme);
  setupEvents();

  try {
    await loadSubjects();
  } catch (error) {
    setStatus(ui.statusLine, UI.startupFailed(error.message), true);
    return;
  }

  try {
    await generateTest();
  } catch (error) {
    setStatus(ui.statusLine, UI.startupGenerateFailed(error.message), true);
  }
}

bootstrap();
