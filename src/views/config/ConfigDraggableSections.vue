<script setup lang="ts">
import draggable from 'vuedraggable';
import { ref } from 'vue';

interface ConfigComponent {
  id: string;
  component: any;
}

const props = defineProps<{
  components: ConfigComponent[];
  colors: Record<string, string>;
}>();

const emit = defineEmits(['update:components']);
</script>

<template>
  <div class="config-layout">
    <div class="config-grid">
      <draggable 
        v-model="props.components"
        item-key="id"
        handle=".drag-handle"
        class="draggable-container"
      >
        <template #item="{ element }">
          <div class="config-section-wrapper">
            <div class="drag-handle">☰</div>
            <component 
              :is="element.component" 
              class="config-section"
              :data-type="element.id"
              :style="{ borderLeftColor: colors[element.id] }"
            />
          </div>
        </template>
      </draggable>
    </div>
  </div>
</template>

<style scoped>
.draggable-container {
  display: flex;
  flex-direction: column;
  gap: 24px;
  height: auto;
  width: auto;
}

.config-section-wrapper {
  position: relative;
  background: var(--n-color-modal);
  border-radius: var(--n-border-radius);
  box-shadow: var(--n-box-shadow);
  width: auto;
  height: auto;
}

.drag-handle {
  position: absolute;
  left: 8px;
  top: 8px;
  cursor: move;
  padding: 8px;
  color: var(--n-text-color);
  opacity: 0.5;
  transition: opacity 0.2s ease;
}

.drag-handle:hover {
  opacity: 1;
}

.config-section {
  padding: 24px;
  padding-left: 40px;
  border-left: 4px solid;
}
</style>
