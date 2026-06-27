<template>
  <div id="game-app">
    <MainPanel />
    <div id="game-area">
      <canvas ref="canvasRef" id="game-canvas"></canvas>
      <UnitPanel />
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue';
import MainPanel from './ui/MainPanel.vue';
import UnitPanel from './ui/UnitPanel.vue';
const canvasRef = ref<HTMLCanvasElement | null>(null);
</script>
<style scoped>
#game-app { display: flex; flex-direction: column; height: 100vh; }
#game-area { flex: 1; display: flex; align-items: stretch; justify-content: center; padding: 8px; gap: 8px; }
#game-canvas { image-rendering: pixelated; cursor: crosshair; }
</style>