<script setup lang="ts">
import { ref } from 'vue';
import { NCard, NColorPicker } from 'naive-ui';
import Transition from 'naive-ui';

const showPicker = ref(false);

interface SectionColors {
  general: string;
  database: string;
  meilisearch: string;
  scheme: string;
  log: string;
  task: string;
  cors: string;
  s3: string;
  ftp: string;
  sftp: string;
}

const props = defineProps<{
  colors: SectionColors;
}>();

const emit = defineEmits(['update:colors']);

const saveColors = () => {
  localStorage.setItem('section_colors', JSON.stringify(props.colors));
};

const defaultColors = {
  general: '#ffffff',
  database: '#ffffff',
  meilisearch: '#ffffff',
  scheme: '#ffffff',
  log: '#ffffff',
  task: '#ffffff',
  cors: '#ffffff',
  s3: '#ffffff',
  ftp: '#ffffff',
  sftp: '#ffffff'
};

const resetColors = () => {
  emit('update:colors', { ...defaultColors });
  saveColors();
};

const handleColorChange = (section: keyof SectionColors, color: string) => {
  const newColors = { ...props.colors, [section]: color };
  emit('update:colors', newColors);
  saveColors();
};
</script>

<template>
  <div>
    <div class="button-group">
      <button 
        class="picker-trigger"
        @click="showPicker = !showPicker"
      >
        {{ showPicker ? '隐藏' : '显示' }} 颜色选择器
      </button>
      <button class="reset-button" @click="resetColors">
        重置颜色
      </button>
    </div>
    
    <Transition name="fade">
      <NCard
        v-if="showPicker"
        class="color-picker-popup"
        :bordered="false"
        size="small"
        style="transform: translate(-50%, -50%)"
      >
        <button class="close-button" @click="showPicker = false">
          ×
        </button>
        <div class="color-picker-grid">
          <div 
            v-for="(color, section) in colors" 
            :key="section" 
            class="color-picker-item"
            :style="{ backgroundColor: color }"
          >
            <label :for="`${section}-color`">{{ 
              section === 'general' ? '常规' :
              section === 'database' ? '数据库' :
              section === 'meilisearch' ? '搜索' :
              section === 'scheme' ? '方案' :
              section === 'log' ? '日志' :
              section === 'task' ? '任务' :
              section === 'cors' ? '跨域' :
              section === 's3' ? 'S3存储' :
              section === 'ftp' ? 'FTP' :
              section === 'sftp' ? 'SFTP' : section
            }}</label>
            <n-color-picker
              :id="`${section}-color`"
              :value="color"
              :show-alpha="false"
              :modes="['hex']"
              @update:value="handleColorChange(section, $event)"
            />
          </div>
        </div>
      </NCard>
    </Transition>
  </div>
</template>

<style scoped>
.button-group {
  display: flex;
  justify-content: center;
  gap: 8px;
  margin-bottom: 12px;
  width: 100%;
}

.picker-trigger, .reset-button {
  padding: 8px 16px;
  border-radius: 4px;
  border: 1px solid var(--n-border-color);
  background-color: var(--n-color);
  cursor: pointer;
}

.reset-button {
  background-color: #ffebee;
  color: #c62828;
}

.reset-button:hover {
  background-color: #ffcdd2;
}

.picker-trigger:hover {
  background-color: var(--n-color-hover);
}

.color-picker-popup {
  position: fixed;
  z-index: 1000;
  top: 50%;
  left: 50%;
}

.close-button {
  position: absolute;
  top: 8px;
  right: 8px;
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  color: var(--n-text-color);
  font-size: 20px;
  line-height: 1;
  cursor: pointer;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}

.close-button:hover {
  color: var(--n-text-color-hover);
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.color-picker-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
  padding: 16px;
  width: 400px;
}

.color-picker-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 12px;
  border-radius: 8px;
  background: var(--n-color);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  transition: all 0.2s ease;
}

.color-picker-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
}

.color-picker-item label {
  font-weight: 500;
  color: var(--n-text-color);
  text-transform: capitalize;
}

.color-picker-item .n-color-picker {
  width: 100%;
}
</style>
