<template>
  <div class="main-panel">
    <div class="speed-controls">
      <button v-for="s in [1,5,10]" :key="s" :class="{ active: store.gameSpeed === s && !store.isPaused }" @click="store.setSpeed(s)">▶ ×{{ s }}</button>
      <button :class="{ active: store.isPaused }" @click="store.togglePause()">⏸</button>
    </div>
    <div class="build-controls">
      <button :class="{ active: store.buildMode === 'bed' }" @click="store.setBuildMode(store.buildMode === 'bed' ? null : 'bed')">🛏️ Кровать</button>
      <button :class="{ active: store.buildMode === 'berry_bush' }" @click="store.setBuildMode(store.buildMode === 'berry_bush' ? null : 'berry_bush')">🫐 Куст</button>
    </div>
  </div>
</template>
<script setup lang="ts">
import { useGameStore } from '../store/gameStore';
const store = useGameStore();
</script>
<style scoped>
.main-panel { display: flex; gap: 16px; padding: 8px 16px; background: #16213e; border-bottom: 1px solid #0f3460; align-items: center; }
.speed-controls, .build-controls { display: flex; gap: 4px; }
button { padding: 4px 12px; background: #0f3460; color: #eee; border: 1px solid #1a4a8a; cursor: pointer; font-family: monospace; font-size: 14px; }
button:hover { background: #1a4a8a; }
button.active { background: #e94560; border-color: #e94560; }
</style>