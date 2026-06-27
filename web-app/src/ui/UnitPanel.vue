<template>
  <div v-if="unit" class="unit-panel">
    <h3>{{ unit.name }}</h3>
    <div class="stat-row"><span>Сытость</span><div class="bar"><div class="bar-fill food" :style="{ width: unit.food + '%' }"></div></div><span class="value">{{ Math.round(unit.food) }}/100</span></div>
    <div class="stat-row"><span>Бодрость</span><div class="bar"><div class="bar-fill energy" :style="{ width: unit.energy + '%' }"></div></div><span class="value">{{ Math.round(unit.energy) }}/100</span></div>
    <div class="state">Состояние: <strong>{{ stateLabel }}</strong></div>
    <div v-if="unit.debuffs.length" class="debuffs">
      <span v-for="d in unit.debuffs" :key="d" class="debuff">{{ d === 'hungry' ? '🍽 Голод!' : '💤 Усталость!' }}</span>
    </div>
  </div>
  <div v-else class="unit-panel empty">Выберите юнита на карте</div>
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { useGameStore } from '../store/gameStore';
const store = useGameStore();
const unit = computed(() => store.selectedUnit);
const stateLabel = computed(() => {
  if (!unit.value) return '';
  const m: Record<string, string> = { Idle: 'Безделье', Moving: 'Идёт к цели', Eating: 'Ест', Sleeping: 'Спит' };
  return m[unit.value.state] || unit.value.state;
});
</script>
<style scoped>
.unit-panel { width: 240px; padding: 12px; background: #16213e; border-left: 1px solid #0f3460; }
.unit-panel.empty { display: flex; align-items: center; justify-content: center; color: #666; font-style: italic; }
h3 { margin: 0 0 8px; font-size: 16px; }
.stat-row { display: flex; align-items: center; gap: 8px; margin: 4px 0; font-size: 12px; }
.bar { flex: 1; height: 10px; background: #333; border-radius: 2px; overflow: hidden; }
.bar-fill { height: 100%; transition: width 0.3s; }
.bar-fill.food { background: #44bb44; }
.bar-fill.energy { background: #bbbb44; }
.value { width: 50px; text-align: right; }
.state { margin-top: 8px; font-size: 13px; }
.debuffs { margin-top: 4px; }
.debuff { color: #ff4444; font-weight: bold; margin-right: 8px; }
</style>