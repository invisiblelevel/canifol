<script>
  import { getStrings } from '../../strings.svelte.js';
  import { pinouts } from '../../data/pinouts/index.js';

  let { activeId = '' } = $props();
  const strings = $derived(getStrings());
  const data = $derived(strings.pinoutsIcsData[activeId]);
  const name = $derived(data?.name ?? activeId);

  // DIP
  const dip8 = ['lm358', 'ne555', 'lm386', 'mc34063', 'tda2822', 'ne5532', 'lm393', 'tl072', 'tl082', 'lm833', 'ne5534', 'lm4562', 'tlc555', 'uc3842', 'uc3843', 'uc3844', 'tny267', 'attiny85', '24c02', 'k140ud6', 'k140ud7', 'kr1006vi1', 'kr140ud1208'];
  const dip4 = ['pc817'];
  const dip6 = ['4n35'];
  const dip14 = ['lm324', 'lm339', '74hc00', '74hc02', '74hc04', '74hc08', '74hc14', '74hc32', '74hc74', '74hc86', 'cd4001', 'k155la3', 'k155tm2', 'k561la7', 'k561tm2', 'kr1533la3', 'kr1533tm2', 'k561kt3'];
  const dip16 = ['tl494', 'ka7500', 'sg3524', 'sg3525', 'ka3525', '74hc138', '74hc139', '74hc595', 'cd4017', 'cd4060', 'cd4051', 'uln2003', 'l293d', 'ch340g', 'max232', 'ds3231', 'tea2025', 'k155ir1', 'k155ag3', 'k155kp1', 'k155ie7', 'k561ie10', 'k561ie11'];
  const dip18 = ['uln2803'];
  const dip20 = ['74hc244', '74hc245', '74hc373', 'kr1533ir22'];
  const dip24 = ['k155id3', 'kr580vi53'];
  const dip28 = ['atmega328p', 'atmega8', 'tlc5940'];
  const dip40 = ['kr580vm80a', 'kr580vv55a'];
  
  // TO-220
  const to220_3 = ['lm7805', 'lm317', 'lm7812', 'lm7905', 'lm337', 'irfz44n', 'irf540n', 'kr142en5a', 'kr142en8b'];
  const to220_5 = ['lm2596', 'tda2030', 'tda2050', 'lm1875'];
  
  // SOT
  const sot223 = ['ams1117'];
  const sot23 = ['ob2263', 'al8805', 'al8860'];
  const sot89 = ['pt4115', 'cl6807'];
  const to92 = ['ds18b20'];
  
  // Multiwatt
  const mw9 = ['tda1521'];
  const mw11 = ['lm3886', 'tda7265'];
  const mw13 = ['tda1552q', 'tda1562q'];
  const mw15 = ['tda7294', 'l298n', 'sta540'];
  
  // Модули
  const module4 = ['dht11', 'ws2812b'];
  const module6 = ['hc05'];
  const module8 = ['esp01'];
  const module22 = ['esp12'];
</script>

{#if dip8.includes(activeId)}
  <!-- DIP-8 -->
  <svg viewBox="0 0 240 100" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="20" width="120" height="60" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <path d="M60 45 A5 5 0 0 1 60 55" stroke="var(--text-muted)" stroke-width="2" fill="none"/>
    <g fill="var(--text-secondary)">
      <rect x="50" y="25" width="10" height="6" rx="1" /><rect x="50" y="38" width="10" height="6" rx="1" /><rect x="50" y="51" width="10" height="6" rx="1" /><rect x="50" y="64" width="10" height="6" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="end">
      <text x="45" y="30">1</text><text x="45" y="43">2</text><text x="45" y="56">3</text><text x="45" y="69">4</text>
    </g>
    <g fill="var(--text-secondary)">
      <rect x="180" y="25" width="10" height="6" rx="1" /><rect x="180" y="38" width="10" height="6" rx="1" /><rect x="180" y="51" width="10" height="6" rx="1" /><rect x="180" y="64" width="10" height="6" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="start">
      <text x="195" y="30">8</text><text x="195" y="43">7</text><text x="195" y="56">6</text><text x="195" y="69">5</text>
    </g>
    <text x="120" y="55" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if dip4.includes(activeId)}
  <!-- DIP-4 -->
  <svg viewBox="0 0 200 80" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="10" width="80" height="60" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <path d="M60 35 A5 5 0 0 1 60 45" stroke="var(--text-muted)" stroke-width="2" fill="none"/>
    <g fill="var(--text-secondary)"><rect x="50" y="20" width="10" height="6" rx="1" /><rect x="50" y="45" width="10" height="6" rx="1" /></g>
    <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="end"><text x="45" y="25">1</text><text x="45" y="50">2</text></g>
    <g fill="var(--text-secondary)"><rect x="140" y="20" width="10" height="6" rx="1" /><rect x="140" y="45" width="10" height="6" rx="1" /></g>
    <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="start"><text x="155" y="25">4</text><text x="155" y="50">3</text></g>
    <text x="100" y="45" fill="var(--text-primary)" font-size="12" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if dip6.includes(activeId)}
  <!-- DIP-6 -->
  <svg viewBox="0 0 220 100" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="10" width="100" height="80" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <path d="M60 45 A5 5 0 0 1 60 55" stroke="var(--text-muted)" stroke-width="2" fill="none"/>
    <g fill="var(--text-secondary)"><rect x="50" y="20" width="10" height="6" rx="1" /><rect x="50" y="40" width="10" height="6" rx="1" /><rect x="50" y="60" width="10" height="6" rx="1" /></g>
    <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="end"><text x="45" y="25">1</text><text x="45" y="45">2</text><text x="45" y="65">3</text></g>
    <g fill="var(--text-secondary)"><rect x="160" y="20" width="10" height="6" rx="1" /><rect x="160" y="40" width="10" height="6" rx="1" /><rect x="160" y="60" width="10" height="6" rx="1" /></g>
    <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="start"><text x="175" y="25">6</text><text x="175" y="45">5</text><text x="175" y="65">4</text></g>
    <text x="110" y="55" fill="var(--text-primary)" font-size="12" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if dip14.includes(activeId)}
  <!-- DIP-14 -->
  <svg viewBox="0 0 260 140" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="10" width="140" height="120" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <path d="M60 65 A5 5 0 0 1 60 75" stroke="var(--text-muted)" stroke-width="2" fill="none"/>
    <g fill="var(--text-secondary)"><rect x="50" y="15" width="10" height="6" rx="1" /><rect x="50" y="30" width="10" height="6" rx="1" /><rect x="50" y="45" width="10" height="6" rx="1" /><rect x="50" y="60" width="10" height="6" rx="1" /><rect x="50" y="75" width="10" height="6" rx="1" /><rect x="50" y="90" width="10" height="6" rx="1" /><rect x="50" y="105" width="10" height="6" rx="1" /></g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="end"><text x="45" y="20">1</text><text x="45" y="35">2</text><text x="45" y="50">3</text><text x="45" y="65">4</text><text x="45" y="80">5</text><text x="45" y="95">6</text><text x="45" y="110">7</text></g>
    <g fill="var(--text-secondary)"><rect x="200" y="15" width="10" height="6" rx="1" /><rect x="200" y="30" width="10" height="6" rx="1" /><rect x="200" y="45" width="10" height="6" rx="1" /><rect x="200" y="60" width="10" height="6" rx="1" /><rect x="200" y="75" width="10" height="6" rx="1" /><rect x="200" y="90" width="10" height="6" rx="1" /><rect x="200" y="105" width="10" height="6" rx="1" /></g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="start"><text x="215" y="20">14</text><text x="215" y="35">13</text><text x="215" y="50">12</text><text x="215" y="65">11</text><text x="215" y="80">10</text><text x="215" y="95">9</text><text x="215" y="110">8</text></g>
    <text x="130" y="70" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if dip16.includes(activeId)}
  <!-- DIP-16 -->
  <svg viewBox="0 0 280 120" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="10" width="160" height="100" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <path d="M60 55 A5 5 0 0 1 60 65" stroke="var(--text-muted)" stroke-width="2" fill="none"/>
    <g fill="var(--text-secondary)"><rect x="50" y="15" width="10" height="5" rx="1" /><rect x="50" y="27" width="10" height="5" rx="1" /><rect x="50" y="39" width="10" height="5" rx="1" /><rect x="50" y="51" width="10" height="5" rx="1" /><rect x="50" y="63" width="10" height="5" rx="1" /><rect x="50" y="75" width="10" height="5" rx="1" /><rect x="50" y="87" width="10" height="5" rx="1" /><rect x="50" y="99" width="10" height="5" rx="1" /></g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="end"><text x="45" y="20">1</text><text x="45" y="32">2</text><text x="45" y="44">3</text><text x="45" y="56">4</text><text x="45" y="68">5</text><text x="45" y="80">6</text><text x="45" y="92">7</text><text x="45" y="104">8</text></g>
    <g fill="var(--text-secondary)"><rect x="220" y="15" width="10" height="5" rx="1" /><rect x="220" y="27" width="10" height="5" rx="1" /><rect x="220" y="39" width="10" height="5" rx="1" /><rect x="220" y="51" width="10" height="5" rx="1" /><rect x="220" y="63" width="10" height="5" rx="1" /><rect x="220" y="75" width="10" height="5" rx="1" /><rect x="220" y="87" width="10" height="5" rx="1" /><rect x="220" y="99" width="10" height="5" rx="1" /></g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="start"><text x="235" y="20">16</text><text x="235" y="32">15</text><text x="235" y="44">14</text><text x="235" y="56">13</text><text x="235" y="68">12</text><text x="235" y="80">11</text><text x="235" y="92">10</text><text x="235" y="104">9</text></g>
    <text x="140" y="65" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if dip18.includes(activeId)}
  <!-- DIP-18 -->
  <svg viewBox="0 0 300 140" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="10" width="180" height="120" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <path d="M60 65 A5 5 0 0 1 60 75" stroke="var(--text-muted)" stroke-width="2" fill="none"/>
    <g fill="var(--text-secondary)"><rect x="50" y="15" width="10" height="5" rx="1" /><rect x="50" y="27" width="10" height="5" rx="1" /><rect x="50" y="39" width="10" height="5" rx="1" /><rect x="50" y="51" width="10" height="5" rx="1" /><rect x="50" y="63" width="10" height="5" rx="1" /><rect x="50" y="75" width="10" height="5" rx="1" /><rect x="50" y="87" width="10" height="5" rx="1" /><rect x="50" y="99" width="10" height="5" rx="1" /><rect x="50" y="111" width="10" height="5" rx="1" /></g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="end"><text x="45" y="20">1</text><text x="45" y="32">2</text><text x="45" y="44">3</text><text x="45" y="56">4</text><text x="45" y="68">5</text><text x="45" y="80">6</text><text x="45" y="92">7</text><text x="45" y="104">8</text><text x="45" y="116">9</text></g>
    <g fill="var(--text-secondary)"><rect x="240" y="15" width="10" height="5" rx="1" /><rect x="240" y="27" width="10" height="5" rx="1" /><rect x="240" y="39" width="10" height="5" rx="1" /><rect x="240" y="51" width="10" height="5" rx="1" /><rect x="240" y="63" width="10" height="5" rx="1" /><rect x="240" y="75" width="10" height="5" rx="1" /><rect x="240" y="87" width="10" height="5" rx="1" /><rect x="240" y="99" width="10" height="5" rx="1" /><rect x="240" y="111" width="10" height="5" rx="1" /></g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="start"><text x="255" y="20">18</text><text x="255" y="32">17</text><text x="255" y="44">16</text><text x="255" y="56">15</text><text x="255" y="68">14</text><text x="255" y="80">13</text><text x="255" y="92">12</text><text x="255" y="104">11</text><text x="255" y="116">10</text></g>
    <text x="150" y="70" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if dip20.includes(activeId)}
  <!-- DIP-20 -->
  <svg viewBox="0 0 300 140" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="10" width="180" height="120" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <path d="M60 65 A5 5 0 0 1 60 75" stroke="var(--text-muted)" stroke-width="2" fill="none"/>
    <g fill="var(--text-secondary)"><rect x="50" y="15" width="10" height="5" rx="1" /><rect x="50" y="27" width="10" height="5" rx="1" /><rect x="50" y="39" width="10" height="5" rx="1" /><rect x="50" y="51" width="10" height="5" rx="1" /><rect x="50" y="63" width="10" height="5" rx="1" /><rect x="50" y="75" width="10" height="5" rx="1" /><rect x="50" y="87" width="10" height="5" rx="1" /><rect x="50" y="99" width="10" height="5" rx="1" /><rect x="50" y="111" width="10" height="5" rx="1" /><rect x="50" y="123" width="10" height="5" rx="1" /></g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="end"><text x="45" y="20">1</text><text x="45" y="32">2</text><text x="45" y="44">3</text><text x="45" y="56">4</text><text x="45" y="68">5</text><text x="45" y="80">6</text><text x="45" y="92">7</text><text x="45" y="104">8</text><text x="45" y="116">9</text><text x="45" y="128">10</text></g>
    <g fill="var(--text-secondary)"><rect x="240" y="15" width="10" height="5" rx="1" /><rect x="240" y="27" width="10" height="5" rx="1" /><rect x="240" y="39" width="10" height="5" rx="1" /><rect x="240" y="51" width="10" height="5" rx="1" /><rect x="240" y="63" width="10" height="5" rx="1" /><rect x="240" y="75" width="10" height="5" rx="1" /><rect x="240" y="87" width="10" height="5" rx="1" /><rect x="240" y="99" width="10" height="5" rx="1" /><rect x="240" y="111" width="10" height="5" rx="1" /><rect x="240" y="123" width="10" height="5" rx="1" /></g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="start"><text x="255" y="20">20</text><text x="255" y="32">19</text><text x="255" y="44">18</text><text x="255" y="56">17</text><text x="255" y="68">16</text><text x="255" y="80">15</text><text x="255" y="92">14</text><text x="255" y="104">13</text><text x="255" y="116">12</text><text x="255" y="128">11</text></g>
    <text x="150" y="70" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if dip24.includes(activeId)}
  <!-- DIP-24 -->
  <svg viewBox="0 0 340 160" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="10" width="220" height="140" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <path d="M60 75 A5 5 0 0 1 60 85" stroke="var(--text-muted)" stroke-width="2" fill="none"/>
    <g fill="var(--text-secondary)"><rect x="50" y="15" width="10" height="5" rx="1" /><rect x="50" y="27" width="10" height="5" rx="1" /><rect x="50" y="39" width="10" height="5" rx="1" /><rect x="50" y="51" width="10" height="5" rx="1" /><rect x="50" y="63" width="10" height="5" rx="1" /><rect x="50" y="75" width="10" height="5" rx="1" /><rect x="50" y="87" width="10" height="5" rx="1" /><rect x="50" y="99" width="10" height="5" rx="1" /><rect x="50" y="111" width="10" height="5" rx="1" /><rect x="50" y="123" width="10" height="5" rx="1" /><rect x="50" y="135" width="10" height="5" rx="1" /><rect x="50" y="147" width="10" height="5" rx="1" /></g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="end"><text x="45" y="20">1</text><text x="45" y="32">2</text><text x="45" y="44">3</text><text x="45" y="56">4</text><text x="45" y="68">5</text><text x="45" y="80">6</text><text x="45" y="92">7</text><text x="45" y="104">8</text><text x="45" y="116">9</text><text x="45" y="128">10</text><text x="45" y="140">11</text><text x="45" y="152">12</text></g>
    <g fill="var(--text-secondary)"><rect x="280" y="15" width="10" height="5" rx="1" /><rect x="280" y="27" width="10" height="5" rx="1" /><rect x="280" y="39" width="10" height="5" rx="1" /><rect x="280" y="51" width="10" height="5" rx="1" /><rect x="280" y="63" width="10" height="5" rx="1" /><rect x="280" y="75" width="10" height="5" rx="1" /><rect x="280" y="87" width="10" height="5" rx="1" /><rect x="280" y="99" width="10" height="5" rx="1" /><rect x="280" y="111" width="10" height="5" rx="1" /><rect x="280" y="123" width="10" height="5" rx="1" /><rect x="280" y="135" width="10" height="5" rx="1" /><rect x="280" y="147" width="10" height="5" rx="1" /></g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="start"><text x="295" y="20">24</text><text x="295" y="32">23</text><text x="295" y="44">22</text><text x="295" y="56">21</text><text x="295" y="68">20</text><text x="295" y="80">19</text><text x="295" y="92">18</text><text x="295" y="104">17</text><text x="295" y="116">16</text><text x="295" y="128">15</text><text x="295" y="140">14</text><text x="295" y="152">13</text></g>
    <text x="170" y="85" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if dip28.includes(activeId)}
  <!-- DIP-28 -->
  <svg viewBox="0 0 360 180" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="10" width="240" height="160" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <path d="M60 85 A5 5 0 0 1 60 95" stroke="var(--text-muted)" stroke-width="2" fill="none"/>
    <g fill="var(--text-secondary)"><rect x="50" y="15" width="10" height="4" rx="1" /><rect x="50" y="25" width="10" height="4" rx="1" /><rect x="50" y="35" width="10" height="4" rx="1" /><rect x="50" y="45" width="10" height="4" rx="1" /><rect x="50" y="55" width="10" height="4" rx="1" /><rect x="50" y="65" width="10" height="4" rx="1" /><rect x="50" y="75" width="10" height="4" rx="1" /><rect x="50" y="85" width="10" height="4" rx="1" /><rect x="50" y="95" width="10" height="4" rx="1" /><rect x="50" y="105" width="10" height="4" rx="1" /><rect x="50" y="115" width="10" height="4" rx="1" /><rect x="50" y="125" width="10" height="4" rx="1" /><rect x="50" y="135" width="10" height="4" rx="1" /><rect x="50" y="145" width="10" height="4" rx="1" /></g>
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="end"><text x="45" y="19">1</text><text x="45" y="29">2</text><text x="45" y="39">3</text><text x="45" y="49">4</text><text x="45" y="59">5</text><text x="45" y="69">6</text><text x="45" y="79">7</text><text x="45" y="89">8</text><text x="45" y="99">9</text><text x="45" y="109">10</text><text x="45" y="119">11</text><text x="45" y="129">12</text><text x="45" y="139">13</text><text x="45" y="149">14</text></g>
    <g fill="var(--text-secondary)"><rect x="300" y="15" width="10" height="4" rx="1" /><rect x="300" y="25" width="10" height="4" rx="1" /><rect x="300" y="35" width="10" height="4" rx="1" /><rect x="300" y="45" width="10" height="4" rx="1" /><rect x="300" y="55" width="10" height="4" rx="1" /><rect x="300" y="65" width="10" height="4" rx="1" /><rect x="300" y="75" width="10" height="4" rx="1" /><rect x="300" y="85" width="10" height="4" rx="1" /><rect x="300" y="95" width="10" height="4" rx="1" /><rect x="300" y="105" width="10" height="4" rx="1" /><rect x="300" y="115" width="10" height="4" rx="1" /><rect x="300" y="125" width="10" height="4" rx="1" /><rect x="300" y="135" width="10" height="4" rx="1" /><rect x="300" y="145" width="10" height="4" rx="1" /></g>
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="start"><text x="315" y="19">28</text><text x="315" y="29">27</text><text x="315" y="39">26</text><text x="315" y="49">25</text><text x="315" y="59">24</text><text x="315" y="69">23</text><text x="315" y="79">22</text><text x="315" y="89">21</text><text x="315" y="99">20</text><text x="315" y="109">19</text><text x="315" y="119">18</text><text x="315" y="129">17</text><text x="315" y="139">16</text><text x="315" y="149">15</text></g>
    <text x="180" y="85" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if dip40.includes(activeId)}
  <!-- DIP-40 -->
  <svg viewBox="0 0 440 220" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="10" width="320" height="200" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <path d="M60 105 A5 5 0 0 1 60 115" stroke="var(--text-muted)" stroke-width="2" fill="none"/>
    <g fill="var(--text-secondary)"><rect x="50" y="15" width="10" height="4" rx="1" /><rect x="50" y="25" width="10" height="4" rx="1" /><rect x="50" y="35" width="10" height="4" rx="1" /><rect x="50" y="45" width="10" height="4" rx="1" /><rect x="50" y="55" width="10" height="4" rx="1" /><rect x="50" y="65" width="10" height="4" rx="1" /><rect x="50" y="75" width="10" height="4" rx="1" /><rect x="50" y="85" width="10" height="4" rx="1" /><rect x="50" y="95" width="10" height="4" rx="1" /><rect x="50" y="105" width="10" height="4" rx="1" /><rect x="50" y="115" width="10" height="4" rx="1" /><rect x="50" y="125" width="10" height="4" rx="1" /><rect x="50" y="135" width="10" height="4" rx="1" /><rect x="50" y="145" width="10" height="4" rx="1" /><rect x="50" y="155" width="10" height="4" rx="1" /><rect x="50" y="165" width="10" height="4" rx="1" /><rect x="50" y="175" width="10" height="4" rx="1" /><rect x="50" y="185" width="10" height="4" rx="1" /><rect x="50" y="195" width="10" height="4" rx="1" /><rect x="50" y="205" width="10" height="4" rx="1" /></g>
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="end"><text x="45" y="19">1</text><text x="45" y="29">2</text><text x="45" y="39">3</text><text x="45" y="49">4</text><text x="45" y="59">5</text><text x="45" y="69">6</text><text x="45" y="79">7</text><text x="45" y="89">8</text><text x="45" y="99">9</text><text x="45" y="109">10</text><text x="45" y="119">11</text><text x="45" y="129">12</text><text x="45" y="139">13</text><text x="45" y="149">14</text><text x="45" y="159">15</text><text x="45" y="169">16</text><text x="45" y="179">17</text><text x="45" y="189">18</text><text x="45" y="199">19</text><text x="45" y="209">20</text></g>
    <g fill="var(--text-secondary)"><rect x="380" y="15" width="10" height="4" rx="1" /><rect x="380" y="25" width="10" height="4" rx="1" /><rect x="380" y="35" width="10" height="4" rx="1" /><rect x="380" y="45" width="10" height="4" rx="1" /><rect x="380" y="55" width="10" height="4" rx="1" /><rect x="380" y="65" width="10" height="4" rx="1" /><rect x="380" y="75" width="10" height="4" rx="1" /><rect x="380" y="85" width="10" height="4" rx="1" /><rect x="380" y="95" width="10" height="4" rx="1" /><rect x="380" y="105" width="10" height="4" rx="1" /><rect x="380" y="115" width="10" height="4" rx="1" /><rect x="380" y="125" width="10" height="4" rx="1" /><rect x="380" y="135" width="10" height="4" rx="1" /><rect x="380" y="145" width="10" height="4" rx="1" /><rect x="380" y="155" width="10" height="4" rx="1" /><rect x="380" y="165" width="10" height="4" rx="1" /><rect x="380" y="175" width="10" height="4" rx="1" /><rect x="380" y="185" width="10" height="4" rx="1" /><rect x="380" y="195" width="10" height="4" rx="1" /><rect x="380" y="205" width="10" height="4" rx="1" /></g>
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="start"><text x="395" y="19">40</text><text x="395" y="29">39</text><text x="395" y="39">38</text><text x="395" y="49">37</text><text x="395" y="59">36</text><text x="395" y="69">35</text><text x="395" y="79">34</text><text x="395" y="89">33</text><text x="395" y="99">32</text><text x="395" y="109">31</text><text x="395" y="119">30</text><text x="395" y="129">29</text><text x="395" y="139">28</text><text x="395" y="149">27</text><text x="395" y="159">26</text><text x="395" y="169">25</text><text x="395" y="179">24</text><text x="395" y="189">23</text><text x="395" y="199">22</text><text x="395" y="209">21</text></g>
    <text x="220" y="105" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if to220_3.includes(activeId)}
  <!-- TO-220-3 -->
  <svg viewBox="0 0 260 200" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="20" width="140" height="100" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <circle cx="130" cy="40" r="8" fill="var(--bg-base)" stroke="var(--text-muted)" stroke-width="1.5"/>
    <g fill="var(--text-secondary)"><rect x="90" y="120" width="8" height="40" rx="1" /><rect x="126" y="120" width="8" height="40" rx="1" /><rect x="162" y="120" width="8" height="40" rx="1" /></g>
    <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="middle"><text x="94" y="175">1</text><text x="130" y="175">2</text><text x="166" y="175">3</text></g>
    <text x="130" y="80" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if to220_5.includes(activeId)}
  <!-- TO-220-5 -->
  <svg viewBox="0 0 260 200" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="50" y="20" width="160" height="100" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <circle cx="130" cy="40" r="8" fill="var(--bg-base)" stroke="var(--text-muted)" stroke-width="1.5"/>
    <g fill="var(--text-secondary)"><rect x="70" y="120" width="8" height="40" rx="1" /><rect x="100" y="120" width="8" height="40" rx="1" /><rect x="130" y="120" width="8" height="40" rx="1" /><rect x="160" y="120" width="8" height="40" rx="1" /><rect x="190" y="120" width="8" height="40" rx="1" /></g>
    <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="middle"><text x="74" y="175">1</text><text x="104" y="175">2</text><text x="134" y="175">3</text><text x="164" y="175">4</text><text x="194" y="175">5</text></g>
    <text x="130" y="80" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if sot223.includes(activeId)}
  <!-- SOT-223 -->
  <svg viewBox="0 0 240 140" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="70" y="30" width="100" height="70" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <rect x="110" y="20" width="20" height="10" rx="1" fill="var(--text-secondary)"/>
    <g fill="var(--text-secondary)"><rect x="85" y="100" width="10" height="20" rx="1" /><rect x="115" y="100" width="10" height="20" rx="1" /><rect x="145" y="100" width="10" height="20" rx="1" /></g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle"><text x="90" y="135">1</text><text x="120" y="135">2</text><text x="150" y="135">3</text></g>
    <text x="120" y="65" fill="var(--text-primary)" font-size="12" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if sot23.includes(activeId)}
  <!-- SOT-23-6 -->
  <svg viewBox="0 0 200 140" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="30" width="80" height="80" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <g fill="var(--text-secondary)"><rect x="50" y="45" width="10" height="8" rx="1" /><rect x="50" y="65" width="10" height="8" rx="1" /><rect x="50" y="85" width="10" height="8" rx="1" /><rect x="140" y="45" width="10" height="8" rx="1" /><rect x="140" y="65" width="10" height="8" rx="1" /><rect x="140" y="85" width="10" height="8" rx="1" /></g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="end"><text x="46" y="51">1</text><text x="46" y="71">2</text><text x="46" y="91">3</text></g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="start"><text x="154" y="51">6</text><text x="154" y="71">5</text><text x="154" y="91">4</text></g>
    <text x="100" y="75" fill="var(--text-primary)" font-size="12" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if sot89.includes(activeId)}
  <!-- SOT-89-5 -->
  <svg viewBox="0 0 200 140" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="30" width="80" height="80" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <g fill="var(--text-secondary)"><rect x="50" y="45" width="10" height="10" rx="1" /><rect x="50" y="85" width="10" height="10" rx="1" /></g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="end"><text x="46" y="52">1</text><text x="46" y="92">2</text></g>
    <rect x="140" y="55" width="10" height="30" rx="1" fill="var(--text-secondary)"/>
    <text x="154" y="75" fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="start">3</text>
    <text x="100" y="75" fill="var(--text-primary)" font-size="12" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if to92.includes(activeId)}
  <!-- TO-92 -->
  <svg viewBox="0 0 200 160" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path d="M60 50 L60 100 Q60 130 100 130 Q140 130 140 100 L140 50 Z" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <line x1="60" y1="50" x2="140" y2="50" stroke="var(--text-muted)" stroke-width="2"/>
    <g fill="var(--text-secondary)"><rect x="75" y="130" width="6" height="25" rx="1" /><rect x="97" y="130" width="6" height="25" rx="1" /><rect x="119" y="130" width="6" height="25" rx="1" /></g>
    <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="middle"><text x="78" y="160">1</text><text x="100" y="160">2</text><text x="122" y="160">3</text></g>
    <text x="100" y="95" fill="var(--text-primary)" font-size="12" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if mw9.includes(activeId)}
  <!-- Multiwatt-9 -->
  <svg viewBox="0 0 320 140" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="20" width="200" height="80" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <g fill="var(--text-secondary)"><rect x="70" y="100" width="6" height="20" rx="1" /><rect x="90" y="100" width="6" height="20" rx="1" /><rect x="110" y="100" width="6" height="20" rx="1" /><rect x="130" y="100" width="6" height="20" rx="1" /><rect x="150" y="100" width="6" height="20" rx="1" /><rect x="170" y="100" width="6" height="20" rx="1" /><rect x="190" y="100" width="6" height="20" rx="1" /><rect x="210" y="100" width="6" height="20" rx="1" /><rect x="230" y="100" width="6" height="20" rx="1" /></g>
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="middle"><text x="73" y="130">1</text><text x="93" y="130">2</text><text x="113" y="130">3</text><text x="133" y="130">4</text><text x="153" y="130">5</text><text x="173" y="130">6</text><text x="193" y="130">7</text><text x="213" y="130">8</text><text x="233" y="130">9</text></g>
    <text x="160" y="65" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if mw11.includes(activeId)}
  <!-- Multiwatt-11 -->
  <svg viewBox="0 0 340 140" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="20" width="220" height="80" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <g fill="var(--text-secondary)"><rect x="70" y="100" width="6" height="20" rx="1" /><rect x="90" y="100" width="6" height="20" rx="1" /><rect x="110" y="100" width="6" height="20" rx="1" /><rect x="130" y="100" width="6" height="20" rx="1" /><rect x="150" y="100" width="6" height="20" rx="1" /><rect x="170" y="100" width="6" height="20" rx="1" /><rect x="190" y="100" width="6" height="20" rx="1" /><rect x="210" y="100" width="6" height="20" rx="1" /><rect x="230" y="100" width="6" height="20" rx="1" /><rect x="250" y="100" width="6" height="20" rx="1" /><rect x="270" y="100" width="6" height="20" rx="1" /></g>
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="middle"><text x="73" y="130">1</text><text x="93" y="130">2</text><text x="113" y="130">3</text><text x="133" y="130">4</text><text x="153" y="130">5</text><text x="173" y="130">6</text><text x="193" y="130">7</text><text x="213" y="130">8</text><text x="233" y="130">9</text><text x="253" y="130">10</text><text x="273" y="130">11</text></g>
    <text x="170" y="65" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if mw13.includes(activeId)}
  <!-- Multiwatt-13 -->
  <svg viewBox="0 0 360 140" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="20" width="240" height="80" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <g fill="var(--text-secondary)"><rect x="70" y="100" width="6" height="20" rx="1" /><rect x="90" y="100" width="6" height="20" rx="1" /><rect x="110" y="100" width="6" height="20" rx="1" /><rect x="130" y="100" width="6" height="20" rx="1" /><rect x="150" y="100" width="6" height="20" rx="1" /><rect x="170" y="100" width="6" height="20" rx="1" /><rect x="190" y="100" width="6" height="20" rx="1" /><rect x="210" y="100" width="6" height="20" rx="1" /><rect x="230" y="100" width="6" height="20" rx="1" /><rect x="250" y="100" width="6" height="20" rx="1" /><rect x="270" y="100" width="6" height="20" rx="1" /><rect x="290" y="100" width="6" height="20" rx="1" /><rect x="310" y="100" width="6" height="20" rx="1" /></g>
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="middle"><text x="73" y="130">1</text><text x="93" y="130">2</text><text x="113" y="130">3</text><text x="133" y="130">4</text><text x="153" y="130">5</text><text x="173" y="130">6</text><text x="193" y="130">7</text><text x="213" y="130">8</text><text x="233" y="130">9</text><text x="253" y="130">10</text><text x="273" y="130">11</text><text x="293" y="130">12</text><text x="313" y="130">13</text></g>
    <text x="180" y="65" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if mw15.includes(activeId)}
  <!-- Multiwatt-15 -->
  <svg viewBox="0 0 320 140" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="40" y="20" width="240" height="80" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <g fill="var(--text-secondary)">
      <rect x="50" y="100" width="6" height="20" rx="1" />
      <rect x="66" y="100" width="6" height="20" rx="1" />
      <rect x="82" y="100" width="6" height="20" rx="1" />
      <rect x="98" y="100" width="6" height="20" rx="1" />
      <rect x="114" y="100" width="6" height="20" rx="1" />
      <rect x="130" y="100" width="6" height="20" rx="1" />
      <rect x="146" y="100" width="6" height="20" rx="1" />
      <rect x="162" y="100" width="6" height="20" rx="1" />
      <rect x="178" y="100" width="6" height="20" rx="1" />
      <rect x="194" y="100" width="6" height="20" rx="1" />
      <rect x="210" y="100" width="6" height="20" rx="1" />
      <rect x="226" y="100" width="6" height="20" rx="1" />
      <rect x="242" y="100" width="6" height="20" rx="1" />
      <rect x="258" y="100" width="6" height="20" rx="1" />
      <rect x="274" y="100" width="6" height="20" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="middle">
      <text x="53" y="130">1</text><text x="69" y="130">2</text><text x="85" y="130">3</text><text x="101" y="130">4</text><text x="117" y="130">5</text><text x="133" y="130">6</text><text x="149" y="130">7</text><text x="165" y="130">8</text><text x="181" y="130">9</text><text x="197" y="130">10</text><text x="213" y="130">11</text><text x="229" y="130">12</text><text x="245" y="130">13</text><text x="261" y="130">14</text><text x="277" y="130">15</text>
    </g>
    <text x="160" y="65" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if ['25q64', 'cp2102', 'hv9910', 'bp3102'].includes(activeId)}
  <!-- SOP-8 -->
  <svg viewBox="0 0 340 180" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="80" y="40" width="180" height="100" rx="2" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="1.5"/>
    <rect x="100" y="50" width="140" height="80" rx="1" fill="var(--bg-elevated-2)" stroke="var(--text-muted)" stroke-width="1"/>
    <g fill="var(--text-secondary)">
      <rect x="68" y="50" width="12" height="8" rx="1" /><rect x="68" y="70" width="12" height="8" rx="1" /><rect x="68" y="90" width="12" height="8" rx="1" /><rect x="68" y="110" width="12" height="8" rx="1" />
      <rect x="260" y="50" width="12" height="8" rx="1" /><rect x="260" y="70" width="12" height="8" rx="1" /><rect x="260" y="90" width="12" height="8" rx="1" /><rect x="260" y="110" width="12" height="8" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="end"><text x="64" y="56">1</text><text x="64" y="76">2</text><text x="64" y="96">3</text><text x="64" y="116">4</text></g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="start"><text x="276" y="56">8</text><text x="276" y="76">7</text><text x="276" y="96">6</text><text x="276" y="116">5</text></g>
    <text x="170" y="95" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if activeId === 'lm3409'}
  <!-- SOP-10 -->
  <svg viewBox="0 0 340 200" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="80" y="40" width="180" height="120" rx="2" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="1.5"/>
    <rect x="100" y="50" width="140" height="100" rx="1" fill="var(--bg-elevated-2)" stroke="var(--text-muted)" stroke-width="1"/>
    <g fill="var(--text-secondary)">
      <rect x="68" y="45" width="12" height="8" rx="1" /><rect x="68" y="65" width="12" height="8" rx="1" /><rect x="68" y="85" width="12" height="8" rx="1" /><rect x="68" y="105" width="12" height="8" rx="1" /><rect x="68" y="125" width="12" height="8" rx="1" />
      <rect x="260" y="45" width="12" height="8" rx="1" /><rect x="260" y="65" width="12" height="8" rx="1" /><rect x="260" y="85" width="12" height="8" rx="1" /><rect x="260" y="105" width="12" height="8" rx="1" /><rect x="260" y="125" width="12" height="8" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="end"><text x="64" y="51">1</text><text x="64" y="71">2</text><text x="64" y="91">3</text><text x="64" y="111">4</text><text x="64" y="131">5</text></g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="start"><text x="276" y="51">10</text><text x="276" y="71">9</text><text x="276" y="91">8</text><text x="276" y="111">7</text><text x="276" y="131">6</text></g>
    <text x="170" y="100" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if activeId === 'stm32f103'}
  <!-- LQFP-48 -->
  <svg viewBox="0 0 400 400" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="80" y="80" width="240" height="240" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <path d="M80 80 L80 100 L100 80 Z" fill="var(--text-muted)"/>
    
    <g fill="var(--text-secondary)">
      <rect x="70" y="98" width="10" height="4" rx="1" /><rect x="70" y="116" width="10" height="4" rx="1" /><rect x="70" y="134" width="10" height="4" rx="1" /><rect x="70" y="152" width="10" height="4" rx="1" /><rect x="70" y="170" width="10" height="4" rx="1" /><rect x="70" y="188" width="10" height="4" rx="1" /><rect x="70" y="206" width="10" height="4" rx="1" /><rect x="70" y="224" width="10" height="4" rx="1" /><rect x="70" y="242" width="10" height="4" rx="1" /><rect x="70" y="260" width="10" height="4" rx="1" /><rect x="70" y="278" width="10" height="4" rx="1" /><rect x="70" y="296" width="10" height="4" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="end">
      <text x="66" y="102">1</text><text x="66" y="120">2</text><text x="66" y="138">3</text><text x="66" y="156">4</text><text x="66" y="174">5</text><text x="66" y="192">6</text><text x="66" y="210">7</text><text x="66" y="228">8</text><text x="66" y="246">9</text><text x="66" y="264">10</text><text x="66" y="282">11</text><text x="66" y="300">12</text>
    </g>

    <g fill="var(--text-secondary)">
      <rect x="98" y="320" width="4" height="10" rx="1" /><rect x="116" y="320" width="4" height="10" rx="1" /><rect x="134" y="320" width="4" height="10" rx="1" /><rect x="152" y="320" width="4" height="10" rx="1" /><rect x="170" y="320" width="4" height="10" rx="1" /><rect x="188" y="320" width="4" height="10" rx="1" /><rect x="206" y="320" width="4" height="10" rx="1" /><rect x="224" y="320" width="4" height="10" rx="1" /><rect x="242" y="320" width="4" height="10" rx="1" /><rect x="260" y="320" width="4" height="10" rx="1" /><rect x="278" y="320" width="4" height="10" rx="1" /><rect x="296" y="320" width="4" height="10" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="middle">
      <text x="100" y="340">13</text><text x="118" y="340">14</text><text x="136" y="340">15</text><text x="154" y="340">16</text><text x="172" y="340">17</text><text x="190" y="340">18</text><text x="208" y="340">19</text><text x="226" y="340">20</text><text x="244" y="340">21</text><text x="262" y="340">22</text><text x="280" y="340">23</text><text x="298" y="340">24</text>
    </g>

    <g fill="var(--text-secondary)">
      <rect x="320" y="296" width="10" height="4" rx="1" /><rect x="320" y="278" width="10" height="4" rx="1" /><rect x="320" y="260" width="10" height="4" rx="1" /><rect x="320" y="242" width="10" height="4" rx="1" /><rect x="320" y="224" width="10" height="4" rx="1" /><rect x="320" y="206" width="10" height="4" rx="1" /><rect x="320" y="188" width="10" height="4" rx="1" /><rect x="320" y="170" width="10" height="4" rx="1" /><rect x="320" y="152" width="10" height="4" rx="1" /><rect x="320" y="134" width="10" height="4" rx="1" /><rect x="320" y="116" width="10" height="4" rx="1" /><rect x="320" y="98" width="10" height="4" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="start">
      <text x="334" y="300">25</text><text x="334" y="282">26</text><text x="334" y="264">27</text><text x="334" y="246">28</text><text x="334" y="228">29</text><text x="334" y="210">30</text><text x="334" y="192">31</text><text x="334" y="174">32</text><text x="334" y="156">33</text><text x="334" y="138">34</text><text x="334" y="120">35</text><text x="334" y="102">36</text>
    </g>

    <g fill="var(--text-secondary)">
      <rect x="296" y="70" width="4" height="10" rx="1" /><rect x="278" y="70" width="4" height="10" rx="1" /><rect x="260" y="70" width="4" height="10" rx="1" /><rect x="242" y="70" width="4" height="10" rx="1" /><rect x="224" y="70" width="4" height="10" rx="1" /><rect x="206" y="70" width="4" height="10" rx="1" /><rect x="188" y="70" width="4" height="10" rx="1" /><rect x="170" y="70" width="4" height="10" rx="1" /><rect x="152" y="70" width="4" height="10" rx="1" /><rect x="134" y="70" width="4" height="10" rx="1" /><rect x="116" y="70" width="4" height="10" rx="1" /><rect x="98" y="70" width="4" height="10" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="middle">
      <text x="298" y="62">37</text><text x="280" y="62">38</text><text x="262" y="62">39</text><text x="244" y="62">40</text><text x="226" y="62">41</text><text x="208" y="62">42</text><text x="190" y="62">43</text><text x="172" y="62">44</text><text x="154" y="62">45</text><text x="136" y="62">46</text><text x="118" y="62">47</text><text x="100" y="62">48</text>
    </g>

    <text x="200" y="200" fill="var(--text-primary)" font-size="16" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if module4.includes(activeId)}
  <!-- Module 1x4 -->
  <svg viewBox="0 0 240 120" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="20" width="120" height="60" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <g fill="var(--text-secondary)">
      <rect x="75" y="10" width="8" height="10" rx="1" /><rect x="100" y="10" width="8" height="10" rx="1" /><rect x="125" y="10" width="8" height="10" rx="1" /><rect x="150" y="10" width="8" height="10" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
      <text x="79" y="8">1</text><text x="104" y="8">2</text><text x="129" y="8">3</text><text x="154" y="8">4</text>
    </g>
    <text x="120" y="55" fill="var(--text-primary)" font-size="12" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if module6.includes(activeId)}
  <!-- Module 2x3 -->
  <svg viewBox="0 0 240 140" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="30" width="120" height="80" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <g fill="var(--text-secondary)">
      <rect x="50" y="40" width="10" height="8" rx="1" /><rect x="50" y="60" width="10" height="8" rx="1" /><rect x="50" y="80" width="10" height="8" rx="1" />
      <rect x="180" y="40" width="10" height="8" rx="1" /><rect x="180" y="60" width="10" height="8" rx="1" /><rect x="180" y="80" width="10" height="8" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="end"><text x="46" y="46">1</text><text x="46" y="66">2</text><text x="46" y="86">3</text></g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="start"><text x="196" y="46">6</text><text x="196" y="66">5</text><text x="196" y="86">4</text></g>
    <text x="120" y="75" fill="var(--text-primary)" font-size="12" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if module8.includes(activeId)}
  <!-- Module 2x4 -->
  <svg viewBox="0 0 240 120" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="30" width="120" height="60" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <g fill="var(--text-secondary)">
      <rect x="75" y="20" width="8" height="10" rx="1" /><rect x="100" y="20" width="8" height="10" rx="1" /><rect x="125" y="20" width="8" height="10" rx="1" /><rect x="150" y="20" width="8" height="10" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
      <text x="79" y="15">1</text><text x="104" y="15">2</text><text x="129" y="15">3</text><text x="154" y="15">4</text>
    </g>
    <g fill="var(--text-secondary)">
      <rect x="75" y="90" width="8" height="10" rx="1" /><rect x="100" y="90" width="8" height="10" rx="1" /><rect x="125" y="90" width="8" height="10" rx="1" /><rect x="150" y="90" width="8" height="10" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
      <text x="79" y="112">8</text><text x="104" y="112">7</text><text x="129" y="112">6</text><text x="154" y="112">5</text>
    </g>
    <text x="120" y="65" fill="var(--text-primary)" font-size="12" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else if module22.includes(activeId)}
  <!-- Module 2x11 -->
  <svg viewBox="0 0 320 300" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="20" width="200" height="260" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <g fill="var(--text-secondary)">
      <rect x="50" y="30" width="10" height="8" rx="1" /><rect x="50" y="50" width="10" height="8" rx="1" /><rect x="50" y="70" width="10" height="8" rx="1" /><rect x="50" y="90" width="10" height="8" rx="1" /><rect x="50" y="110" width="10" height="8" rx="1" /><rect x="50" y="130" width="10" height="8" rx="1" /><rect x="50" y="150" width="10" height="8" rx="1" /><rect x="50" y="170" width="10" height="8" rx="1" /><rect x="50" y="190" width="10" height="8" rx="1" /><rect x="50" y="210" width="10" height="8" rx="1" /><rect x="50" y="230" width="10" height="8" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="end">
      <text x="45" y="36">1</text><text x="45" y="56">2</text><text x="45" y="76">3</text><text x="45" y="96">4</text><text x="45" y="116">5</text><text x="45" y="136">6</text><text x="45" y="156">7</text><text x="45" y="176">8</text><text x="45" y="196">9</text><text x="45" y="216">10</text><text x="45" y="236">11</text>
    </g>
    <g fill="var(--text-secondary)">
      <rect x="260" y="30" width="10" height="8" rx="1" /><rect x="260" y="50" width="10" height="8" rx="1" /><rect x="260" y="70" width="10" height="8" rx="1" /><rect x="260" y="90" width="10" height="8" rx="1" /><rect x="260" y="110" width="10" height="8" rx="1" /><rect x="260" y="130" width="10" height="8" rx="1" /><rect x="260" y="150" width="10" height="8" rx="1" /><rect x="260" y="170" width="10" height="8" rx="1" /><rect x="260" y="190" width="10" height="8" rx="1" /><rect x="260" y="210" width="10" height="8" rx="1" /><rect x="260" y="230" width="10" height="8" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="start">
      <text x="275" y="36">22</text><text x="275" y="56">21</text><text x="275" y="76">20</text><text x="275" y="96">19</text><text x="275" y="116">18</text><text x="275" y="136">17</text><text x="275" y="156">16</text><text x="275" y="176">15</text><text x="275" y="196">14</text><text x="275" y="216">13</text><text x="275" y="236">12</text>
    </g>
    <text x="160" y="150" fill="var(--text-primary)" font-size="12" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{name.toUpperCase()}</text>
  </svg>

{:else}
  <div class="scheme-placeholder">Нет схемы для {activeId}</div>
{/if}

<style>
  .scheme-placeholder { color: var(--text-muted); font-style: italic; }
</style>