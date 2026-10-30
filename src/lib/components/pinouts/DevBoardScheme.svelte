<script>
  import { getStrings } from '../../strings.svelte.js';

  let { activeId = '' } = $props();
  const strings = $derived(getStrings());
  const data = $derived(strings.pinoutsDevBoardsData[activeId]);
</script>

{#if activeId === 'arduino-uno'}
  <!-- Arduino Uno R3 -->
  <svg viewBox="0 0 400 200" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <!-- Плата -->
    <rect x="40" y="50" width="320" height="100" rx="8" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <!-- Верхняя гребёнка (Digital) -->
    <g fill="var(--text-secondary)">
      <rect x="60" y="40" width="10" height="12" rx="1" />
      <rect x="85" y="40" width="10" height="12" rx="1" />
      <rect x="110" y="40" width="10" height="12" rx="1" />
      <rect x="135" y="40" width="10" height="12" rx="1" />
      <rect x="160" y="40" width="10" height="12" rx="1" />
      <rect x="185" y="40" width="10" height="12" rx="1" />
      <rect x="210" y="40" width="10" height="12" rx="1" />
      <rect x="235" y="40" width="10" height="12" rx="1" />
      <rect x="260" y="40" width="10" height="12" rx="1" />
      <rect x="285" y="40" width="10" height="12" rx="1" />
      <rect x="310" y="40" width="10" height="12" rx="1" />
      <rect x="335" y="40" width="10" height="12" rx="1" />
    </g>
    <!-- Нижняя гребёнка (Power) -->
    <g fill="var(--text-secondary)">
      <rect x="60" y="148" width="10" height="12" rx="1" />
      <rect x="85" y="148" width="10" height="12" rx="1" />
      <rect x="110" y="148" width="10" height="12" rx="1" />
      <rect x="135" y="148" width="10" height="12" rx="1" />
      <rect x="160" y="148" width="10" height="12" rx="1" />
      <rect x="185" y="148" width="10" height="12" rx="1" />
      <rect x="210" y="148" width="10" height="12" rx="1" />
      <rect x="235" y="148" width="10" height="12" rx="1" />
      <rect x="260" y="148" width="10" height="12" rx="1" />
      <rect x="285" y="148" width="10" height="12" rx="1" />
      <rect x="310" y="148" width="10" height="12" rx="1" />
      <rect x="335" y="148" width="10" height="12" rx="1" />
    </g>
    <!-- Подписи -->
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
      <text x="65" y="36">1</text><text x="90" y="36">2</text><text x="115" y="36">3</text><text x="140" y="36">4</text>
      <text x="165" y="36">5</text><text x="190" y="36">6</text><text x="215" y="36">7</text><text x="240" y="36">8</text>
      <text x="265" y="36">9</text><text x="290" y="36">10</text><text x="315" y="36">11</text><text x="340" y="36">12</text>
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
      <text x="65" y="175">13</text><text x="90" y="175">14</text><text x="115" y="175">15</text><text x="140" y="175">16</text>
      <text x="165" y="175">17</text><text x="190" y="175">18</text><text x="215" y="175">19</text><text x="240" y="175">20</text>
    </g>
    <text x="200" y="105" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{data?.name ?? 'Arduino Uno R3'}</text>
  </svg>

{:else if activeId === 'arduino-nano'}
  <!-- Arduino Nano -->
  <svg viewBox="0 0 300 220" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="10" width="180" height="200" rx="6" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <!-- Левая гребёнка -->
    <g fill="var(--text-secondary)">
      {#each Array(15) as _, i}
        <rect x="48" y={22 + i * 12} width="12" height="8" rx="1" />
      {/each}
    </g>
    <!-- Правая гребёнка -->
    <g fill="var(--text-secondary)">
      {#each Array(15) as _, i}
        <rect x="240" y={22 + i * 12} width="12" height="8" rx="1" />
      {/each}
    </g>
    <!-- Подписи слева -->
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="end">
      {#each Array(15) as _, i}
        <text x="44" y={29 + i * 12}>{i + 1}</text>
      {/each}
    </g>
    <!-- Подписи справа -->
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="start">
      {#each Array(15) as _, i}
        <text x="256" y={29 + i * 12}>{30 - i}</text>
      {/each}
    </g>
    <text x="150" y="112" fill="var(--text-primary)" font-size="12" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{data?.name ?? 'Arduino Nano'}</text>
  </svg>

{:else if activeId === 'esp32-devkit'}
  <!-- ESP32 DevKit V1 -->
  <svg viewBox="0 0 320 220" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="10" width="200" height="200" rx="6" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <!-- Левая гребёнка -->
    <g fill="var(--text-secondary)">
      {#each Array(15) as _, i}
        <rect x="48" y={22 + i * 12} width="12" height="8" rx="1" />
      {/each}
    </g>
    <!-- Правая гребёнка -->
    <g fill="var(--text-secondary)">
      {#each Array(15) as _, i}
        <rect x="260" y={22 + i * 12} width="12" height="8" rx="1" />
      {/each}
    </g>
    <!-- Подписи -->
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="end">
      {#each Array(15) as _, i}
        <text x="44" y={29 + i * 12}>{i + 1}</text>
      {/each}
    </g>
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="start">
      {#each Array(15) as _, i}
        <text x="276" y={29 + i * 12}>{30 - i}</text>
      {/each}
    </g>
    <text x="160" y="112" fill="var(--text-primary)" font-size="12" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{data?.name ?? 'ESP32 DevKit'}</text>
  </svg>

{:else if activeId === 'rpi-40pin'}
  <!-- Raspberry Pi 40-pin -->
  <svg viewBox="0 0 320 120" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <!-- Разъём 2x20 -->
    <rect x="60" y="20" width="200" height="80" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <!-- Верхний ряд (нечётные) -->
    <g fill="var(--text-secondary)">
      {#each Array(20) as _, i}
        <rect x={65 + i * 10} y="32" width="7" height="10" rx="1" />
      {/each}
    </g>
    <!-- Нижний ряд (чётные) -->
    <g fill="var(--text-secondary)">
      {#each Array(20) as _, i}
        <rect x={65 + i * 10} y="58" width="7" height="10" rx="1" />
      {/each}
    </g>
    <!-- Номера -->
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="middle">
      {#each Array(20) as _, i}
        <text x={68.5 + i * 10} y="28">{i * 2 + 1}</text>
        <text x={68.5 + i * 10} y="80">{i * 2 + 2}</text>
      {/each}
    </g>
    <text x="160" y="110" fill="var(--text-primary)" font-size="12" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{data?.name ?? 'Raspberry Pi 40-pin'}</text>
  </svg>

{:else}
  <div class="scheme-placeholder">Нет схемы для этой платы</div>
{/if}

<style>
  .scheme-placeholder {
    color: var(--text-muted);
    font-style: italic;
  }
</style>