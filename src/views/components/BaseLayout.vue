<template>
  <div class="base-layout">
    <header class="header">
      <slot name="header"></slot>
    </header>
    <main class="main-content">
      <slot></slot>
    </main>
    <footer class="footer">
      <slot name="footer"></slot>
    </footer>
  </div>
</template>

<style scoped>
.base-layout {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  padding: 1rem;
}

.header {
  margin-bottom: 2rem;
}

.main-content {
  flex: 1;
  max-width: 1200px;
  width: 100%;
  margin: 0 auto;
  padding: 0 1rem;
}

.footer {
  margin-top: 2rem;
  padding: 1rem 0;
  text-align: center;
}

@media (max-width: 768px) {
  .base-layout {
    padding: 0.5rem;
  }
  
  .main-content {
    padding: 0 0.5rem;
  }
}
</style>
