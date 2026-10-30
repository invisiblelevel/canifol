<script>
  import Card from '../components/Card.svelte';
  import StarButton from '../components/StarButton.svelte';
  import { getStrings } from '../strings.svelte.js';
  import { pinouts } from '../data/pinouts/index.js';
  import MicrochipScheme from '../components/pinouts/MicrochipScheme.svelte';
  import DevBoardScheme from '../components/pinouts/DevBoardScheme.svelte';
  import { favoritesStore } from '../stores/favorites.svelte.js';

  const strings = $derived(getStrings());
  const F = $derived(strings.favorites);

  // Принимаем начальные значения из App.svelte
  let { initialTab = 'connectors', initialId = pinouts.connectors[0]?.id ?? '' } = $props();

  let activeTab = $state(initialTab);
  let activeId = $state(initialId);
  let search = $state('');

  const currentList = $derived(pinouts[activeTab] || []);

  const filteredList = $derived(currentList.filter(c => {
    if (!search) return true;
    const name = activeTab === 'soviet' ? strings.pinoutsSovietData[c.id]?.name :
                 activeTab === 'devBoards' ? strings.pinoutsDevBoardsData[c.id]?.name :
                 strings.pinoutsData[c.id]?.name || strings.pinoutsIcsData[c.id]?.name;
    const query = search.toLowerCase();
    return c.id.toLowerCase().includes(query) || (name && name.toLowerCase().includes(query));
  }));

  const activeMeta = $derived(currentList.find(c => c.id === activeId));
  const activeData = $derived(
    activeTab === 'soviet' ? strings.pinoutsSovietData[activeId] :
    activeTab === 'devBoards' ? strings.pinoutsDevBoardsData[activeId] :
    activeTab === 'ics' ? strings.pinoutsIcsData[activeId] :
    strings.pinoutsData[activeId]
  );
  const activePins = $derived(activeMeta?.pins ?? []);
  const activeName = $derived(
    activeTab === 'soviet' ? strings.pinoutsSovietData[activeId]?.name :
    activeTab === 'devBoards' ? strings.pinoutsDevBoardsData[activeId]?.name :
    activeTab === 'ics' ? strings.pinoutsIcsData[activeId]?.name :
    strings.pinoutsData[activeId]?.name
  );
  const isActiveFav = $derived(activeId ? favoritesStore.isFavoritePinout(activeId) : false);

  function switchTab(tab) {
    activeTab = tab;
    activeId = pinouts[tab][0]?.id ?? '';
    search = '';
  }

  // Следим, чтобы при поиске activeId не улетал в пустоту
  $effect(() => {
    // Не сбрасываем активный ID, если мы только что пришли из избранного
    if (activeId && currentList.some(c => c.id === activeId)) return;
    
    if (filteredList.length > 0 && !filteredList.some(c => c.id === activeId)) {
      activeId = filteredList[0].id;
    }
  });

  function toggleFav() {
    if (!activeId || !activeData) return;
    favoritesStore.togglePinout({
      id: activeId,
      tab: activeTab,
      name: activeName || activeId.toUpperCase()
    });
  }
</script>

<Card title={strings.pinouts.title}>
  <div class="pinout-tabs no-select">
    <button class="tab-btn" class:active={activeTab === 'connectors'} onclick={() => switchTab('connectors')}>
      {strings.pinoutsTabs.connectors}
    </button>
    <button class="tab-btn" class:active={activeTab === 'ics'} onclick={() => switchTab('ics')}>
      {strings.pinoutsTabs.ics}
    </button>
    <button class="tab-btn" class:active={activeTab === 'soviet'} onclick={() => switchTab('soviet')}>
      {strings.pinoutsTabs.soviet}
    </button>
    <button class="tab-btn" class:active={activeTab === 'devBoards'} onclick={() => switchTab('devBoards')}>
      {strings.pinoutsTabs.devBoards}
    </button>
  </div>

  <div class="pinout-layout">
    <nav class="pinout-nav">
      <div class="pinout-search">
        <input 
          type="search" 
          placeholder={strings.pinouts.searchPlaceholder} 
          bind:value={search} 
        />
      </div>
      <div class="pinout-nav__list custom-scrollbar">
        {#each filteredList as conn (conn.id)}
          <button 
            class="pinout-nav__btn" 
            class:active={activeId === conn.id}
            onclick={() => activeId = conn.id}
          >
            {activeTab === 'soviet' ? strings.pinoutsSovietData[conn.id]?.name : 
             activeTab === 'devBoards' ? strings.pinoutsDevBoardsData[conn.id]?.name : 
             activeTab === 'ics' ? strings.pinoutsIcsData[conn.id]?.name || conn.id.toUpperCase() :
             strings.pinoutsData[conn.id]?.name || conn.id}
          </button>
        {:else}
          <div class="pinout-nav__empty">{strings.pinouts.nothingFound}</div>
        {/each}
      </div>
    </nav>

    {#if activeData}
      <div class="pinout-content">
        <div class="pinout-header">
          <div class="pinout-header__info">
            <h2>{activeData.name}</h2>
            <p>{activeData.desc}</p>
          </div>
          <StarButton
            active={isActiveFav}
            onclick={toggleFav}
            title={isActiveFav ? F.removeFromFavorites : F.addToFavorites}
          />
        </div>

        <div class="pinout-scheme">
          {#if activeTab === 'connectors'}
            {#if activeId === 'usb-a'}
              <svg viewBox="0 0 240 100" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
                <rect x="30" y="20" width="180" height="60" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
                <rect x="60" y="35" width="14" height="15" rx="2" fill="var(--text-secondary)"/>
                <text x="67" y="46" fill="var(--bg-base)" font-size="10" font-family="var(--font-mono)" text-anchor="middle" font-weight="bold">1</text>
                <text x="67" y="65" fill="var(--accent)" font-size="9" font-family="var(--font-sans)" text-anchor="middle">VBUS</text>
                <rect x="84" y="35" width="14" height="15" rx="2" fill="var(--text-secondary)"/>
                <text x="91" y="46" fill="var(--bg-base)" font-size="10" font-family="var(--font-mono)" text-anchor="middle" font-weight="bold">2</text>
                <text x="91" y="65" fill="var(--accent)" font-size="9" font-family="var(--font-sans)" text-anchor="middle">D-</text>
                <rect x="108" y="35" width="14" height="15" rx="2" fill="var(--text-secondary)"/>
                <text x="115" y="46" fill="var(--bg-base)" font-size="10" font-family="var(--font-mono)" text-anchor="middle" font-weight="bold">3</text>
                <text x="115" y="65" fill="var(--accent)" font-size="9" font-family="var(--font-sans)" text-anchor="middle">D+</text>
                <rect x="132" y="35" width="14" height="15" rx="2" fill="var(--text-secondary)"/>
                <text x="139" y="46" fill="var(--bg-base)" font-size="10" font-family="var(--font-mono)" text-anchor="middle" font-weight="bold">4</text>
                <text x="139" y="65" fill="var(--accent)" font-size="9" font-family="var(--font-sans)" text-anchor="middle">GND</text>
              </svg>
            {:else if activeId === 'usb-b'}
              <svg viewBox="0 0 240 120" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
                <rect x="60" y="10" width="120" height="100" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
                <rect x="80" y="30" width="80" height="60" rx="2" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="1" stroke-dasharray="4 2"/>
                <g fill="var(--text-secondary)">
                  <rect x="85" y="40" width="12" height="8" rx="1" /><rect x="85" y="55" width="12" height="8" rx="1" /><rect x="85" y="70" width="12" height="8" rx="1" /><rect x="135" y="40" width="12" height="8" rx="1" />
                </g>
                <g fill="var(--accent)" font-size="9" font-family="var(--font-mono)" text-anchor="middle">
                  <text x="91" y="36">1</text><text x="91" y="51">2</text><text x="91" y="66">3</text><text x="141" y="36">4</text>
                </g>
              </svg>
            {:else if activeId === 'usb-mini'}
              <svg viewBox="0 0 240 120" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M60 20 L180 20 L190 100 L50 100 Z" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
                <g fill="var(--text-secondary)">
                  <rect x="75" y="40" width="10" height="8" rx="1" /><rect x="90" y="40" width="10" height="8" rx="1" /><rect x="105" y="40" width="10" height="8" rx="1" /><rect x="120" y="40" width="10" height="8" rx="1" /><rect x="135" y="40" width="10" height="8" rx="1" />
                </g>
                <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="middle">
                  <text x="80" y="36">1</text><text x="95" y="36">2</text><text x="110" y="36">3</text><text x="125" y="36">4</text><text x="140" y="36">5</text>
                </g>
                <g fill="var(--accent)" font-size="8" font-family="var(--font-sans)" text-anchor="middle">
                  <text x="80" y="60">VBUS</text><text x="95" y="60">D-</text><text x="110" y="60">D+</text><text x="125" y="60">ID</text><text x="140" y="60">GND</text>
                </g>
              </svg>
            {:else if activeId === 'usb-micro'}
              <svg viewBox="0 0 240 120" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
                <rect x="70" y="30" width="100" height="60" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
                <g fill="var(--text-secondary)">
                  <rect x="80" y="45" width="8" height="10" rx="1" /><rect x="96" y="45" width="8" height="10" rx="1" /><rect x="112" y="45" width="8" height="10" rx="1" /><rect x="128" y="45" width="8" height="10" rx="1" /><rect x="144" y="45" width="8" height="10" rx="1" />
                </g>
                <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="middle">
                  <text x="84" y="40">1</text><text x="100" y="40">2</text><text x="116" y="40">3</text><text x="132" y="40">4</text><text x="148" y="40">5</text>
                </g>
                <g fill="var(--accent)" font-size="8" font-family="var(--font-sans)" text-anchor="middle">
                  <text x="84" y="75">VBUS</text><text x="100" y="75">D-</text><text x="116" y="75">D+</text><text x="132" y="75">ID</text><text x="148" y="75">GND</text>
                </g>
              </svg>
            {:else if activeId === 'usb-c'}
              <svg viewBox="0 0 240 120" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
                <rect x="20" y="30" width="200" height="50" rx="25" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
                <rect x="40" y="40" width="160" height="30" rx="15" fill="var(--text-secondary)" opacity="0.3"/>
                <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
                  <text x="60" y="25">A1</text><text x="80" y="25">A2</text><text x="100" y="25">A3</text><text x="120" y="25">A4</text><text x="140" y="25">A5</text><text x="160" y="25">A6</text><text x="180" y="25">A7</text>
                </g>
                <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
                  <text x="60" y="95">B1</text><text x="80" y="95">B2</text><text x="100" y="95">B3</text><text x="120" y="95">B4</text><text x="140" y="95">B5</text><text x="160" y="95">B6</text><text x="180" y="95">B7</text>
                </g>
                <g fill="var(--text-secondary)">
                  <circle cx="60" cy="35" r="3" /><circle cx="80" cy="35" r="3" /><circle cx="100" cy="35" r="3" /><circle cx="120" cy="35" r="3" /><circle cx="140" cy="35" r="3" /><circle cx="160" cy="35" r="3" /><circle cx="180" cy="35" r="3" />
                  <circle cx="60" cy="75" r="3" /><circle cx="80" cy="75" r="3" /><circle cx="100" cy="75" r="3" /><circle cx="120" cy="75" r="3" /><circle cx="140" cy="75" r="3" /><circle cx="160" cy="75" r="3" /><circle cx="180" cy="75" r="3" />
                </g>
              </svg>
            {:else if activeId === 'usb3-a'}
              <svg viewBox="0 0 240 100" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
                <rect x="30" y="20" width="180" height="60" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
                <g fill="var(--text-secondary)">
                  <rect x="45" y="45" width="8" height="15" rx="1" /><rect x="60" y="45" width="8" height="15" rx="1" /><rect x="75" y="45" width="8" height="15" rx="1" /><rect x="90" y="45" width="8" height="15" rx="1" />
                </g>
                <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
                  <text x="49" y="70">1</text><text x="64" y="70">2</text><text x="79" y="70">3</text><text x="94" y="70">4</text>
                </g>
                <g fill="var(--text-secondary)">
                  <rect x="115" y="45" width="8" height="15" rx="1" /><rect x="130" y="45" width="8" height="15" rx="1" /><rect x="145" y="45" width="8" height="15" rx="1" /><rect x="160" y="45" width="8" height="15" rx="1" /><rect x="175" y="45" width="8" height="15" rx="1" />
                </g>
                <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
                  <text x="119" y="70">5</text><text x="134" y="70">6</text><text x="149" y="70">7</text><text x="164" y="70">8</text><text x="179" y="70">9</text>
                </g>
              </svg>
            {:else if activeId === 'hdmi'}
              <svg viewBox="0 0 240 100" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
                <rect x="30" y="20" width="180" height="60" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
                <g fill="var(--text-secondary)">
                  <rect x="45" y="35" width="6" height="10" rx="1" /><rect x="60" y="35" width="6" height="10" rx="1" /><rect x="75" y="35" width="6" height="10" rx="1" /><rect x="90" y="35" width="6" height="10" rx="1" /><rect x="105" y="35" width="6" height="10" rx="1" /><rect x="120" y="35" width="6" height="10" rx="1" /><rect x="135" y="35" width="6" height="10" rx="1" /><rect x="150" y="35" width="6" height="10" rx="1" /><rect x="165" y="35" width="6" height="10" rx="1" /><rect x="180" y="35" width="6" height="10" rx="1" />
                </g>
                <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
                  <text x="48" y="30">1</text><text x="63" y="30">3</text><text x="78" y="30">5</text><text x="93" y="30">7</text><text x="108" y="30">9</text><text x="123" y="30">11</text><text x="138" y="30">13</text><text x="153" y="30">15</text><text x="168" y="30">17</text><text x="183" y="30">19</text>
                </g>
                <g fill="var(--text-secondary)">
                  <rect x="52" y="55" width="6" height="10" rx="1" /><rect x="67" y="55" width="6" height="10" rx="1" /><rect x="82" y="55" width="6" height="10" rx="1" /><rect x="97" y="55" width="6" height="10" rx="1" /><rect x="112" y="55" width="6" height="10" rx="1" /><rect x="127" y="55" width="6" height="10" rx="1" /><rect x="142" y="55" width="6" height="10" rx="1" /><rect x="157" y="55" width="6" height="10" rx="1" /><rect x="172" y="55" width="6" height="10" rx="1" />
                </g>
                <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
                  <text x="55" y="75">2</text><text x="70" y="75">4</text><text x="85" y="75">6</text><text x="100" y="75">8</text><text x="115" y="75">10</text><text x="130" y="75">12</text><text x="145" y="75">14</text><text x="160" y="75">16</text><text x="175" y="75">18</text>
                </g>
              </svg>
            {:else if activeId === 'displayport'}
              <svg viewBox="0 0 240 120" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
                <rect x="30" y="20" width="180" height="60" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
                <g fill="var(--text-secondary)">
                  <rect x="45" y="35" width="6" height="10" rx="1" /><rect x="60" y="35" width="6" height="10" rx="1" /><rect x="75" y="35" width="6" height="10" rx="1" /><rect x="90" y="35" width="6" height="10" rx="1" /><rect x="105" y="35" width="6" height="10" rx="1" /><rect x="120" y="35" width="6" height="10" rx="1" /><rect x="135" y="35" width="6" height="10" rx="1" /><rect x="150" y="35" width="6" height="10" rx="1" /><rect x="165" y="35" width="6" height="10" rx="1" /><rect x="180" y="35" width="6" height="10" rx="1" />
                </g>
                <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
                  <text x="48" y="30">1</text><text x="63" y="30">3</text><text x="78" y="30">5</text><text x="93" y="30">7</text><text x="108" y="30">9</text><text x="123" y="30">11</text><text x="138" y="30">13</text><text x="153" y="30">15</text><text x="168" y="30">17</text><text x="183" y="30">19</text>
                </g>
                <g fill="var(--text-secondary)">
                  <rect x="52" y="55" width="6" height="10" rx="1" /><rect x="67" y="55" width="6" height="10" rx="1" /><rect x="82" y="55" width="6" height="10" rx="1" /><rect x="97" y="55" width="6" height="10" rx="1" /><rect x="112" y="55" width="6" height="10" rx="1" /><rect x="127" y="55" width="6" height="10" rx="1" /><rect x="142" y="55" width="6" height="10" rx="1" /><rect x="157" y="55" width="6" height="10" rx="1" /><rect x="172" y="55" width="6" height="10" rx="1" />
                </g>
                <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
                  <text x="55" y="75">2</text><text x="70" y="75">4</text><text x="85" y="75">6</text><text x="100" y="75">8</text><text x="115" y="75">10</text><text x="130" y="75">12</text><text x="145" y="75">14</text><text x="160" y="75">16</text><text x="175" y="75">18</text>
                </g>
                <text x="120" y="105" fill="var(--accent)" font-size="9" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">20</text>
              </svg>
            {:else if activeId === 'vga'}
              <svg viewBox="0 0 240 120" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M40 20 L200 20 L220 90 L20 90 Z" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
                <g fill="var(--text-secondary)">
                  <circle cx="70" cy="38" r="4" /><circle cx="95" cy="38" r="4" /><circle cx="120" cy="38" r="4" /><circle cx="145" cy="38" r="4" /><circle cx="170" cy="38" r="4" />
                </g>
                <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="middle">
                  <text x="70" y="32">1</text><text x="95" y="32">2</text><text x="120" y="32">3</text><text x="145" y="32">4</text><text x="170" y="32">5</text>
                </g>
                <g fill="var(--text-secondary)">
                  <circle cx="60" cy="55" r="4" /><circle cx="85" cy="55" r="4" /><circle cx="110" cy="55" r="4" /><circle cx="135" cy="55" r="4" /><circle cx="160" cy="55" r="4" />
                </g>
                <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="middle">
                  <text x="60" y="49">6</text><text x="85" y="49">7</text><text x="110" y="49">8</text><text x="135" y="49">9</text><text x="160" y="49">10</text>
                </g>
                <g fill="var(--text-secondary)">
                  <circle cx="75" cy="72" r="4" /><circle cx="100" cy="72" r="4" /><circle cx="125" cy="72" r="4" /><circle cx="150" cy="72" r="4" />
                </g>
                <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="middle">
                  <text x="75" y="66">11</text><text x="100" y="66">12</text><text x="125" y="66">13</text><text x="150" y="66">14</text>
                </g>
              </svg>
            {:else if activeId === 'jack-3.5-trs'}
              <svg viewBox="0 0 320 100" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
                <rect x="20" y="30" width="30" height="40" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
                <path d="M50 35 L170 35 L170 65 L50 65 Z" fill="var(--text-secondary)" opacity="0.3"/>
                <text x="110" y="55" fill="var(--text-primary)" font-size="10" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">Sleeve (GND)</text>
                <rect x="170" y="35" width="10" height="30" fill="var(--bg-base)"/>
                <rect x="180" y="35" width="20" height="30" fill="var(--text-secondary)" opacity="0.8"/>
                <text x="190" y="20" fill="var(--accent)" font-size="9" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">Ring (R)</text>
                <rect x="200" y="35" width="10" height="30" fill="var(--bg-base)"/>
                <path d="M210 35 L280 45 L280 55 L210 65 Z" fill="var(--text-secondary)"/>
                <text x="245" y="85" fill="var(--accent)" font-size="9" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">Tip (L)</text>
              </svg>
            {:else if activeId === 'jack-3.5-trrs'}
              <svg viewBox="0 0 360 100" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
                <rect x="20" y="30" width="30" height="40" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
                <path d="M50 35 L150 35 L150 65 L50 65 Z" fill="var(--text-secondary)" opacity="0.3"/>
                <text x="100" y="55" fill="var(--text-primary)" font-size="10" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">Sleeve (Mic)</text>
                <rect x="150" y="35" width="10" height="30" fill="var(--bg-base)"/>
                <rect x="160" y="35" width="20" height="30" fill="var(--text-secondary)" opacity="0.6"/>
                <text x="170" y="20" fill="var(--accent)" font-size="9" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">Ring 2 (GND)</text>
                <rect x="180" y="35" width="10" height="30" fill="var(--bg-base)"/>
                <rect x="190" y="35" width="20" height="30" fill="var(--text-secondary)" opacity="0.8"/>
                <text x="200" y="85" fill="var(--accent)" font-size="9" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">Ring 1 (R)</text>
                <rect x="210" y="35" width="10" height="30" fill="var(--bg-base)"/>
                <path d="M220 35 L300 45 L300 55 L220 65 Z" fill="var(--text-secondary)"/>
                <text x="260" y="20" fill="var(--accent)" font-size="9" font-family="var(--font-sans)" text-anchor="middle" font-weight="bold">Tip (L)</text>
              </svg>
            {:else if activeId === 'rj45'}
              <svg viewBox="0 0 240 120" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
                <rect x="60" y="20" width="120" height="80" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
                <rect x="80" y="10" width="80" height="10" rx="2" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
                <g fill="var(--text-secondary)">
                  <rect x="70" y="40" width="8" height="12" rx="1" /><rect x="85" y="40" width="8" height="12" rx="1" /><rect x="100" y="40" width="8" height="12" rx="1" /><rect x="115" y="40" width="8" height="12" rx="1" /><rect x="130" y="40" width="8" height="12" rx="1" /><rect x="145" y="40" width="8" height="12" rx="1" /><rect x="160" y="40" width="8" height="12" rx="1" /><rect x="175" y="40" width="8" height="12" rx="1" />
                </g>
                <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="middle">
                  <text x="74" y="36">1</text><text x="89" y="36">2</text><text x="104" y="36">3</text><text x="119" y="36">4</text><text x="134" y="36">5</text><text x="149" y="36">6</text><text x="164" y="36">7</text><text x="179" y="36">8</text>
                </g>
              </svg>
            {:else if activeId === 'ps2'}
              <svg viewBox="0 0 240 120" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
                <circle cx="120" cy="60" r="40" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
                <g fill="var(--text-secondary)">
                  <circle cx="100" cy="40" r="4" /><circle cx="140" cy="40" r="4" /><circle cx="100" cy="80" r="4" /><circle cx="140" cy="80" r="4" /><circle cx="120" cy="30" r="4" /><circle cx="120" cy="90" r="4" />
                </g>
                <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="middle">
                  <text x="100" y="35">1</text><text x="140" y="35">2</text><text x="100" y="95">3</text><text x="140" y="95">4</text><text x="120" y="25">5</text><text x="120" y="105">6</text>
                </g>
              </svg>
            {:else if activeId === 'atx-24'}
              <svg viewBox="0 0 300 100" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
                <rect x="20" y="20" width="260" height="60" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
                <g fill="var(--text-secondary)">
                  <rect x="30" y="35" width="8" height="10" rx="1" /><rect x="50" y="35" width="8" height="10" rx="1" /><rect x="70" y="35" width="8" height="10" rx="1" /><rect x="90" y="35" width="8" height="10" rx="1" /><rect x="110" y="35" width="8" height="10" rx="1" /><rect x="130" y="35" width="8" height="10" rx="1" /><rect x="150" y="35" width="8" height="10" rx="1" /><rect x="170" y="35" width="8" height="10" rx="1" /><rect x="190" y="35" width="8" height="10" rx="1" /><rect x="210" y="35" width="8" height="10" rx="1" /><rect x="230" y="35" width="8" height="10" rx="1" /><rect x="250" y="35" width="8" height="10" rx="1" />
                </g>
                <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
                  <text x="34" y="30">1</text><text x="54" y="30">2</text><text x="74" y="30">3</text><text x="94" y="30">4</text><text x="114" y="30">5</text><text x="134" y="30">6</text><text x="154" y="30">7</text><text x="174" y="30">8</text><text x="194" y="30">9</text><text x="214" y="30">10</text><text x="234" y="30">11</text><text x="254" y="30">12</text>
                </g>
                <g fill="var(--text-secondary)">
                  <rect x="30" y="55" width="8" height="10" rx="1" /><rect x="50" y="55" width="8" height="10" rx="1" /><rect x="70" y="55" width="8" height="10" rx="1" /><rect x="90" y="55" width="8" height="10" rx="1" /><rect x="110" y="55" width="8" height="10" rx="1" /><rect x="130" y="55" width="8" height="10" rx="1" /><rect x="150" y="55" width="8" height="10" rx="1" /><rect x="170" y="55" width="8" height="10" rx="1" /><rect x="190" y="55" width="8" height="10" rx="1" /><rect x="210" y="55" width="8" height="10" rx="1" /><rect x="230" y="55" width="8" height="10" rx="1" /><rect x="250" y="55" width="8" height="10" rx="1" />
                </g>
                <g fill="var(--accent)" font-size="7" font-family="var(--font-mono)" text-anchor="middle">
                  <text x="34" y="75">13</text><text x="54" y="75">14</text><text x="74" y="75">15</text><text x="94" y="75">16</text><text x="114" y="75">17</text><text x="134" y="75">18</text><text x="154" y="75">19</text><text x="174" y="75">20</text><text x="194" y="75">21</text><text x="214" y="75">22</text><text x="234" y="75">23</text><text x="254" y="75">24</text>
                </g>
              </svg>
            {:else if activeId === 'sata-power'}
              <svg viewBox="0 0 300 110" width="100%" height="auto" fill="none" xmlns="http://www.w3.org/2000/svg">
                <rect x="20" y="20" width="260" height="40" rx="4" fill="var(--bg-elevated)" stroke="var(--text-muted)" stroke-width="2"/>
                <g fill="var(--text-secondary)">
                  <rect x="30" y="30" width="8" height="20" rx="1" /><rect x="45" y="30" width="8" height="20" rx="1" /><rect x="60" y="30" width="8" height="20" rx="1" /><rect x="75" y="30" width="8" height="20" rx="1" /><rect x="90" y="30" width="8" height="20" rx="1" /><rect x="105" y="30" width="8" height="20" rx="1" /><rect x="120" y="30" width="8" height="20" rx="1" /><rect x="135" y="30" width="8" height="20" rx="1" /><rect x="150" y="30" width="8" height="20" rx="1" /><rect x="165" y="30" width="8" height="20" rx="1" /><rect x="180" y="30" width="8" height="20" rx="1" /><rect x="195" y="30" width="8" height="20" rx="1" /><rect x="210" y="30" width="8" height="20" rx="1" /><rect x="225" y="30" width="8" height="20" rx="1" /><rect x="240" y="30" width="8" height="20" rx="1" />
                </g>
                <g fill="var(--accent)" font-size="8" font-family="var(--font-mono)" text-anchor="middle">
                  <text x="34" y="72">1</text><text x="49" y="72">2</text><text x="64" y="72">3</text><text x="79" y="72">4</text><text x="94" y="72">5</text><text x="109" y="72">6</text><text x="124" y="72">7</text><text x="139" y="72">8</text><text x="154" y="72">9</text><text x="169" y="72">10</text><text x="184" y="72">11</text><text x="199" y="72">12</text><text x="214" y="72">13</text><text x="229" y="72">14</text><text x="244" y="72">15</text>
                </g>
              </svg>
            {:else}
              <div class="scheme-placeholder">{activeData.name}</div>
            {/if}
          {:else if activeTab === 'devBoards'}
            <DevBoardScheme {activeId} />
          {:else}
            <MicrochipScheme {activeId} />
          {/if}
        </div>

        <div class="pinout-table-wrap">
          <table class="pinout-table">
            <thead>
              <tr>
                <th>{strings.pinouts.pinNumber}</th>
                <th>{strings.pinouts.pinName}</th>
                <th>{strings.pinouts.pinDesc}</th>
              </tr>
            </thead>
            <tbody>
              {#each activePins as pinNumber (pinNumber)}
                {@const pinInfo = activeData.pins ? activeData.pins[pinNumber] : null}
                {#if pinInfo}
                  <tr>
                    <td class="pinout-table__num">{pinNumber}</td>
                    <td class="pinout-table__name">{pinInfo.name}</td>
                    <td class="pinout-table__desc">{pinInfo.desc}</td>
                  </tr>
                {/if}
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    {:else}
      <div class="pinout-content pinout-empty">
        <p>{strings.pinouts.nothingFound}</p>
      </div>
    {/if}
  </div>
</Card>

<style>
  .pinout-tabs { display: flex; gap: var(--space-2); margin-bottom: var(--space-4); border-bottom: 1px solid var(--border-subtle); padding-bottom: var(--space-2); overflow-x: auto; }
  .tab-btn { padding: var(--space-2) var(--space-4); border-radius: var(--radius-md); color: var(--text-secondary); font-size: var(--font-size-sm); font-weight: 600; transition: background var(--transition-fast), color var(--transition-fast); white-space: nowrap; }
  .tab-btn:hover { background: var(--bg-elevated-2); color: var(--text-primary); }
  .tab-btn.active { background: var(--accent-dim); color: var(--accent); }
  
  .pinout-layout { display: grid; grid-template-columns: 240px 1fr; gap: var(--space-4); min-height: 500px; }
  .pinout-nav { display: flex; flex-direction: column; gap: var(--space-2); border-right: 1px solid var(--border-subtle); padding-right: var(--space-3); height: 100%; max-height: 70vh; }
  
  .pinout-search input {
    width: 100%;
    min-height: 36px;
    padding: 0 var(--space-3);
    background: var(--bg-elevated-2);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-size: var(--font-size-sm);
  }
  .pinout-search input:focus { outline: none; border-color: var(--accent); }

  .pinout-nav__list {
    display: flex;
    flex-direction: column;
    gap: 2px;
    overflow-y: auto;
    flex-grow: 1;
    padding-right: 4px;
  }
  
  .custom-scrollbar::-webkit-scrollbar { width: 6px; }
  .custom-scrollbar::-webkit-scrollbar-track { background: var(--bg-elevated); border-radius: 4px; }
  .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border-strong); border-radius: 4px; }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--text-muted); }

  .pinout-nav__btn {
    text-align: left;
    padding: var(--space-2) var(--space-3);
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    font-size: var(--font-size-sm);
    font-weight: 500;
    letter-spacing: 0.02em;
    transition: background var(--transition-fast), color var(--transition-fast);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    line-height: 1.4;
    min-height: 44px;
    display: flex;
    align-items: center;
  }
  .pinout-nav__btn:hover { background: var(--bg-elevated-2); color: var(--text-primary); }
  .pinout-nav__btn.active { background: var(--accent-dim); color: var(--accent); font-weight: 600; }
  .pinout-nav__empty { padding: var(--space-3); color: var(--text-muted); font-size: var(--font-size-sm); font-style: italic; }
  
  .pinout-content { display: flex; flex-direction: column; gap: var(--space-4); }
  .pinout-empty { align-items: center; justify-content: center; color: var(--text-muted); font-style: italic; }
  .pinout-header { display: flex; justify-content: space-between; align-items: flex-start; gap: var(--space-3); }
  .pinout-header__info h2 { font-size: var(--font-size-xl); color: var(--accent); margin-bottom: 4px; }
  .pinout-header__info p { font-size: var(--font-size-sm); color: var(--text-muted); }
  .pinout-scheme { background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); padding: var(--space-4); display: flex; justify-content: center; align-items: center; min-height: 200px; }
  .scheme-placeholder { color: var(--text-muted); font-style: italic; }
  .pinout-table-wrap { overflow-x: auto; }
  .pinout-table { width: 100%; border-collapse: collapse; font-size: var(--font-size-sm); }
  .pinout-table th { text-align: left; padding: var(--space-2) var(--space-3); color: var(--text-muted); font-weight: 600; border-bottom: 2px solid var(--border-strong); }
  .pinout-table td { padding: var(--space-2) var(--space-3); border-bottom: 1px solid var(--border-subtle); color: var(--text-secondary); }
  .pinout-table__num { font-family: var(--font-mono); color: var(--text-primary) !important; }
  .pinout-table__name { font-weight: 600; color: var(--text-primary) !important; }
  
  @media (max-width: 768px) {
    .pinout-layout { 
      grid-template-columns: 1fr; 
      display: flex;
      flex-direction: column;
    }
    
    .pinout-nav { 
      border-right: none; 
      border-bottom: 1px solid var(--border-subtle); 
      padding-right: 0; 
      padding-bottom: var(--space-3); 
      max-height: 220px;
      height: auto;
      width: 100%;
    }
    
    .pinout-nav__list { 
      flex-direction: column;
      flex-wrap: nowrap; 
      overflow-y: auto;
      overflow-x: hidden;
      max-height: 160px; 
    }
    
    .pinout-nav__btn { 
      white-space: nowrap;
    }
  }
</style>