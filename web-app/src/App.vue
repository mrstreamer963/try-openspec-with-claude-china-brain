<template>
  <div id="game-app">
    <MainPanel />
    <div id="game-area">
      <canvas ref="canvasRef"></canvas>
      <UnitPanel />
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import MainPanel from './ui/MainPanel.vue';
import UnitPanel from './ui/UnitPanel.vue';
import { GameEngine } from './game/engine';
const canvasRef = ref<HTMLCanvasElement | null>(null);
let engine: GameEngine | null = null;
onMounted(() => { if (canvasRef.value) engine = new GameEngine(canvasRef.value); });
onUnmounted(() => engine?.destroy());
</script>
<style scoped>
#game-app { display: flex; flex-direction: column; height: 100vh; }
#game-area { flex: 1; display: flex; align-items: stretch; justify-content: center; padding: 8px; gap: 8px; }
#game-canvas { image-rendering: pixelated; cursor: crosshair; }
</style>