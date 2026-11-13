<script>
  let { size = 40 } = $props();
</script>

<img
  src="/logo.png"
  width={size}
  height={size}
  alt="Canifol"
  class="logo"
  style="border-radius: {size * 0.22}px;"
/>

<style>
  .logo {
    flex-shrink: 0;
    display: block;
    object-fit: contain;
    /* Добавляем лёгкую тень, чтобы иконка выделялась на тёмном фоне */
    filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.3));
  }
</style>