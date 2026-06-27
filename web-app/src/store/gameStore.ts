import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export interface UnitData {
  id: number; name: string; x: number; y: number;
  food: number; energy: number; state: string; debuffs: string[];
}

export interface GameState {
  units: UnitData[];
  map: { kind: string; building: string | null }[][];
}

export const useGameStore = defineStore('game', () => {
  const gameState = ref<GameState | null>(null);
  const gameSpeed = ref(1);
  const isPaused = ref(false);
  const selectedUnitId = ref<number | null>(null);
  const buildMode = ref<string | null>(null);
  const isInitialized = ref(false);

  const selectedUnit = computed(() => {
    if (selectedUnitId.value === null || !gameState.value) return null;
    return gameState.value.units.find(u => u.id === selectedUnitId.value) ?? null;
  });

  function setState(s: GameState) { gameState.value = s; }
  function setSpeed(s: number) { gameSpeed.value = s; isPaused.value = false; }
  function togglePause() { isPaused.value = !isPaused.value; }
  function selectUnit(id: number | null) { selectedUnitId.value = id; }
  function setBuildMode(m: string | null) { buildMode.value = m; }
  function setInitialized() { isInitialized.value = true; }

  return { gameState, gameSpeed, isPaused, selectedUnitId, selectedUnit, buildMode, isInitialized, setState, setSpeed, togglePause, selectUnit, setBuildMode, setInitialized };
});