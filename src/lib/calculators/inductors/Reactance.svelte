<script>
  import Input from '../../components/Input.svelte';
  import Select from '../../components/Select.svelte';
  import Button from '../../components/Button.svelte';
  import { getStrings } from '../../strings.svelte.js';
  import { inductiveReactance, formatFrequency } from '../../utils/formulas.js';
  import { formatHenry } from '../../utils/inductorCodes.js';
  import { formatOhm } from '../../utils/resistorCodes.js';

  const strings = $derived(getStrings());

  let fStr = $state('');
  let lStr = $state('');
  let xlStr = $state('');

  let fUnit = $state('hz');
  let lUnit = $state('uh');
  let xlUnit = $state('ohm');

  const fUnits = $derived([
    { value: 'hz', label: strings.units.hz },
    { value: 'khz', label: strings.units.khz },
    { value: 'mhz', label: strings.units.mhz }
  ]);
  const lUnits = $derived([
    { value: 'nh', label: strings.units.nh },
    { value: 'uh', label: strings.units.uh },
    { value: 'mh', label: strings.units.mh },
    { value: 'h', label: strings.units.h }
  ]);
  const xlUnits = $derived([
    { value: 'ohm', label: strings.units.ohm },
    { value: 'kohm', label: strings.units.kohm },
    { value: 'mohm', label: strings.units.mohm }
  ]);

  const fFactor = { hz: 1, khz: 1e3, mhz: 1e6 };
  const lFactor = { nh: 1e-9, uh: 1e-6, mh: 1e-3, h: 1 };
  const xlFactor = { ohm: 1, kohm: 1e3, mohm: 1e6 };

  function parseField(s) {
    if (s == null) return null;
    const t = String(s).trim().replace(',', '.');
    if (!t) return null;
    const n = parseFloat(t);
    return isNaN(n) ? null : n;
  }

  const f = $derived(parseField(fStr) != null ? parseField(fStr) * fFactor[fUnit] : null);
  const l = $derived(parseField(lStr) != null ? parseField(lStr) * lFactor[lUnit] : null);
  const xl = $derived(parseField(xlStr) != null ? parseField(xlStr) * xlFactor[xlUnit] : null);

  const filledCount = $derived([f, l, xl].filter((v) => v !== null).length);

  const result = $derived.by(() => {
    if (filledCount !== 2) return null;
    let outF = f, outL = l, outXl = xl;

    if (outXl === null && outF !== null && outL !== null) {
      outXl = inductiveReactance(outF, outL);
    } else if (outF === null && outXl !== null && outL !== null) {
      if (outL === 0) return null;
      outF = outXl / (2 * Math.PI * outL);
    } else if (outL === null && outF !== null && outXl !== null) {
      if (outF === 0) return null;
      outL = outXl / (2 * Math.PI * outF);
    }

    return { f: outF, l: outL, xl: outXl };
  });

  const computed = $derived({
    f: result != null && f === null,
    l: result != null && l === null,
    xl: result != null && xl === null
  });

  $effect(() => {
    if (!result) return;
    if (computed.f && result.f != null) fStr = fmtNum(result.f / fFactor[fUnit]);
    if (computed.l && result.l != null) lStr = fmtNum(result.l / lFactor[lUnit]);
    if (computed.xl && result.xl != null) xlStr = fmtNum(result.xl / xlFactor[xlUnit]);
  });

  function fmtNum(n) {
    if (n == null) return '';
    return String(Math.round(n * 10000) / 10000);
  }

  function reset() {
    fStr = '';
    lStr = '';
    xlStr = '';
  }

  const statusText = $derived(
    filledCount === 0 ? strings.inductors.labels.enterTwo
    : filledCount === 1 ? strings.inductors.labels.enterOneMore
    : filledCount === 2 ? (result ? strings.inductors.labels.ready : strings.inductors.labels.calcFailed)
    : strings.inductors.labels.enterExactlyTwo
  );

  const statusKind = $derived(
    filledCount < 2 ? 'hint' : filledCount === 2 ? (result ? 'ok' : 'error') : 'warning'
  );
</script>

<div class="xl-calc">
  <div class="fields">
    <div class="field-row">
      <div class="field-row__input">
        <Input bind:value={fStr} label={strings.inductors.labels.frequency} placeholder="1000" inputmode="decimal" computed={computed.f} />
      </div>
      <Select bind:value={fUnit} options={fUnits} compact aria-label={strings.inductors.labels.frequency} />
    </div>
    <div class="field-row">
      <div class="field-row__input">
        <Input bind:value={lStr} label={strings.inductors.labels.inductance} placeholder="1" inputmode="decimal" computed={computed.l} />
      </div>
      <Select bind:value={lUnit} options={lUnits} compact aria-label={strings.inductors.labels.inductance} />
    </div>
    <div class="field-row">
      <div class="field-row__input">
        <Input bind:value={xlStr} label={strings.inductors.labels.reactanceL} placeholder="6.28" inputmode="decimal" computed={computed.xl} />
      </div>
      <Select bind:value={xlUnit} options={xlUnits} compact aria-label={strings.inductors.labels.reactanceL} />
    </div>
  </div>

  <div class="status status--{statusKind}" aria-live="polite">{statusText}</div>

  {#if result}
    <div class="result" aria-live="polite">
      <div class="result__header">{strings.inductors.labels.result}</div>
      <ul class="result__list">
        {#if computed.f && result.f != null}
          <li>f = {formatFrequency(result.f)}</li>
        {/if}
        {#if computed.l && result.l != null}
          <li>L = {formatHenry(result.l).value} {strings.units[formatHenry(result.l).unit]}</li>
        {/if}
        {#if computed.xl && result.xl != null}
          <li>Xl = {formatOhm(result.xl)}</li>
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
  .xl-calc { display: flex; flex-direction: column; gap: var(--space-4); }
  .fields { display: flex; flex-direction: column; gap: var(--space-3); }
  .field-row { display: flex; gap: var(--space-2); align-items: flex-end; }
  .field-row__input { flex: 1; }
  .status { font-size: var(--font-size-sm); padding: var(--space-2) var(--space-3); border-radius: var(--radius-md); border: 1px solid transparent; }
  .status--hint { color: var(--text-muted); background: var(--bg-elevated-2); border-color: var(--border-subtle); }
  .status--ok { color: var(--success); background: rgba(139, 196, 122, 0.08); border-color: rgba(139, 196, 122, 0.25); }
  .status--warning { color: var(--warning); background: rgba(230, 164, 32, 0.08); border-color: rgba(230, 164, 32, 0.25); }
  .status--error { color: var(--danger); background: rgba(232, 90, 90, 0.08); border-color: rgba(232, 90, 90, 0.25); }
  .result { display: flex; flex-direction: column; gap: var(--space-3); padding: var(--space-4); background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); }
  .result__header { font-size: var(--font-size-sm); text-transform: uppercase; color: var(--text-muted); }
  .result__list { list-style: none; display: flex; flex-direction: column; gap: var(--space-1); }
  .result__list li { font-size: var(--font-size-lg); font-weight: 600; color: var(--accent); font-family: var(--font-mono); }
  .result__actions { display: flex; gap: var(--space-2); }
  @media (max-width: 768px) { .result { padding: var(--space-3); } }
</style>