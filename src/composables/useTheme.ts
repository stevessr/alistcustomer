import { ref, watchEffect } from 'vue'

export function useTheme() {
  const isDark = ref(false)
  
  // Initialize theme from localStorage or system preference
  const storedTheme = localStorage.getItem('theme')
  const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
  
  isDark.value = storedTheme 
    ? storedTheme === 'dark'
    : systemPrefersDark

  // Watch for changes and update localStorage
  watchEffect(() => {
    document.documentElement.setAttribute('data-theme', isDark.value ? 'dark' : 'light')
    localStorage.setItem('theme', isDark.value ? 'dark' : 'light')
  })

  // Watch system preference changes
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
  mediaQuery.addEventListener('change', (e) => {
    if (!localStorage.getItem('theme')) {
      isDark.value = e.matches
    }
  })

  function toggleTheme() {
    isDark.value = !isDark.value
  }

  return {
    isDark,
    toggleTheme
  }
}
