export const API_CONFIG = {
  baseUrl: '',
  endpoints: {
    subjects: '/api/subjects',
    questionsBySubject: (subject) => `/api/questions/${encodeURIComponent(subject)}`,
    submit: '/api/questions/submit',
  },
};

export const WEB3_CONFIG = {
  solanaCluster: 'devnet',
  programId: null,
};
