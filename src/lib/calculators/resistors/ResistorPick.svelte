<script>
  import Input from '../../components/Input.svelte';
  import Select from '../../components/Select.svelte';
  import Button from '../../components/Button.svelte';
  import { solveResistorPower, formatVoltage, formatPower } from '../../utils/formulas.js';
  import { formatOhm, formatOhmCompact } from '../../utils/resistorCodes.js';
  import { nearestE24 } from '../../utils/formulas.js';

  let uStr = $state(''), iStr = $state(''), rStr = $state('');
  let uUnit = $state('v'), iUnit = $state('a'), rUnit = $state('ohm');

  const voltageUnits = [{ value: 'v', label: 'В' }, { value: 'mv', label: 'мВ' }, { value: 'kv', label: 'кВ' }];
  const currentUnits = [{ value: 'a', label: 'А' }, { value: 'ma', label: 'мА' }, { value: 'ua', label: 'мкА' }];
  const ohmUnits = [{ value: 'ohm', label: 'Ом' }, { value: 'kohm', label: 'кОм' }, { value: 'mohm', label: 'МОм' }];
  const uFactor = { v: 1, mv: 1e-3, kv: 1e3 };
  const iFactor = { a: 1, ma: 1e-3, ua: 1e-6 };
  const rFactor = { ohm: 1, kohm: 1e3, mohm: 1e6 };

  function parseField(s) { if (s == null) return null; const t = String(s).trim().replace(',', '.'); if (!t) return null; const n = parseFloat(t); return isNaN(n) ? null : n; }

  const u = $derived(parseField(uStr) != null ? parseField(uStr) * uFactor[uUnit] : null);
  const i = $derived(parseField(iStr) != null ? parseField(iStr) * iFactor[iUnit] : null);
  const r = $derived(parseField(rStr) != null ? parseField(rStr) * rFactor[rUnit] : null);

  const filledCount = $derived([u, i, r].filter((v) => v !== null).length);
  const result = $derived(filledCount === 2 ? solveResistorPower({ u, i, r }) : null);
  const computed = $derived({ u: result != null && u === null, i: result != null && i === null, r: result != null && r === null });

  $effect(() => {
    if (!result) return;
    if (computed.u && result.u != null) uStr = fmtNum(result.u / uFactor[uUnit]);
    if (computed.i && result.i != null) iStr = fmtNum(result.i / iFactor[iUnit]);
    if (computed.r && result.r != null) rStr = fmtNum(result.r / rFactor[rUnit]);
  });

  function fmtNum(n) { return n == null ? '' : String(Math.round(n * 10000) / 10000); }
  function reset() { uStr = ''; iStr = ''; rStr = ''; }

  const statusText = $derived(filledCount === 0 ? 'Введи две величины' : filledCount === 1 ? 'Введи ещё одну' : filledCount === 2 ? (result ? 'Готово' : 'Не удалось') : 'Введи ровно две');
  const statusKind = $derived(filledCount < 2 ? 'hint' : filledCount === 2 ? (result ? 'ok' : 'error') : 'warning');

  const e24 = $derived(result?.r != null ? nearestE24(result.r, 3) : []);

  const warnings = $derived.by(() => {
    if (!result) return [];
    const list = [];
    if (result.p > 5) list.push('Мощность больше 5 Вт — нужен мощный резистор');
    if (result.i > 1) list.push('Ток больше 1 А — проверь проводники');
    return list;
  });

  let copied = $state(false);
  const copyText = $derived.by(() => {
    if (!result) return '';
    const parts = [`R = ${formatOhm(result.r)}`, `P = ${formatPower(result.p)}`];
    if (result.recommendedPower) parts.push(`Рекомендуемая мощность: ${result.recommendedPower} Вт`);
    return parts.join('\n');
  });
  async function copyResult() { if (!copyText) return; try { await navigator.clipboard.writeText(copyText); copied = true; setTimeout(() => (copied = false), 1200); } catch {} }
</script>

<div class="pick">
  <div class="fields">
    <div class="field-row">
      <div class="field-row__input"><Input bind:value={uStr} label="Напряжение (U)" placeholder="12" inputmode="decimal" computed={computed.u} /></div>
      <Select bind:value={uUnit} options={voltageUnits} compact />
    </div>
    <div class="field-row">
      <div class="field-row__input"><Input bind:value={iStr} label="Ток (I)" placeholder="0.02" inputmode="decimal" computed={computed.i} /></div>
      <Select bind:value={iUnit} options={currentUnits} compact />
    </div>
    <div class="field-row">
      <div class="field-row__input"><Input bind:value={rStr} label="Сопротивление (R)" placeholder="600" inputmode="decimal" computed={computed.r} /></div>
      <Select bind:value={rUnit} options={ohmUnits} compact />
    </div>
  </div>

  <div class="status status--{statusKind}">{statusText}</div>

  {#if result}
    <div class="result">
      <div class="result__header">Результат</div>
      <div class="result__main">
        <div class="result__value">{formatOhm(result.r)}</div>
        <div class="result__compact">{formatOhmCompact(result.r)}</div>
      </div>
      <div class="result__power">Выделяемая мощность: <strong>{formatPower(result.p)}</strong></div>
      {#if result.recommendedPower}
        <div class="result__recommend">Рекомендуемый резистор: <strong>{result.recommendedPower} Вт</strong></div>
      {/if}
      {#if e24.length > 0}
        <div class="e24">
          <div class="e24__label">Ближайшие E24:</div>
          <div class="e24__list">
            {#each e24 as val, idx (idx)}<span class="chip" class:chip--nearest={idx === 0}>{formatOhmCompact(val)}</span>{/each}
          </div>
        </div>
      {/if}
      {#if warnings.length > 0}
        <ul class="warnings">{#each warnings as w, idx (idx)}<li>{w}</li>{/each}</ul>
      {/if}
      <div class="result__actions">
        <Button variant="secondary" size="sm" onclick={copyResult}>{copied ? 'Скопировано' : 'Копировать'}</Button>
        <Button variant="ghost" size="sm" onclick={reset}>Сброс</Button>
      </div>
    </div>
  {/if}
</div>

<style>
  .pick { display: flex; flex-direction: column; gap: var(--space-4); }
  .fields { display: flex; flex-direction: column; gap: var(--space-3); }
  .field-row { display: flex; gap: var(--space-2); align-items: flex-end; }
  .field-row__input { flex: 1; }
  .status { font-size: var(--font-size-sm); padding: var(--space-2) var(--space-3); border-radius: var(--radius-md); border: 1px solid transparent; }
  .status--hint { color: var(--text-muted); background: var(--bg-elevated-2); }
  .status--ok { color: var(--success); background: rgba(102, 187, 106, 0.08); }
  .status--warning { color: var(--warning); background: rgba(255, 167, 38, 0.08); }
  .status--error { color: var(--danger); background: rgba(239, 83, 80, 0.08); }
  .result { display: flex; flex-direction: column; gap: var(--space-3); padding: var(--space-4); background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); }
  .result__header { font-size: var(--font-size-sm); text-transform: uppercase; color: var(--text-muted); }
  .result__main { display: flex; align-items: baseline; gap: var(--space-3); }
  .result__value { font-size: 2rem; font-weight: 700; color: var(--accent); }
  .result__compact { font-family: var(--font-mono); color: var(--text-muted); }
  .result__power, .result__recommend { font-size: var(--font-size-base); color: var(--text-secondary); }
  .result__power strong, .result__recommend strong { color: var(--accent); }
  .e24 { display: flex; flex-direction: column; gap: var(--space-2); }
  .e24__label { font-size: var(--font-size-sm); color: var(--text-muted); }
  .e24__list { display: flex; gap: var(--space-2); flex-wrap: wrap; }
  .chip { padding: 4px 10px; background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-sm); font-family: var(--font-mono); font-size: var(--font-size-sm); color: var(--text-secondary); }
  .chip--nearest { color: var(--accent); border-color: var(--accent-glow); }
  .warnings { list-style: none; display: flex; flex-direction: column; gap: var(--space-1); }
  .warnings li { padding: var(--space-2) var(--space-3); color: var(--warning); background: rgba(255, 167, 38, 0.08); border-radius: var(--radius-md); font-size: var(--font-size-sm); }
  .result__actions { display: flex; gap: var(--space-2); flex-wrap: wrap; }
</style>