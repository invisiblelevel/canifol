<script>
  import Input from '../../components/Input.svelte';
  import Select from '../../components/Select.svelte';
  import Button from '../../components/Button.svelte';
  import { getStrings } from '../../strings.svelte.js';
  import { solveVoltageDivider, formatVoltage } from '../../utils/formulas.js';
  import { formatOhm } from '../../utils/resistorCodes.js';

  const strings = $derived(getStrings());

  let uinStr = $state(''), uoutStr = $state(''), r1Str = $state(''), r2Str = $state('');
  let uinUnit = $state('v'), uoutUnit = $state('v'), r1Unit = $state('ohm'), r2Unit = $state('ohm');

  const voltageUnits = $derived([
    { value: 'v', label: strings.units.v },
    { value: 'mv', label: strings.units.mv }
  ]);
  const ohmUnits = $derived([
    { value: 'ohm', label: strings.units.ohm },
    { value: 'kohm', label: strings.units.kohm },
    { value: 'mohm', label: strings.units.mohm }
  ]);
  const vFactor = { v: 1, mv: 1e-3 };
  const rFactor = { ohm: 1, kohm: 1e3, mohm: 1e6 };

  function parseField(s) { if (s == null) return null; const t = String(s).trim().replace(',', '.'); if (!t) return null; const n = parseFloat(t); return isNaN(n) ? null : n; }

  const uin = $derived(parseField(uinStr) != null ? parseField(uinStr) * vFactor[uinUnit] : null);
  const uout = $derived(parseField(uoutStr) != null ? parseField(uoutStr) * vFactor[uoutUnit] : null);
  const r1 = $derived(parseField(r1Str) != null ? parseField(r1Str) * rFactor[r1Unit] : null);
  const r2 = $derived(parseField(r2Str) != null ? parseField(r2Str) * rFactor[r2Unit] : null);

  const filledCount = $derived([uin, uout, r1, r2].filter((v) => v !== null).length);
  const result = $derived(filledCount === 3 ? solveVoltageDivider({ uin, uout, r1, r2 }) : null);
  const computed = $derived({ uin: result != null && uin === null, uout: result != null && uout === null, r1: result != null && r1 === null, r2: result != null && r2 === null });

  $effect(() => {
    if (!result) return;
    if (computed.uin && result.uin != null) uinStr = fmtNum(result.uin / vFactor[uinUnit]);
    if (computed.uout && result.uout != null) uoutStr = fmtNum(result.uout / vFactor[uoutUnit]);
    if (computed.r1 && result.r1 != null) r1Str = fmtNum(result.r1 / rFactor[r1Unit]);
    if (computed.r2 && result.r2 != null) r2Str = fmtNum(result.r2 / rFactor[r2Unit]);
  });

  function fmtNum(n) { return n == null ? '' : String(Math.round(n * 10000) / 10000); }
  function reset() { uinStr = ''; uoutStr = ''; r1Str = ''; r2Str = ''; }

  const statusText = $derived(
    filledCount === 0 ? strings.resistors.labels.enterThree
    : filledCount === 1 ? strings.resistors.labels.enterTwoMore
    : filledCount === 2 ? strings.resistors.labels.enterOneMore
    : filledCount === 3 ? (result ? strings.resistors.labels.ready : strings.resistors.labels.calcFailed)
    : strings.resistors.labels.enterExactlyThree
  );
  const statusKind = $derived(filledCount < 3 ? 'hint' : filledCount === 3 ? (result ? 'ok' : 'error') : 'warning');

  let copied = $state(false);
  const copyText = $derived.by(() => {
    if (!result) return '';
    const parts = [];
    if (result.uin != null) parts.push(`${strings.resistors.labels.uin} = ${formatVoltage(result.uin)}`);
    if (result.uout != null) parts.push(`${strings.resistors.labels.uout} = ${formatVoltage(result.uout)}`);
    if (result.r1 != null) parts.push(`R1 = ${formatOhm(result.r1)}`);
    if (result.r2 != null) parts.push(`R2 = ${formatOhm(result.r2)}`);
    return parts.join('\n');
  });
  async function copyResult() { if (!copyText) return; try { await navigator.clipboard.writeText(copyText); copied = true; setTimeout(() => (copied = false), 1200); } catch {} }
</script>

<div class="divider">
  <div class="fields">
    <div class="field-row">
      <div class="field-row__input"><Input bind:value={uinStr} label={strings.resistors.labels.uin} placeholder="12" inputmode="decimal" computed={computed.uin} /></div>
      <Select bind:value={uinUnit} options={voltageUnits} compact />
    </div>
    <div class="field-row">
      <div class="field-row__input"><Input bind:value={uoutStr} label={strings.resistors.labels.uout} placeholder="5" inputmode="decimal" computed={computed.uout} /></div>
      <Select bind:value={uoutUnit} options={voltageUnits} compact />
    </div>
    <div class="field-row">
      <div class="field-row__input"><Input bind:value={r1Str} label="R1" placeholder="1000" inputmode="decimal" computed={computed.r1} /></div>
      <Select bind:value={r1Unit} options={ohmUnits} compact />
    </div>
    <div class="field-row">
      <div class="field-row__input"><Input bind:value={r2Str} label="R2" placeholder="470" inputmode="decimal" computed={computed.r2} /></div>
      <Select bind:value={r2Unit} options={ohmUnits} compact />
    </div>
  </div>

  <div class="status status--{statusKind}">{statusText}</div>

  {#if result}
    <div class="result">
      <div class="result__header">{strings.resistors.labels.result}</div>
      <ul class="result__list">
        {#if computed.uin && result.uin != null}<li>{strings.resistors.labels.uin} = {formatVoltage(result.uin)}</li>{/if}
        {#if computed.uout && result.uout != null}<li>{strings.resistors.labels.uout} = {formatVoltage(result.uout)}</li>{/if}
        {#if computed.r1 && result.r1 != null}<li>R1 = {formatOhm(result.r1)}</li>{/if}
        {#if computed.r2 && result.r2 != null}<li>R2 = {formatOhm(result.r2)}</li>{/if}
      </ul>
      <div class="result__actions">
        <Button variant="secondary" size="sm" onclick={copyResult}>{copied ? strings.buttons.copied : strings.buttons.copy}</Button>
        <Button variant="ghost" size="sm" onclick={reset}>{strings.buttons.reset}</Button>
      </div>
    </div>
  {/if}
</div>

<style>
  .divider { display: flex; flex-direction: column; gap: var(--space-4); }
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
  .result__list { list-style: none; display: flex; flex-direction: column; gap: var(--space-1); }
  .result__list li { font-size: var(--font-size-lg); font-weight: 600; color: var(--accent); font-family: var(--font-mono); }
  .result__actions { display: flex; gap: var(--space-2); flex-wrap: wrap; }
</style>