<script>
  import Input from '../../components/Input.svelte';
  import Button from '../../components/Button.svelte';
  import { getStrings } from '../../strings.svelte.js';

  const strings = $derived(getStrings());

  let uIn = $state('5');     // Напряжение управления (например, с ножки МК)
  let uLoad = $state('12');  // Напряжение нагрузки (например, питание реле)
  let rLoad = $state('100'); // Сопротивление нагрузки
  let hfe = $state('100');   // Коэффициент усиления (мин)
  let vBe = $state('0.7');   // Падение база-эмиттер

  const uInNum = $derived(parseFloat(uIn.replace(',', '.')) || 0);
  const uLoadNum = $derived(parseFloat(uLoad.replace(',', '.')) || 0);
  const rLoadNum = $derived(parseFloat(rLoad.replace(',', '.')) || 0);
  const hfeNum = $derived(parseFloat(hfe.replace(',', '.')) || 0);
  const vBeNum = $derived(parseFloat(vBe.replace(',', '.')) || 0);

  const result = $derived.by(() => {
    if (uInNum <= 0 || uLoadNum <= 0 || rLoadNum <= 0 || hfeNum <= 0 || vBeNum <= 0 || uInNum <= vBeNum) return null;

    // 1. Ток коллектора (нагрузки)
    const iC = uLoadNum / rLoadNum;
    
    // 2. Ток базы для насыщения (с коэффициентом запаса 2.5)
    const saturationFactor = 2.5;
    const iB = (iC / hfeNum) * saturationFactor;

    // 3. Базовый резистор
    const rB = (uInNum - vBeNum) / iB;

    // 4. Мощность на базовом резисторе
    const pB = (uInNum - vBeNum) * iB;

    return { iC, iB, rB, pB };
  });

  function reset() {
    uIn = '5'; uLoad = '12'; rLoad = '100'; hfe = '100'; vBe = '0.7';
  }
</script>

<div class="calc">
  <div class="fields">
    <Input bind:value={uIn} label={strings.transistors.labels.uIn} placeholder="5" suffix={strings.units.v} inputmode="decimal" />
    <Input bind:value={uLoad} label={strings.transistors.labels.uLoad} placeholder="12" suffix={strings.units.v} inputmode="decimal" />
    <Input bind:value={rLoad} label={strings.transistors.labels.rLoad} placeholder="100" suffix={strings.units.ohm} inputmode="decimal" />
    <Input bind:value={hfe} label={strings.transistors.labels.hfe} placeholder="100" inputmode="decimal" />
    <Input bind:value={vBe} label={strings.transistors.labels.vBe} placeholder="0.7" suffix={strings.units.v} inputmode="decimal" />
  </div>

  {#if result}
    <div class="result" aria-live="polite">
      <div class="result__main">
        <div class="result__value">{Math.round(result.rB)} {strings.units.ohm}</div>
        <div class="result__label">{strings.transistors.labels.baseResistor}</div>
      </div>
      <div class="result__meta">
        <span class="badge">{strings.transistors.labels.baseCurrent}: {(result.iB * 1000).toFixed(1)} {strings.units.ma}</span>
        <span class="badge">{strings.transistors.labels.collectorCurrent}: {(result.iC * 1000).toFixed(1)} {strings.units.ma}</span>
        <span class="badge">{strings.transistors.labels.powerOnResistor}: {result.pB.toFixed(3)} {strings.units.w}</span>
      </div>
      <div class="note">{strings.transistors.labels.note}</div>
      <div class="result__actions">
        <Button variant="ghost" size="sm" onclick={reset}>{strings.buttons.reset}</Button>
      </div>
    </div>
  {:else}
    <div class="error-note">{strings.common.enterValues}</div>
  {/if}
</div>

<style>
  .calc { display: flex; flex-direction: column; gap: var(--space-4); }
  .fields { display: grid; grid-template-columns: repeat(auto-fit, minmax(160px, 1fr)); gap: var(--space-3); }
  .result { display: flex; flex-direction: column; gap: var(--space-3); padding: var(--space-4); background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); }
  .result__main { display: flex; align-items: baseline; gap: var(--space-3); }
  .result__value { font-size: clamp(1.5rem, 5vw, 2.25rem); font-weight: 700; color: var(--accent); line-height: 1.1; }
  .result__label { font-size: var(--font-size-sm); color: var(--text-muted); }
  .result__meta { display: flex; gap: var(--space-2); flex-wrap: wrap; }
  .badge { display: inline-flex; align-items: center; padding: 4px 10px; font-size: var(--font-size-sm); color: var(--text-secondary); background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: 999px; }
  .note { font-size: var(--font-size-sm); color: var(--text-muted); font-style: italic; margin-top: var(--space-2); }
  .error-note { padding: var(--space-3); color: var(--text-muted); text-align: center; font-size: var(--font-size-sm); }
  .result__actions { display: flex; gap: var(--space-2); margin-top: var(--space-2); }
  @media (max-width: 768px) { .result { padding: var(--space-3); } }
</style>