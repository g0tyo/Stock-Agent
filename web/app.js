const symbols = {
  AAPL: { company: 'Apple Inc.', price: 341.00, change: '+$4.82  +1.43%', score: '+8.4', signal: 'BUY' },
  MSFT: { company: 'Microsoft Corp.', price: 506.33, change: '+$6.24  +1.25%', score: '+3.1', signal: 'HOLD' },
  NVDA: { company: 'NVIDIA Corp.', price: 178.21, change: '+$3.98  +2.28%', score: '+6.7', signal: 'BUY' },
  GOOGL: { company: 'Alphabet Inc.', price: 251.18, change: '+$1.36  +0.54%', score: '+5.2', signal: 'HOLD' }
};

const toast = document.querySelector('#toast');
let toastTimer;
function showToast(message) {
  toast.textContent = message;
  toast.classList.add('show');
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => toast.classList.remove('show'), 2200);
}

function selectSymbol(symbol) {
  const item = symbols[symbol];
  if (!item) return;
  document.querySelector('#selectedSymbol').innerHTML = `${symbol} <span>NASDAQ</span>`;
  document.querySelector('#selectedCompany').textContent = item.company;
  document.querySelector('#quotePrice').textContent = `$${item.price.toFixed(2)}`;
  document.querySelector('#quoteChange').textContent = item.change;
  document.querySelector('#scoreValue').innerHTML = `${item.score} <small>/ 10</small>`;
  document.querySelector('#signalText').textContent = item.signal;
  document.querySelector('#signalText').classList.toggle('positive', item.signal === 'BUY');
  document.querySelectorAll('.watch-row').forEach(row => row.classList.toggle('selected', row.dataset.symbol === symbol));
  document.querySelector('#quoteTimestamp').textContent = 'Updated just now';
}

document.querySelectorAll('.watch-row').forEach(row => {
  row.addEventListener('click', () => {
    selectSymbol(row.dataset.symbol);
    showToast(`${row.dataset.symbol} selected`);
  });
});

document.querySelectorAll('.nav-item, [data-view-target]').forEach(button => {
  button.addEventListener('click', () => {
    const view = button.dataset.view || button.dataset.viewTarget;
    document.querySelectorAll('.nav-item').forEach(item => item.classList.toggle('active', item.dataset.view === view));
    document.querySelectorAll('.dashboard-view').forEach(panel => panel.classList.toggle('active-view', panel.dataset.panel === view));
    window.scrollTo({ top: 0, behavior: 'smooth' });
  });
});

document.querySelector('#refreshButton').addEventListener('click', () => {
  document.querySelector('#quoteTimestamp').textContent = 'Updated just now';
  showToast('Market snapshot refreshed');
});

document.querySelector('#analyzeButton').addEventListener('click', (event) => {
  const button = event.currentTarget;
  button.disabled = true;
  button.innerHTML = 'Analyzing… <span>↗</span>';
  setTimeout(() => {
    button.disabled = false;
    button.innerHTML = 'Run analysis <span>→</span>';
    showToast('Analysis refreshed');
  }, 700);
});
