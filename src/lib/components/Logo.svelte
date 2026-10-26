<script>
  let { size = 40 } = $props();
</script>

<svg
  width={size}
  height={size}
  viewBox="0 0 100 100"
  fill="none"
  xmlns="http://www.w3.org/2000/svg"
  class="logo"
>
  <!-- 1. КРИСТАЛЛ (внизу, крупный, острые грани) -->
  <g>
    <!-- Левая грань (тень) -->
    <path d="M15 85 L35 55 L30 70 L40 95 L20 90 Z" fill="#b87c1a" stroke="#8c5a2b" stroke-width="1" stroke-linejoin="miter" />
    <!-- Центральная левая (светлая) -->
    <path d="M35 55 L50 40 L45 80 L30 70 Z" fill="#f7c55c" stroke="#d48a14" stroke-width="1" stroke-linejoin="miter" />
    <!-- Верхняя грань (самая светлая) -->
    <path d="M50 40 L65 35 L70 60 L45 80 Z" fill="#fde2a0" stroke="#d48a14" stroke-width="1" stroke-linejoin="miter" />
    <!-- Правая грань (тень) -->
    <path d="M65 35 L85 50 L80 80 L70 60 Z" fill="#d48a14" stroke="#b87c1a" stroke-width="1" stroke-linejoin="miter" />
    <!-- Нижняя грань (очень тёмная) -->
    <path d="M45 80 L70 60 L80 80 L40 95 Z" fill="#8c5a2b" stroke="#5a3a1a" stroke-width="1" stroke-linejoin="miter" />
    <!-- Блик -->
    <path d="M50 40 L65 35 L60 45 L48 50 Z" fill="#ffffff" opacity="0.4" />
  </g>

  <!-- 2. ДЫМ (много, идёт вверх от кристалла) -->
  <g stroke="#bdb2a6" stroke-linecap="round" fill="none" opacity="0.9">
    <!-- Главная струя -->
    <path d="M50 50 Q35 35 45 20 Q55 5 40 -10" stroke-width="5" />
    <!-- Боковые -->
    <path d="M45 55 Q25 45 30 30 Q35 15 20 0" stroke-width="4" opacity="0.7" />
    <path d="M55 55 Q75 45 70 30 Q65 15 80 0" stroke-width="4" opacity="0.7" />
    <!-- Дополнительные маленькие струйки -->
    <path d="M50 60 Q40 50 45 40" stroke-width="2" opacity="0.5" />
    <path d="M55 60 Q65 50 60 40" stroke-width="2" opacity="0.5" />
  </g>

  <!-- 3. ИСКРЫ (на вершине кристалла) -->
  <circle cx="50" cy="50" r="3" fill="#f5b83d" />
  <circle cx="55" cy="45" r="1.5" fill="#f7c55c" />
  <circle cx="45" cy="45" r="1.5" fill="#f7c55c" />
</svg>

<style>
  .logo {
    flex-shrink: 0;
    display: block;
  }
</style>