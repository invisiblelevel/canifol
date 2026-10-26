<script>
  import Input from '../../components/Input.svelte';
  import Select from '../../components/Select.svelte';
  import Button from '../../components/Button.svelte';
  import { getStrings } from '../../strings.svelte.js';
  import { capacitiveReactance, formatFrequency } from '../../utils/formulas.js';
  import { formatFarad } from '../../utils/capacitorCodes.js';
  import { formatOhm } from '../../utils/resistorCodes.js';

  const strings = $derived(getStrings());

  let fStr = $state('');
  let cStr = $state('');
  let xcStr = $state('');

  let fUnit = $state('hz');
  let cUnit = $state('uf');
  let xcUnit = $state('ohm');

  const fUnits = $derived([
    { value: 'hz', label: strings.units.hz },
    { value: 'khz', label: strings.units.khz },
    { value: 'mhz', label: strings.units.mhz }
  ]);
  const cUnits = $derived([
    { value: 'pf', label: strings.units.pf },
    { value: 'nf', label: strings.units.nf },
    { value: 'uf', label: strings.units.uf },
    { value: 'mf', label: strings.units.mf }
  ]);
  const xcUnits = $derived([
    { value: 'ohm', label: strings.units.ohm },
    { value: 'kohm', label: strings.units.kohm },
    { value: 'mohm', label: strings.units.mohm }
  ]);

  const fFactor = { hz: 1, khz: 1e3, mhz: 1e6 };
  const cFactor = { pf: 1e-12, nf: 1e-9, uf: 1e-6, mf: 1e-3 };
  const xcFactor = { ohm: 1, kohm: 1e3, mohm: 1e6 };

  function parseField(s) {
    if (s == null) return null;
    const t = String(s).trim().replace(',', '.');
    if (!t) return null;
    const n = parseFloat(t);
    return isNaN(n) ? null : n;
  }

  const f = $derived(parseField(fStr) != null ? parseField(fStr) * fFactor[fUnit] : null);
  const c = $derived(parseField(cStr) != null ? parseField(cStr) * cFactor[cUnit] : null);
  const xc = $derived(parseField(xcStr) != null ? parseField(xcStr) * xcFactor[xcUnit] : null);

  const filledCount = $derived([f, c, xc].filter((v) => v !== null).length);

  const result = $derived.by(() => {
    if (filledCount !== 2) return null;
    let outF = f, outC = c, outXc = xc;
    if (outXc === null && outF !== null && outC !== null) {
      outXc = capacitiveReactance(outF, outC);
    } else if (outF === null && outXc !== null && outC !== null) {
      if (outXc === 0 || outC === 0) return null;
      outF = 1 / (2 * Math.PI * outXc * outC);
    } else if (outC === null && outF !== null && outXc !== null) {
      if (outF === 0 || outXc === 0) return null;
      outC = 1 / (2 * Math.PI * outF * outXc);
    }
    return { f: outF, c: outC, xc: outXc };
  });

  const computed = $derived({
    f: result != null && f === null,
    c: result != null && c === null,
    xc: result != null && xc === null
  });

  $effect(() => {
    if (!result) return;
    if (computed.f && result.f != null) fStr = fmtNum(result.f / fFactor[fUnit]);
    if (computed.c && result.c != null) cStr = fmtNum(result.c / cFactor[cUnit]);
    if (computed.xc && result.xc != null) xcStr = fmtNum(result.xc / xcFactor[xcUnit]);
  });

  function fmtNum(n) {
    if (n == null) return '';
    return String(Math.round(n * 10000) / 10000);
  }

  function reset() {
    fStr = '';
    cStr = '';
    xcStr = '';
  }

  const statusText = $derived(
    filledCount === 0 ? strings.capacitors.labels.enterTwo
    : filledCount === 1 ? strings.capacitors.labels.enterOneMore
    : filledCount === 2 ? (result ? strings.capacitors.labels.ready : strings.capacitors.labels.calcFailed)
    : strings.capacitors.labels.enterExactlyTwo
  );

  const statusKind = $derived(
    filledCount < 2 ? 'hint' : filledCount === 2 ? (result ? 'ok' : 'error') : 'warning'
  );
</script>

<div class="xc-calc">
  <div class="fields">
    <div class="field-row">
      <div class="field-row__input">
        <Input bind:value={fStr} label={strings.capacitors.labels.frequency} placeholder="1000" inputmode="decimal" computed={computed.f} />
      </div>
      <Select bind:value={fUnit} options={fUnits} compact aria-label={strings.capacitors.labels.frequency} />
    </div>
    <div class="field-row">
      <div class="field-row__input">
        <Input bind:value={cStr} label={strings.capacitors.labels.capacitance} placeholder="1" inputmode="decimal" computed={computed.c} />
      </div>
      <Select bind:value={cUnit} options={cUnits} compact aria-label={strings.capacitors.labels.capacitance} />
    </div>
    <div class="field-row">
      <div class="field-row__input">
        <Input bind:value={xcStr} label={strings.capacitors.labels.reactanceC} placeholder="159" inputmode="decimal" computed={computed.xc} />
      </div>
      <Select bind:value={xcUnit} options={xcUnits} compact aria-label={strings.capacitors.labels.reactanceC} />
    </div>
  </div>

  <div class="status status--{statusKind}" aria-live="polite">{statusText}</div>

  {#if result}
    <div class="result" aria-live="polite">
      <div class="result__header">{strings.capacitors.labels.result}</div>
      <ul class="result__list">
        {#if computed.f && result.f != null}
          <li>f = {formatFrequency(result.f)}</li>
        {/if}
        {#if computed.c && result.c != null}
          <li>C = {formatFarad(result.c).value} {strings.units[formatFarad(result.c).unit]}</li>
        {/if}
        {#if computed.xc && result.xc != null}
          <li>Xc = {formatOhm(result.xc)}</li>
        {/if}
      </ul>
      <div class="result__actions">
        <Button variant="ghost" size="sm" onclick={reset}>{strings.buttons.reset}</Button>
      </div>
    </div>
  {:else if filledCount === 2}
    <div class="result__actions">
      <Button variant="ghost" size="sm" onclick={reset}>{strings.buttons.reset}</Button>
    </div>
  {/if}
</div>

<style>
  .xc-calc { display: flex; flex-direction: column; gap: var(--space-4); }
  .fields { display: flex; flex-direction: column; gap: var(--space-3); }
  .field-row { display: flex; gap: var(--space-2); align-items: flex-end; }
  .field-row__input { flex: 1; }
  .status { font-size: var(--font-size-sm); padding: var(--space-2) var(--space-3); border-radius: var(--radius-md); border: 1px solid transparent; }
  .status--hint { color: var(--text-muted); background: var(--bg-elevated-2); border-color: var(--border-subtle); }
  .status--ok { color: var(--success); background: rgba(111, 206, 117, 0.08); border-color: rgba(111, 206, 117, 0.25); }
  .status--warning { color: var(--warning); background: rgba(247, 178, 74, 0.08); border-color: rgba(247, 178, 74, 0.25); }
  .status--error { color: var(--danger); background: rgba(242, 106, 106, 0.08); border-color: rgba(242, 106, 106, 0.25); }
  .result { display: flex; flex-direction: column; gap: var(--space-3); padding: var(--space-4); background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); }
  .result__header { font-size: var(--font-size-sm); text-transform: uppercase; color: var(--text-muted); }
  .result__list { list-style: none; display: flex; flex-direction: column; gap: var(--space-1); }
  .result__list li { font-size: var(--font-size-lg); font-weight: 600; color: var(--accent); font-family: var(--font-mono); }
  .result__actions { display: flex; gap: var(--space-2); }
  @media (max-width: 768px) { .result { padding: var(--space-3); } }
</style>