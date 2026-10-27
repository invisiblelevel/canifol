<script>
  let { activeId = '' } = $props();
</script>

{#if ['lm358', 'ne555', 'lm386', 'mc34063', 'tda2822', 'ne5532', 'k140ud6', 'k140ud7', 'kr1006vi1', 'kr140ud1208'].includes(activeId)}
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
    <text x="120" y="55" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{activeId.toUpperCase()}</text>
  </svg>

{:else if ['lm7805', 'lm317', 'lm7812', 'irfz44n', 'kr142en5a', 'kr142en8b'].includes(activeId)}
  <!-- TO-220 (3 ножки) -->
  <svg viewBox="0 0 240 100" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="90" y="10" width="60" height="50" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <g fill="var(--text-secondary)">
      <rect x="100" y="60" width="6" height="20" rx="1" /><rect x="117" y="60" width="6" height="20" rx="1" /><rect x="134" y="60" width="6" height="20" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="middle">
      <text x="103" y="90">1</text><text x="120" y="90">2</text><text x="137" y="90">3</text>
    </g>
    <text x="120" y="40" fill="var(--text-primary)" font-size="12" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{activeId.toUpperCase()}</text>
  </svg>

{:else if activeId === 'tda2030'}
  <!-- TO-220 (5 ножек) -->
  <svg viewBox="0 0 240 100" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="80" y="10" width="80" height="50" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <g fill="var(--text-secondary)">
      <rect x="90" y="60" width="6" height="20" rx="1" /><rect x="102" y="60" width="6" height="20" rx="1" /><rect x="114" y="60" width="6" height="20" rx="1" /><rect x="126" y="60" width="6" height="20" rx="1" /><rect x="138" y="60" width="6" height="20" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
      <text x="93" y="90">1</text><text x="105" y="90">2</text><text x="117" y="90">3</text><text x="129" y="90">4</text><text x="141" y="90">5</text>
    </g>
    <text x="120" y="40" fill="var(--text-primary)" font-size="12" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{activeId.toUpperCase()}</text>
  </svg>

{:else if ['tl494', '74hc595', 'l293d', 'uln2003', 'k155ir1', 'k155ag3', 'k155kp1', 'k155ie7', 'k561ie10', 'ch340g', 'max232', 'ds3231', 'k561ie11'].includes(activeId)}
  <!-- DIP-16 -->
  <svg viewBox="0 0 280 120" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="10" width="160" height="100" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <path d="M60 55 A5 5 0 0 1 60 65" stroke="var(--text-muted)" stroke-width="2" fill="none"/>
    <g fill="var(--text-secondary)">
      <rect x="50" y="15" width="10" height="5" rx="1" /><rect x="50" y="27" width="10" height="5" rx="1" /><rect x="50" y="39" width="10" height="5" rx="1" /><rect x="50" y="51" width="10" height="5" rx="1" /><rect x="50" y="63" width="10" height="5" rx="1" /><rect x="50" y="75" width="10" height="5" rx="1" /><rect x="50" y="87" width="10" height="5" rx="1" /><rect x="50" y="99" width="10" height="5" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="end">
      <text x="45" y="20">1</text><text x="45" y="32">2</text><text x="45" y="44">3</text><text x="45" y="56">4</text><text x="45" y="68">5</text><text x="45" y="80">6</text><text x="45" y="92">7</text><text x="45" y="104">8</text>
    </g>
    <g fill="var(--text-secondary)">
      <rect x="220" y="15" width="10" height="5" rx="1" /><rect x="220" y="27" width="10" height="5" rx="1" /><rect x="220" y="39" width="10" height="5" rx="1" /><rect x="220" y="51" width="10" height="5" rx="1" /><rect x="220" y="63" width="10" height="5" rx="1" /><rect x="220" y="75" width="10" height="5" rx="1" /><rect x="220" y="87" width="10" height="5" rx="1" /><rect x="220" y="99" width="10" height="5" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="start">
      <text x="235" y="20">16</text><text x="235" y="32">15</text><text x="235" y="44">14</text><text x="235" y="56">13</text><text x="235" y="68">12</text><text x="235" y="80">11</text><text x="235" y="92">10</text><text x="235" y="104">9</text>
    </g>
    <text x="140" y="65" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{activeId.toUpperCase()}</text>
  </svg>

{:else if ['74hc14', 'k155la3', 'k155tm2', 'k561la7', 'k561tm2', 'kr1533la3', 'kr1533tm2', 'k561kt3'].includes(activeId)}
  <!-- DIP-14 -->
  <svg viewBox="0 0 260 140" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="10" width="140" height="120" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <path d="M60 65 A5 5 0 0 1 60 75" stroke="var(--text-muted)" stroke-width="2" fill="none"/>
    <g fill="var(--text-secondary)">
      <rect x="50" y="15" width="10" height="6" rx="1" /><rect x="50" y="30" width="10" height="6" rx="1" /><rect x="50" y="45" width="10" height="6" rx="1" /><rect x="50" y="60" width="10" height="6" rx="1" /><rect x="50" y="75" width="10" height="6" rx="1" /><rect x="50" y="90" width="10" height="6" rx="1" /><rect x="50" y="105" width="10" height="6" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="end">
      <text x="45" y="20">1</text><text x="45" y="35">2</text><text x="45" y="50">3</text><text x="45" y="65">4</text><text x="45" y="80">5</text><text x="45" y="95">6</text><text x="45" y="110">7</text>
    </g>
    <g fill="var(--text-secondary)">
      <rect x="200" y="15" width="10" height="6" rx="1" /><rect x="200" y="30" width="10" height="6" rx="1" /><rect x="200" y="45" width="10" height="6" rx="1" /><rect x="200" y="60" width="10" height="6" rx="1" /><rect x="200" y="75" width="10" height="6" rx="1" /><rect x="200" y="90" width="10" height="6" rx="1" /><rect x="200" y="105" width="10" height="6" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="start">
      <text x="215" y="20">14</text><text x="215" y="35">13</text><text x="215" y="50">12</text><text x="215" y="65">11</text><text x="215" y="80">10</text><text x="215" y="95">9</text><text x="215" y="110">8</text>
    </g>
    <text x="130" y="70" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{activeId.toUpperCase()}</text>
  </svg>

{:else if activeId === 'pc817'}
  <!-- DIP-4 -->
  <svg viewBox="0 0 200 80" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="10" width="80" height="60" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <path d="M60 35 A5 5 0 0 1 60 45" stroke="var(--text-muted)" stroke-width="2" fill="none"/>
    <g fill="var(--text-secondary)">
      <rect x="50" y="20" width="10" height="6" rx="1" /><rect x="50" y="45" width="10" height="6" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="end">
      <text x="45" y="25">1</text><text x="45" y="50">2</text>
    </g>
    <g fill="var(--text-secondary)">
      <rect x="140" y="20" width="10" height="6" rx="1" /><rect x="140" y="45" width="10" height="6" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="start">
      <text x="155" y="25">4</text><text x="155" y="50">3</text>
    </g>
    <text x="100" y="45" fill="var(--text-primary)" font-size="12" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{activeId.toUpperCase()}</text>
  </svg>

{:else if activeId === 'kr1533ir22'}
  <!-- DIP-20 -->
  <svg viewBox="0 0 300 140" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="10" width="180" height="120" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <path d="M60 65 A5 5 0 0 1 60 75" stroke="var(--text-muted)" stroke-width="2" fill="none"/>
    <g fill="var(--text-secondary)">
      <rect x="50" y="15" width="10" height="5" rx="1" /><rect x="50" y="27" width="10" height="5" rx="1" /><rect x="50" y="39" width="10" height="5" rx="1" /><rect x="50" y="51" width="10" height="5" rx="1" /><rect x="50" y="63" width="10" height="5" rx="1" /><rect x="50" y="75" width="10" height="5" rx="1" /><rect x="50" y="87" width="10" height="5" rx="1" /><rect x="50" y="99" width="10" height="5" rx="1" /><rect x="50" y="111" width="10" height="5" rx="1" /><rect x="50" y="123" width="10" height="5" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="end">
      <text x="45" y="20">1</text><text x="45" y="32">2</text><text x="45" y="44">3</text><text x="45" y="56">4</text><text x="45" y="68">5</text><text x="45" y="80">6</text><text x="45" y="92">7</text><text x="45" y="104">8</text><text x="45" y="116">9</text><text x="45" y="128">10</text>
    </g>
    <g fill="var(--text-secondary)">
      <rect x="240" y="15" width="10" height="5" rx="1" /><rect x="240" y="27" width="10" height="5" rx="1" /><rect x="240" y="39" width="10" height="5" rx="1" /><rect x="240" y="51" width="10" height="5" rx="1" /><rect x="240" y="63" width="10" height="5" rx="1" /><rect x="240" y="75" width="10" height="5" rx="1" /><rect x="240" y="87" width="10" height="5" rx="1" /><rect x="240" y="99" width="10" height="5" rx="1" /><rect x="240" y="111" width="10" height="5" rx="1" /><rect x="240" y="123" width="10" height="5" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="start">
      <text x="255" y="20">20</text><text x="255" y="32">19</text><text x="255" y="44">18</text><text x="255" y="56">17</text><text x="255" y="68">16</text><text x="255" y="80">15</text><text x="255" y="92">14</text><text x="255" y="104">13</text><text x="255" y="116">12</text><text x="255" y="128">11</text>
    </g>
    <text x="150" y="70" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{activeId.toUpperCase()}</text>
  </svg>

{:else if ['k155id3', 'kr580vi53'].includes(activeId)}
  <!-- DIP-24 -->
  <svg viewBox="0 0 340 160" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="10" width="220" height="140" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <path d="M60 75 A5 5 0 0 1 60 85" stroke="var(--text-muted)" stroke-width="2" fill="none"/>
    <g fill="var(--text-secondary)">
      <rect x="50" y="15" width="10" height="5" rx="1" /><rect x="50" y="27" width="10" height="5" rx="1" /><rect x="50" y="39" width="10" height="5" rx="1" /><rect x="50" y="51" width="10" height="5" rx="1" /><rect x="50" y="63" width="10" height="5" rx="1" /><rect x="50" y="75" width="10" height="5" rx="1" /><rect x="50" y="87" width="10" height="5" rx="1" /><rect x="50" y="99" width="10" height="5" rx="1" /><rect x="50" y="111" width="10" height="5" rx="1" /><rect x="50" y="123" width="10" height="5" rx="1" /><rect x="50" y="135" width="10" height="5" rx="1" /><rect x="50" y="147" width="10" height="5" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="end">
      <text x="45" y="20">1</text><text x="45" y="32">2</text><text x="45" y="44">3</text><text x="45" y="56">4</text><text x="45" y="68">5</text><text x="45" y="80">6</text><text x="45" y="92">7</text><text x="45" y="104">8</text><text x="45" y="116">9</text><text x="45" y="128">10</text><text x="45" y="140">11</text><text x="45" y="152">12</text>
    </g>
    <g fill="var(--text-secondary)">
      <rect x="280" y="15" width="10" height="5" rx="1" /><rect x="280" y="27" width="10" height="5" rx="1" /><rect x="280" y="39" width="10" height="5" rx="1" /><rect x="280" y="51" width="10" height="5" rx="1" /><rect x="280" y="63" width="10" height="5" rx="1" /><rect x="280" y="75" width="10" height="5" rx="1" /><rect x="280" y="87" width="10" height="5" rx="1" /><rect x="280" y="99" width="10" height="5" rx="1" /><rect x="280" y="111" width="10" height="5" rx="1" /><rect x="280" y="123" width="10" height="5" rx="1" /><rect x="280" y="135" width="10" height="5" rx="1" /><rect x="280" y="147" width="10" height="5" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="start">
      <text x="295" y="20">24</text><text x="295" y="32">23</text><text x="295" y="44">22</text><text x="295" y="56">21</text><text x="295" y="68">20</text><text x="295" y="80">19</text><text x="295" y="92">18</text><text x="295" y="104">17</text><text x="295" y="116">16</text><text x="295" y="128">15</text><text x="295" y="140">14</text><text x="295" y="152">13</text>
    </g>
    <text x="170" y="85" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{activeId.toUpperCase()}</text>
  </svg>

{:else if activeId === 'atmega328p'}
  <!-- DIP-28 -->
  <svg viewBox="0 0 360 180" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="10" width="240" height="160" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <path d="M60 85 A5 5 0 0 1 60 95" stroke="var(--text-muted)" stroke-width="2" fill="none"/>
    <g fill="var(--text-secondary)">
      <rect x="50" y="15" width="10" height="4" rx="1" /><rect x="50" y="25" width="10" height="4" rx="1" /><rect x="50" y="35" width="10" height="4" rx="1" /><rect x="50" y="45" width="10" height="4" rx="1" /><rect x="50" y="55" width="10" height="4" rx="1" /><rect x="50" y="65" width="10" height="4" rx="1" /><rect x="50" y="75" width="10" height="4" rx="1" /><rect x="50" y="85" width="10" height="4" rx="1" /><rect x="50" y="95" width="10" height="4" rx="1" /><rect x="50" y="105" width="10" height="4" rx="1" /><rect x="50" y="115" width="10" height="4" rx="1" /><rect x="50" y="125" width="10" height="4" rx="1" /><rect x="50" y="135" width="10" height="4" rx="1" /><rect x="50" y="145" width="10" height="4" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="end">
      <text x="45" y="19">1</text><text x="45" y="29">2</text><text x="45" y="39">3</text><text x="45" y="49">4</text><text x="45" y="59">5</text><text x="45" y="69">6</text><text x="45" y="79">7</text><text x="45" y="89">8</text><text x="45" y="99">9</text><text x="45" y="109">10</text><text x="45" y="119">11</text><text x="45" y="129">12</text><text x="45" y="139">13</text><text x="45" y="149">14</text>
    </g>
    <g fill="var(--text-secondary)">
      <rect x="300" y="15" width="10" height="4" rx="1" /><rect x="300" y="25" width="10" height="4" rx="1" /><rect x="300" y="35" width="10" height="4" rx="1" /><rect x="300" y="45" width="10" height="4" rx="1" /><rect x="300" y="55" width="10" height="4" rx="1" /><rect x="300" y="65" width="10" height="4" rx="1" /><rect x="300" y="75" width="10" height="4" rx="1" /><rect x="300" y="85" width="10" height="4" rx="1" /><rect x="300" y="95" width="10" height="4" rx="1" /><rect x="300" y="105" width="10" height="4" rx="1" /><rect x="300" y="115" width="10" height="4" rx="1" /><rect x="300" y="125" width="10" height="4" rx="1" /><rect x="300" y="135" width="10" height="4" rx="1" /><rect x="300" y="145" width="10" height="4" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="start">
      <text x="315" y="19">28</text><text x="315" y="29">27</text><text x="315" y="39">26</text><text x="315" y="49">25</text><text x="315" y="59">24</text><text x="315" y="69">23</text><text x="315" y="79">22</text><text x="315" y="89">21</text><text x="315" y="99">20</text><text x="315" y="109">19</text><text x="315" y="119">18</text><text x="315" y="129">17</text><text x="315" y="139">16</text><text x="315" y="149">15</text>
    </g>
    <text x="180" y="85" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{activeId.toUpperCase()}</text>
  </svg>

{:else if activeId === 'tda7294'}
  <!-- Multiwatt 15-pin -->
  <svg viewBox="0 0 320 140" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="80" y="10" width="160" height="80" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <g fill="var(--text-secondary)">
      <rect x="85" y="90" width="6" height="20" rx="1" /><rect x="95" y="90" width="6" height="20" rx="1" /><rect x="105" y="90" width="6" height="20" rx="1" /><rect x="115" y="90" width="6" height="20" rx="1" /><rect x="125" y="90" width="6" height="20" rx="1" /><rect x="135" y="90" width="6" height="20" rx="1" /><rect x="145" y="90" width="6" height="20" rx="1" /><rect x="155" y="90" width="6" height="20" rx="1" /><rect x="165" y="90" width="6" height="20" rx="1" /><rect x="175" y="90" width="6" height="20" rx="1" /><rect x="185" y="90" width="6" height="20" rx="1" /><rect x="195" y="90" width="6" height="20" rx="1" /><rect x="205" y="90" width="6" height="20" rx="1" /><rect x="215" y="90" width="6" height="20" rx="1" /><rect x="225" y="90" width="6" height="20" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="middle">
      <text x="88" y="120">1</text><text x="98" y="120">2</text><text x="108" y="120">3</text><text x="118" y="120">4</text><text x="128" y="120">5</text><text x="138" y="120">6</text><text x="148" y="120">7</text><text x="158" y="120">8</text><text x="168" y="120">9</text><text x="178" y="120">10</text><text x="188" y="120">11</text><text x="198" y="120">12</text><text x="208" y="120">13</text><text x="218" y="120">14</text><text x="228" y="120">15</text>
    </g>
    <text x="160" y="55" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{activeId.toUpperCase()}</text>
  </svg>

{:else if ['kr580vm80a', 'kr580vv55a'].includes(activeId)}
  <!-- DIP-40 -->
  <svg viewBox="0 0 440 220" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="60" y="10" width="320" height="200" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <path d="M60 105 A5 5 0 0 1 60 115" stroke="var(--text-muted)" stroke-width="2" fill="none"/>
    <g fill="var(--text-secondary)">
      <rect x="50" y="15" width="10" height="4" rx="1" /><rect x="50" y="25" width="10" height="4" rx="1" /><rect x="50" y="35" width="10" height="4" rx="1" /><rect x="50" y="45" width="10" height="4" rx="1" /><rect x="50" y="55" width="10" height="4" rx="1" /><rect x="50" y="65" width="10" height="4" rx="1" /><rect x="50" y="75" width="10" height="4" rx="1" /><rect x="50" y="85" width="10" height="4" rx="1" /><rect x="50" y="95" width="10" height="4" rx="1" /><rect x="50" y="105" width="10" height="4" rx="1" /><rect x="50" y="115" width="10" height="4" rx="1" /><rect x="50" y="125" width="10" height="4" rx="1" /><rect x="50" y="135" width="10" height="4" rx="1" /><rect x="50" y="145" width="10" height="4" rx="1" /><rect x="50" y="155" width="10" height="4" rx="1" /><rect x="50" y="165" width="10" height="4" rx="1" /><rect x="50" y="175" width="10" height="4" rx="1" /><rect x="50" y="185" width="10" height="4" rx="1" /><rect x="50" y="195" width="10" height="4" rx="1" /><rect x="50" y="205" width="10" height="4" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="end">
      <text x="45" y="19">1</text><text x="45" y="29">2</text><text x="45" y="39">3</text><text x="45" y="49">4</text><text x="45" y="59">5</text><text x="45" y="69">6</text><text x="45" y="79">7</text><text x="45" y="89">8</text><text x="45" y="99">9</text><text x="45" y="109">10</text><text x="45" y="119">11</text><text x="45" y="129">12</text><text x="45" y="139">13</text><text x="45" y="149">14</text><text x="45" y="159">15</text><text x="45" y="169">16</text><text x="45" y="179">17</text><text x="45" y="189">18</text><text x="45" y="199">19</text><text x="45" y="209">20</text>
    </g>
    <g fill="var(--text-secondary)">
      <rect x="380" y="15" width="10" height="4" rx="1" /><rect x="380" y="25" width="10" height="4" rx="1" /><rect x="380" y="35" width="10" height="4" rx="1" /><rect x="380" y="45" width="10" height="4" rx="1" /><rect x="380" y="55" width="10" height="4" rx="1" /><rect x="380" y="65" width="10" height="4" rx="1" /><rect x="380" y="75" width="10" height="4" rx="1" /><rect x="380" y="85" width="10" height="4" rx="1" /><rect x="380" y="95" width="10" height="4" rx="1" /><rect x="380" y="105" width="10" height="4" rx="1" /><rect x="380" y="115" width="10" height="4" rx="1" /><rect x="380" y="125" width="10" height="4" rx="1" /><rect x="380" y="135" width="10" height="4" rx="1" /><rect x="380" y="145" width="10" height="4" rx="1" /><rect x="380" y="155" width="10" height="4" rx="1" /><rect x="380" y="165" width="10" height="4" rx="1" /><rect x="380" y="175" width="10" height="4" rx="1" /><rect x="380" y="185" width="10" height="4" rx="1" /><rect x="380" y="195" width="10" height="4" rx="1" /><rect x="380" y="205" width="10" height="4" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="6" font-family="var(--font-mono)" text-anchor="start">
      <text x="395" y="19">40</text><text x="395" y="29">39</text><text x="395" y="39">38</text><text x="395" y="49">37</text><text x="395" y="59">36</text><text x="395" y="69">35</text><text x="395" y="79">34</text><text x="395" y="89">33</text><text x="395" y="99">32</text><text x="395" y="109">31</text><text x="395" y="119">30</text><text x="395" y="129">29</text><text x="395" y="139">28</text><text x="395" y="149">27</text><text x="395" y="159">26</text><text x="395" y="169">25</text><text x="395" y="179">24</text><text x="395" y="189">23</text><text x="395" y="199">22</text><text x="395" y="209">21</text>
    </g>
    <text x="220" y="105" fill="var(--text-primary)" font-size="14" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{activeId.toUpperCase()}</text>
  </svg>

{:else if activeId === 'lm2596'}
  <!-- TO-220-5 -->
  <svg viewBox="0 0 240 100" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="80" y="10" width="80" height="50" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <g fill="var(--text-secondary)">
      <rect x="90" y="60" width="6" height="20" rx="1" /><rect x="102" y="60" width="6" height="20" rx="1" /><rect x="114" y="60" width="6" height="20" rx="1" /><rect x="126" y="60" width="6" height="20" rx="1" /><rect x="138" y="60" width="6" height="20" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
      <text x="93" y="90">1</text><text x="105" y="90">2</text><text x="117" y="90">3</text><text x="129" y="90">4</text><text x="141" y="90">5</text>
    </g>
    <text x="120" y="40" fill="var(--text-primary)" font-size="12" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{activeId.toUpperCase()}</text>
  </svg>

{:else if activeId === 'ams1117'}
  <!-- SOT-223 -->
  <svg viewBox="0 0 240 120" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="70" y="20" width="100" height="60" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    <g fill="var(--text-secondary)">
      <rect x="85" y="80" width="8" height="15" rx="1" /><rect x="116" y="80" width="8" height="15" rx="1" /><rect x="147" y="80" width="8" height="15" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
      <text x="89" y="105">1</text><text x="120" y="105">2</text><text x="151" y="105">3</text>
    </g>
    <rect x="100" y="8" width="40" height="12" rx="2" fill="var(--text-secondary)"/>
    <text x="120" y="105" fill="var(--text-muted)" font-size="7" font-family="var(--font-sans)" text-anchor="middle">TAB</text>
    <text x="120" y="55" fill="var(--text-primary)" font-size="12" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">{activeId.toUpperCase()}</text>
  </svg>

{:else if activeId === 'esp01'}
  <!-- ESP-01 (2x4) ИСПРАВЛЕНО -->
  <svg viewBox="0 0 240 120" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
    <!-- Корпус -->
    <rect x="60" y="30" width="120" height="60" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
    
    <!-- Верхний ряд (Пины 1-4) -->
    <g fill="var(--text-secondary)">
      <rect x="75" y="20" width="6" height="10" rx="1" />
      <rect x="95" y="20" width="6" height="10" rx="1" />
      <rect x="115" y="20" width="6" height="10" rx="1" />
      <rect x="135" y="20" width="6" height="10" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
      <text x="78" y="15">1</text><text x="98" y="15">2</text><text x="118" y="15">3</text><text x="138" y="15">4</text>
    </g>

    <!-- Нижний ряд (Пины 5-8) -->
    <g fill="var(--text-secondary)">
      <rect x="75" y="90" width="6" height="10" rx="1" />
      <rect x="95" y="90" width="6" height="10" rx="1" />
      <rect x="115" y="90" width="6" height="10" rx="1" />
      <rect x="135" y="90" width="6" height="10" rx="1" />
    </g>
    <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
      <text x="78" y="108">5</text><text x="98" y="108">6</text><text x="118" y="108">7</text><text x="138" y="108">8</text>
    </g>

    <text x="120" y="65" fill="var(--text-primary)" font-size="12" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">ESP-01</text>
  </svg>

{:else}
  <div class="scheme-placeholder">Нет схемы для этого корпуса</div>
{/if}

<style>
  .scheme-placeholder {
    color: var(--text-muted);
    font-style: italic;
  }
</style>