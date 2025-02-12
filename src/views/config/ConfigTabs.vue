<template>
  <div>
    <div class="tabs">
      <div 
        v-for="tab in tabs" 
        :key="tab.id" 
        :class="['tab', { active: activeTab === tab.id }]"
        @click="activeTab = tab.id"
      >
        {{ tab.label }}
      </div>
    </div>
    <div class="tab-content">
      <component 
        v-if="activeComponent" 
        :key="activeComponent.id" 
        :is="activeComponent.component" 
        :data-type="activeComponent.id"
        :style="{ borderLeftColor: colors[activeComponent.id] }"
        v-bind="activeComponent.props"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, defineProps, computed } from 'vue';

const props = defineProps<{
  tabs: { id: string; label: string; component: any; props: () => Record<string, unknown> }[];
  colors: Record<string, string>;
}>();

const activeTab = ref(props.tabs[0].id);

const activeComponent = computed(() => {
  return props.tabs.find(tab => tab.id === activeTab.value);
});
</script>

<style scoped>
.tabs {
  display: flex;
  cursor: pointer;
  margin-bottom: 16px;
}

.tab {
  padding: 12px 16px;
  border: 1px solid transparent;
  border-radius: 4px 4px 0 0;
  background: #f1f1f1;
  margin-right: 4px;
}

.tab.active {
  background: white;
  border-bottom: 1px solid transparent;
}

.tab-content {
  border: 1px solid #ccc;
  padding: 16px;
  border-radius: 0 0 4px 4px;
}
</style>
