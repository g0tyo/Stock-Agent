const symbolCatalog = {
  AAPL: { company: 'Apple Inc.', price: 341.00, change: '+$4.82  +1.43%', score: 8.4, signal: 'BUY', confidence: 82, reward: '1 : 2.6', trend: 'Constructive trend', rsi: 64.8, macd: 2.14, volume: 1.42, volatility: 18.6, color: 'lime' },
  MSFT: { company: 'Microsoft Corp.', price: 506.33, change: '+$6.24  +1.25%', score: 3.1, signal: 'HOLD', confidence: 54, reward: '1 : 1.4', trend: 'Mixed momentum', rsi: 52.6, macd: 0.48, volume: 0.94, volatility: 15.2, color: 'blue' },
  NVDA: { company: 'NVIDIA Corp.', price: 178.21, change: '+$3.98  +2.28%', score: 6.7, signal: 'BUY', confidence: 74, reward: '1 : 2.1', trend: 'Momentum returning', rsi: 61.2, macd: 1.72, volume: 1.68, volatility: 24.1, color: 'orange' },
  GOOGL: { company: 'Alphabet Inc.', price: 251.18, change: '+$1.36  +0.54%', score: 5.2, signal: 'HOLD', confidence: 63, reward: '1 : 1.8', trend: 'Range-bound', rsi: 57.4, macd: 0.91, volume: 1.06, volatility: 17.1, color: 'red' }
};

const positions = [
  { symbol: 'AAPL', quantity: 120, average: 328.40, risk: 'Low' },
  { symbol: 'MSFT', quantity: 34, average: 498.10, risk: 'Medium' },
  { symbol: 'NVDA', quantity: 58, average: 169.72, risk: 'Medium' }
];

const decisions = [
  { time: '09:42:18', symbol: 'AAPL', signal: 'BUY', score: 8.4, confidence: 82, reason: 'Trend and momentum aligned', executed: true },
  { time: '09:36:02', symbol: 'MSFT', signal: 'HOLD', score: 3.1, confidence: 54, reason: 'Score below entry threshold', executed: false },
  { time: '09:30:45', symbol: 'NVDA', signal: 'BUY', score: 6.7, confidence: 74, reason: 'MACD crossover with strong volume', executed: true },
  { time: '09:18:11', symbol: 'GOOGL', signal: 'REJECTED', score: 5.2, confidence: 63, reason: 'Exposure limit would be exceeded', executed: false }
];

const state = {
  selectedSymbol: localStorage.getItem('stock-agent-symbol') || 'AAPL',
  watchlist: JSON.parse(localStorage.getItem('stock-agent-watchlist') || '["AAPL","MSFT","NVDA","GOOGL"]'),
  decisionFilter: 'all'
};

const toast = document.querySelector('#toast');
const dialog = document.querySelector('#symbolDialog');
const symbolInput = document.querySelector('#symbolInput');
let toastTimer;

function showToast(message) {
  toast.textContent = message;
  toast.classList.add('show');
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => toast.classList.remove('show'), 2400);
}

function formatMoney(value) {
  return `$${value.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
}

function escapeHtml(value) {
  return value.replace(/[&<>'"]/g, character => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', "'": '&#39;', '"': '&quot;' })[character]);
}

function getSymbolData(symbol) {
  if (symbolCatalog[symbol]) return symbolCatalog[symbol];
  const seed = symbol.split('').reduce((total, character) => total + character.charCodeAt(0), 0);
  return { company: `${symbol} Holdings`, price: 80 + (seed % 280), change: '+$1.24  +0.82%', score: 4.8, signal: 'HOLD', confidence: 58, reward: '1 : 1.6', trend: 'Early observation', rsi: 50 + (seed % 14), macd: 0.3, volume: 1.03, volatility: 16.4, color: 'blue' };
}

function renderWatchlist() {
  document.querySelector('#watchlistRows').innerHTML = state.watchlist.map(symbol => {
    const data = getSymbolData(symbol);
    return `<button class="watch-row ${symbol === state.selectedSymbol ? 'selected' : ''}" data-symbol="${escapeHtml(symbol)}"><span class="ticker-dot ${data.color}"></span><span><strong>${escapeHtml(symbol)}</strong><small>${escapeHtml(data.company)}</small></span><b>${formatMoney(data.price)}</b></button>`;
  }).join('');
  document.querySelectorAll('.watch-row').forEach(row => row.addEventListener('click', () => selectSymbol(row.dataset.symbol)));
}

function chartPath(data, range = '1D') {
  const points = Array.from({ length: 18 }, (_, index) => {
    const wave = Math.sin((index + data.price) * 0.7) * 12;
    const drift = index * (range === '1D' ? 5.3 : range === '1W' ? 3.9 : 2.4);
    return 174 - drift - wave;
  });
  const line = points.map((point, index) => `${index ? 'L' : 'M'} ${(index / (points.length - 1)) * 760} ${point}`).join(' ');
  return { line, area: `${line} V220 H0 Z`, last: points[points.length - 1] };
}

function renderChart(range = '1D') {
  const chart = chartPath(getSymbolData(state.selectedSymbol), range);
  document.querySelector('#chartLine').setAttribute('d', chart.line);
  document.querySelector('#chartArea').setAttribute('d', chart.area);
  document.querySelector('#chartPoint').setAttribute('cy', chart.last);
}

function renderQuote() {
  const data = getSymbolData(state.selectedSymbol);
  document.querySelector('#selectedSymbol').innerHTML = `${escapeHtml(state.selectedSymbol)} <span>NASDAQ</span>`;
  document.querySelector('#selectedCompany').textContent = data.company;
  document.querySelector('#quotePrice').textContent = formatMoney(data.price);
  document.querySelector('#quoteChange').textContent = data.change;
  document.querySelector('#signalText').textContent = data.signal;
  document.querySelector('#signalSummary').textContent = data.trend;
  document.querySelector('#signalOrb').textContent = data.signal;
  document.querySelector('#signalStatus').textContent = data.confidence >= 70 ? 'HIGH CONVICTION' : 'WATCH CLOSELY';
  document.querySelector('#scoreValue').innerHTML = `${data.score > 0 ? '+' : ''}${data.score.toFixed(1)} <small>/ 10</small>`;
  document.querySelector('#confidenceValue').textContent = `${data.confidence}%`;
  document.querySelector('#rewardValue').textContent = data.reward;
  const scorePosition = Math.max(8, Math.min(96, 50 + data.score * 5));
  document.querySelector('#scoreFill').style.width = `${scorePosition}%`;
  document.querySelector('#scoreMarker').style.left = `${scorePosition}%`;
  document.querySelector('#signalOrb').classList.toggle('hold', data.signal === 'HOLD');
  document.querySelector('#signalText').classList.toggle('hold', data.signal === 'HOLD');
  renderChart();
}

function renderMetrics() {
  const data = getSymbolData(state.selectedSymbol);
  const metrics = [
    ['RSI (14)', data.rsi.toFixed(1), `${data.rsi > 60 ? 'Bullish' : 'Balanced'} range`, 'lime-bg', `${Math.round(data.rsi)}%`],
    ['MACD', `${data.macd > 0 ? '+' : ''}${data.macd.toFixed(2)}`, data.macd > 0 ? 'Positive crossover' : 'Needs confirmation', 'blue-bg', `${Math.min(100, 50 + data.macd * 14)}%`],
    ['Volume ratio', `${data.volume.toFixed(2)}x`, data.volume > 1.2 ? 'Above average' : 'Normal participation', 'orange-bg', `${Math.min(100, data.volume * 48)}%`],
    ['Volatility', `${data.volatility.toFixed(1)}%`, data.volatility > 22 ? 'Elevated risk' : 'Moderate risk', 'red-bg', `${Math.min(100, data.volatility * 2)}%`]
  ];
  document.querySelector('#metricsGrid').innerHTML = metrics.map(([label, value, note, color, width], index) => `<article class="metric-card"><div class="metric-label"><span>${label}</span><span class="metric-icon ${color}">${['↗', '⌁', '▥', '∿'][index]}</span></div><strong>${value}</strong><div class="metric-bar ${index === 3 ? 'red-bar' : ''}"><span style="width:${width}"></span></div><small class="${index < 3 ? 'positive' : ''}">${note}</small></article>`).join('');
}

function renderOverviewLists() {
  document.querySelector('#decisionPreview').innerHTML = decisions.slice(0, 3).map(decision => `<div class="decision-row"><span class="decision-symbol ${getSymbolData(decision.symbol).color}">${decision.symbol}</span><div><strong>${decision.signal === 'REJECTED' ? 'Risk check blocked' : `${decision.signal} signal generated`}</strong><small>${decision.reason}</small></div><span class="decision-score ${decision.signal === 'HOLD' ? 'neutral' : 'positive'}">${decision.score > 0 ? '+' : ''}${decision.score.toFixed(1)}</span><time>${decision.time}</time></div>`).join('');
  document.querySelector('#allocationLegend').innerHTML = [['lime', 'Equities', '$68,420'], ['blue', 'Cash', '$31,580'], ['gray', 'Reserved risk', '$1,000']].map(([color, label, value]) => `<div><span class="legend-dot ${color}"></span><span>${label}</span><strong>${value}</strong></div>`).join('');
}

function renderResearch() {
  const data = getSymbolData(state.selectedSymbol);
  document.querySelector('#researchTicker').textContent = state.selectedSymbol;
  document.querySelector('#researchTitle').textContent = `${state.selectedSymbol} is showing ${data.trend.toLowerCase()}`;
  document.querySelector('#researchDescription').textContent = data.signal === 'BUY' ? 'Price, momentum, and participation are aligned enough to keep the setup on the active watchlist.' : 'The evidence is mixed. The agent is keeping this symbol visible while it waits for a cleaner setup.';
  document.querySelector('#thesisAction').textContent = data.signal === 'BUY' ? 'Protect the entry' : 'Wait for confirmation';
  const evidence = [['Signal', data.signal, data.signal === 'BUY' ? 'positive' : 'neutral'], ['Confidence', `${data.confidence}%`, data.confidence > 70 ? 'positive' : 'neutral'], ['Risk / reward', data.reward, 'positive'], ['Volatility', `${data.volatility.toFixed(1)}%`, data.volatility > 22 ? 'warning' : 'neutral']];
  document.querySelector('#evidenceList').innerHTML = evidence.map(([label, value, tone]) => `<div class="evidence-row"><span>${label}</span><strong class="${tone}">${value}</strong><i></i></div>`).join('');
}

function renderPortfolio() {
  const totalValue = positions.reduce((total, position) => total + position.quantity * getSymbolData(position.symbol).price, 0);
  const totalCost = positions.reduce((total, position) => total + position.quantity * position.average, 0);
  const profit = totalValue - totalCost;
  document.querySelector('#portfolioStats').innerHTML = [['Total equity', formatMoney(100000 + profit), '＋2.84% today'], ['Deployed', formatMoney(totalValue), '68.4% of equity'], ['Unrealized P/L', `+${formatMoney(profit).slice(1)}`, '+3.84% blended'], ['Available cash', formatMoney(100000 - totalCost), 'Ready for setups']].map(([label, value, note]) => `<div class="stat-card"><small>${label}</small><strong>${value}</strong><span>${note}</span></div>`).join('');
  document.querySelector('#positionsTable').innerHTML = positions.map(position => {
    const data = getSymbolData(position.symbol);
    const value = data.price * position.quantity;
    const pnl = value - position.average * position.quantity;
    return `<tr><td><strong>${position.symbol}</strong><small>${data.company}</small></td><td>${position.quantity}</td><td>${formatMoney(position.average)}</td><td>${formatMoney(data.price)}</td><td>${formatMoney(value)}</td><td class="positive">+$${pnl.toLocaleString('en-US', { maximumFractionDigits: 0 })} +${((pnl / (position.average * position.quantity)) * 100).toFixed(2)}%</td><td><span class="risk-pill ${position.risk.toLowerCase()}">${position.risk}</span></td></tr>`;
  }).join('');
}

function renderDecisions() {
  const filtered = decisions.filter(decision => state.decisionFilter === 'all' || decision.signal === state.decisionFilter);
  document.querySelector('#decisionCount').textContent = `${filtered.length} decision${filtered.length === 1 ? '' : 's'}`;
  document.querySelector('#decisionsTable').innerHTML = filtered.map(decision => `<tr><td>${decision.time}</td><td><strong>${decision.symbol}</strong></td><td><span class="signal-pill ${decision.signal.toLowerCase()}">${decision.signal}</span></td><td class="${decision.score > 5 ? 'positive' : ''}">${decision.score > 0 ? '+' : ''}${decision.score.toFixed(1)}</td><td>${decision.confidence}%</td><td>${decision.reason}</td><td>${decision.executed ? '<span class="yes-dot">●</span> Yes' : 'No'}</td></tr>`).join('');
}

function renderRisk() {
  const risks = [['Daily loss limit', '0.42%', '10.00%', '4.2%', 'lime'], ['Portfolio exposure', '68.42%', '80.00%', '85%', 'blue'], ['Consecutive losses', '0', '2', '1%', 'orange'], ['Maximum drawdown', '3.18%', '20.00%', '16%', 'red']];
  document.querySelector('#riskGrid').innerHTML = risks.map(([label, current, maximum, width, color]) => `<article class="risk-card"><span class="metric-icon ${color}-bg">${color === 'lime' ? '✓' : color === 'red' ? '⌁' : '◌'}</span><div><small>${label}</small><strong>${current} <em>/ ${maximum}</em></strong><div class="risk-progress ${color}-progress"><span style="width:${width}"></span></div></div></article>`).join('');
}

function selectSymbol(symbol) {
  state.selectedSymbol = symbol.toUpperCase();
  if (!state.watchlist.includes(state.selectedSymbol)) state.watchlist.push(state.selectedSymbol);
  localStorage.setItem('stock-agent-symbol', state.selectedSymbol);
  localStorage.setItem('stock-agent-watchlist', JSON.stringify(state.watchlist));
  renderWatchlist(); renderQuote(); renderMetrics(); renderResearch(); renderPortfolio();
  showToast(`${state.selectedSymbol} selected`);
}

function showView(view) {
  document.querySelectorAll('.nav-item').forEach(item => item.classList.toggle('active', item.dataset.view === view));
  document.querySelectorAll('.dashboard-view').forEach(panel => panel.classList.toggle('active-view', panel.dataset.panel === view));
  document.querySelector('#pageTitle').textContent = { overview: 'Good morning, operator.', research: 'Research before action.', positions: 'Know where capital is.', decisions: 'Every signal leaves a trace.', risk: 'Guardrails stay in charge.', model: 'Teach the agent with evidence.' }[view];
  window.location.hash = view;
  window.scrollTo({ top: 0, behavior: 'smooth' });
}

document.querySelectorAll('.nav-item, [data-view-target]').forEach(button => button.addEventListener('click', () => showView(button.dataset.view || button.dataset.viewTarget)));
document.querySelectorAll('.timeframe-tabs button').forEach(button => button.addEventListener('click', () => { document.querySelectorAll('.timeframe-tabs button').forEach(tab => tab.classList.remove('active')); button.classList.add('active'); renderChart(button.dataset.range); }));
document.querySelectorAll('.filter-tabs button').forEach(button => button.addEventListener('click', () => { document.querySelectorAll('.filter-tabs button').forEach(tab => tab.classList.remove('active')); button.classList.add('active'); state.decisionFilter = button.dataset.filter; renderDecisions(); }));
document.querySelector('#refreshButton').addEventListener('click', () => { document.querySelector('#quoteTimestamp').textContent = 'Updated just now'; renderQuote(); renderMetrics(); renderPortfolio(); showToast('Market snapshot refreshed'); });
document.querySelector('#analyzeButton').addEventListener('click', event => { const button = event.currentTarget; button.disabled = true; button.innerHTML = 'Analyzing… <span>↗</span>'; setTimeout(() => { button.disabled = false; button.innerHTML = 'Run analysis <span>→</span>'; renderQuote(); renderResearch(); showToast(`Analysis refreshed for ${state.selectedSymbol}`); }, 650); });
document.querySelector('#addSymbolButton').addEventListener('click', () => { dialog.showModal(); symbolInput.focus(); });
document.querySelector('#confirmSymbolButton').addEventListener('click', event => { const symbol = symbolInput.value.trim().toUpperCase(); if (!/^[A-Z]{1,5}$/.test(symbol)) { event.preventDefault(); showToast('Enter a valid ticker symbol'); return; } selectSymbol(symbol); symbolInput.value = ''; });
document.querySelector('#searchInput').addEventListener('keydown', event => { if (event.key === 'Enter' && event.currentTarget.value.trim()) { selectSymbol(event.currentTarget.value.trim()); event.currentTarget.value = ''; showView('research'); } });
document.querySelector('#saveResearchButton').addEventListener('click', () => showToast(`${state.selectedSymbol} thesis saved locally`));
document.querySelector('#featureButton').addEventListener('click', () => showToast('Feature review: 6 indicators are active'));
document.querySelector('#shadowButton').addEventListener('click', event => { event.currentTarget.textContent = 'Shadow plan active'; event.currentTarget.disabled = true; showToast('30-session shadow plan started'); });
document.querySelector('#exportButton').addEventListener('click', () => { const rows = [['Symbol', 'Quantity', 'Average cost', 'Last', 'Market value'], ...positions.map(position => { const data = getSymbolData(position.symbol); return [position.symbol, position.quantity, position.average, data.price, data.price * position.quantity]; })]; const blob = new Blob([rows.map(row => row.join(',')).join('\n')], { type: 'text/csv' }); const link = document.createElement('a'); link.href = URL.createObjectURL(blob); link.download = 'stock-agent-positions.csv'; link.click(); URL.revokeObjectURL(link.href); showToast('Portfolio CSV exported'); });

renderWatchlist(); renderQuote(); renderMetrics(); renderOverviewLists(); renderResearch(); renderPortfolio(); renderDecisions(); renderRisk();
const initialView = window.location.hash.slice(1); if (document.querySelector(`[data-panel="${initialView}"]`)) showView(initialView);
