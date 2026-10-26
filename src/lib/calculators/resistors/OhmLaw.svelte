<script>
  import Input from '../../components/Input.svelte';
  import Button from '../../components/Button.svelte';
  import { solveOhmLaw, formatVoltage, formatCurrent, formatPower } from '../../utils/formulas.js';
  import { formatOhm } from '../../utils/resistorCodes.js';

  let uStr = $state(''), iStr = $state(''), rStr = $state(''), pStr = $state('');

  const uVal = $derived(parseField(uStr));
  const iVal = $derived(parseField(iStr));
  const rVal = $derived(parseField(rStr));
  const pVal = $derived(parseField(pStr));

  const filled = $derived([uVal, iVal, rVal, pVal].filter((v) => v !== null).length);
  const result = $derived(filled === 2 ? solveOhmLaw({ u: uVal, i: iVal, r: rVal, p: pVal }) : null);
  const computed = $derived({ u: result != null && uVal === null, i: result != null && iVal === null, r: result != null && rVal === null, p: result != null && pVal === null });

  $effect(() => {
    if (!result) return;
    if (computed.u) uStr = result.u != null ? fmtInput(result.u) : '';
    if (computed.i) iStr = result.i != null ? fmtInput(result.i) : '';
    if (computed.r) rStr = result.r != null ? fmtInput(result.r) : '';
    if (computed.p) pStr = result.p != null ? fmtInput(result.p) : '';
  });

  function parseField(s) { if (s == null) return null; const t = String(s).trim().replace(',', '.'); if (!t) return null; const n = parseFloat(t); return isNaN(n) ? null : n; }
  function fmtInput(n) { return n == null ? '' : String(Math.round(n * 1000) / 1000); }
  function reset() { uStr = ''; iStr = ''; rStr = ''; pStr = ''; }

  const statusText = $derived(filled === 0 ? 'Введи любые две величины' : filled === 1 ? 'Введи ещё одну величину' : filled === 2 ? (result ? 'Готово' : 'Не удалось рассчитать') : 'Введи ровно две величины');
  const statusKind = $derived(filled < 2 ? 'hint' : filled === 2 ? (result ? 'ok' : 'error') : 'warning');

  const resultLines = $derived.by(() => {
    if (!result) return [];
    const lines = [];
    if (computed.u && result.u != null) lines.push(`U = ${formatVoltage(result.u)}`);
    if (computed.i && result.i != null) lines.push(`I = ${formatCurrent(result.i)}`);
    if (computed.r && result.r != null) lines.push(`R = ${formatOhm(result.r)}`);
    if (computed.p && result.p != null) lines.push(`P = ${formatPower(result.p)}`);
    return lines;
  });

  let copied = $state(false);
  async function copyResult() {
    if (!resultLines.length) return;
    try { await navigator.clipboard.writeText(resultLines.join('\n')); copied = true; setTimeout(() => (copied = false), 1200); } catch {}
  }
</script>

<div class="ohm">
  <div class="ohm__fields">
    <Input bind:value={uStr} label="Напряжение (U)" placeholder="12" suffix="В" inputmode="decimal" computed={computed.u} />
    <Input bind:value={iStr} label="Ток (I)" placeholder="0.5" suffix="А" inputmode="decimal" computed={computed.i} />
    <Input bind:value={rStr} label="Сопротивление (R)" placeholder="1000" suffix="Ом" inputmode="decimal" computed={computed.r} />
    <Input bind:value={pStr} label="Мощность (P)" placeholder="0.25" suffix="Вт" inputmode="decimal" computed={computed.p} />
  </div>

  <div class="status status--{statusKind}">{statusText}</div>

  {#if result && resultLines.length > 0}
    <div class="result">
      <div class="result__header">Результат</div>
      <ul class="result__list">{#each resultLines as line, idx (idx)}<li>{line}</li>{/each}</ul>
      <div class="result__actions">
        <Button variant="secondary" size="sm" onclick={copyResult}>{copied ? 'Скопировано' : 'Копировать'}</Button>
        <Button variant="ghost" size="sm" onclick={reset}>Сброс</Button>
      </div>
    </div>
  {/if}
</div>

<style>
  .ohm { display: flex; flex-direction: column; gap: var(--space-4); }
  .ohm__fields { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: var(--space-3); }
  .status { font-size: var(--font-size-sm); padding: var(--space-2) var(--space-3); border-radius: var(--radius-md); border: 1px solid transparent; }
  .status--hint { color: var(--text-muted); background: var(--bg-elevated-2); border-color: var(--border-subtle); }
  .status--ok { color: var(--success); background: rgba(102, 187, 106, 0.08); border-color: rgba(102, 187, 106, 0.25); }
  .status--warning { color: var(--warning); background: rgba(255, 167, 38, 0.08); border-color: rgba(255, 167, 38, 0.25); }
  .status--error { color: var(--danger); background: rgba(239, 83, 80, 0.08); border-color: rgba(239, 83, 80, 0.25); }
  .result { display: flex; flex-direction: column; gap: var(--space-3); padding: var(--space-4); background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); }
  .result__header { font-size: var(--font-size-sm); text-transform: uppercase; color: var(--text-muted); }
  .result__list { list-style: none; display: flex; flex-direction: column; gap: var(--space-1); }
  .result__list li { font-size: var(--font-size-lg); font-weight: 600; color: var(--accent); font-family: var(--font-mono); }
  .result__actions { display: flex; gap: var(--space-2); flex-wrap: wrap; }
</style>