import { UI } from './i18n/cs.js';

function escapeHtml(value) {
  return String(value)
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
}

export function renderSubjects(selectEl, subjects, selectedValue) {
  selectEl.innerHTML = subjects
    .map((subject) => `<option value="${escapeHtml(subject)}">${escapeHtml(subject)}</option>`)
    .join('');

  if (selectedValue) {
    selectEl.value = selectedValue;
  }
}

export function renderQuestions(container, template, questions) {
  container.innerHTML = '';

  const fragment = document.createDocumentFragment();

  questions.forEach((question, index) => {
    const node = template.content.cloneNode(true);
    const card = node.querySelector('.question-card');
    card.dataset.questionIndex = String(index);
    if (question.id !== undefined && question.id !== null) {
      card.dataset.questionId = String(question.id);
    }

    node.querySelector('[data-role="subject"]').textContent = question.subject || UI.noSubject;
    node.querySelector('[data-role="section"]').textContent = question.section || UI.noSection;
    node.querySelector('[data-role="question"]').textContent = question.question || '';

    const optionsList = node.querySelector('[data-role="options"]');
    const options = question.options ?? {};

    Object.entries(options).forEach(([key, value]) => {
      const item = document.createElement('li');
      item.className = 'option-row';

      const label = document.createElement('label');
      label.className = 'option-label';

      const input = document.createElement('input');
      input.type = 'checkbox';
      input.className = 'option-input';
      input.dataset.role = 'option-input';
      input.value = key;
      input.name = `question-${index}`;

      const text = document.createElement('span');
      text.className = 'option-text';
      text.innerHTML = `<strong>${escapeHtml(key)})</strong> ${escapeHtml(value)}`;

      label.append(input, text);
      item.append(label);
      optionsList.append(item);
    });

    fragment.appendChild(node);
  });

  container.appendChild(fragment);
}

export function collectAnswersFromDom(container, questions) {
  const cards = container.querySelectorAll('.question-card');

  return Array.from(cards).map((card) => {
    const index = Number(card.dataset.questionIndex);
    const question = questions[index];

    const selected = [...card.querySelectorAll('[data-role="option-input"]:checked')]
      .map((input) => input.value)
      .sort();

    return {
      id: question.id,
      subject: question.subject,
      section: question.section,
      question: question.question,
      options: question.options ?? {},
      answer: selected,
    };
  });
}

export function setStatus(statusEl, text, isError = false) {
  statusEl.textContent = text;
  statusEl.style.color = isError ? '#ff7e93' : '';
}

export function applyAssessment(container, assessments) {
  const map = new Map((assessments || []).map((a) => [String(a.id), a.flag]));

  const cards = container.querySelectorAll('.question-card');
  let greenCount = 0;
  cards.forEach((card) => {
    const id = card.dataset.questionId;
    card.classList.remove('flag-green', 'flag-yellow', 'flag-red');

    if (!id) return;
    const flag = map.get(id);
    if (!flag) return;

    if (flag === 'Green') {
      card.classList.add('flag-green');
      greenCount += 1;
    } else if (flag === 'Yellow') card.classList.add('flag-yellow');
    else if (flag === 'Red') card.classList.add('flag-red');
  });

  const successEl = document.getElementById('success-counter');
  if (successEl) {
    // display as "<greenCount> / 30"
    successEl.textContent = `${greenCount} / 30`;
  }
}
