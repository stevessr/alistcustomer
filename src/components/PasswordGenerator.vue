<script setup lang="ts">
import { ref } from 'vue'

const password = ref('')
const length = ref(12)
const includeUppercase = ref(true)
const includeNumbers = ref(true)
const includeSymbols = ref(true)

const generatePassword = () => {
  const lowercase = 'abcdefghijklmnopqrstuvwxyz'
  const uppercase = includeUppercase.value ? 'ABCDEFGHIJKLMNOPQRSTUVWXYZ' : ''
  const numbers = includeNumbers.value ? '0123456789' : ''
  const symbols = includeSymbols.value ? '!@#$%^&*()_+-=[]{}|;:,.<>?' : ''
  
  const charset = lowercase + uppercase + numbers + symbols
  let result = ''
  
  for (let i = 0; i < length.value; i++) {
    result += charset.charAt(Math.floor(Math.random() * charset.length))
  }
  
  password.value = result
}

const copyPassword = async () => {
  try {
    await navigator.clipboard.writeText(password.value)
    alert('Password copied to clipboard!')
  } catch (err) {
    console.error('Failed to copy password:', err)
  }
}
</script>

<template>
  <div class="max-w-md mx-auto p-6 bg-white rounded-lg shadow-md">
    <h2 class="text-xl font-bold mb-4">Password Generator</h2>
    
    <div class="mb-4">
      <label class="block mb-2">Password Length</label>
      <input 
        v-model.number="length"
        type="number"
        min="8"
        max="32"
        class="w-full p-2 border rounded"
      />
    </div>

    <div class="mb-4">
      <label class="flex items-center">
        <input 
          v-model="includeUppercase"
          type="checkbox"
          class="mr-2"
        />
        Include Uppercase Letters
      </label>
      
      <label class="flex items-center">
        <input 
          v-model="includeNumbers"
          type="checkbox"
          class="mr-2"
        />
        Include Numbers
      </label>
      
      <label class="flex items-center">
        <input 
          v-model="includeSymbols"
          type="checkbox"
          class="mr-2"
        />
        Include Symbols
      </label>
    </div>

    <button 
      @click="generatePassword"
      class="w-full bg-blue-500 text-white p-2 rounded hover:bg-blue-600 mb-4"
    >
      Generate Password
    </button>

    <div v-if="password" class="mt-4">
      <div class="flex items-center gap-2">
        <input 
          :value="password"
          readonly
          class="w-full p-2 border rounded"
        />
        <button
          @click="copyPassword"
          class="bg-green-500 text-white p-2 rounded hover:bg-green-600"
        >
          Copy
        </button>
      </div>
    </div>
  </div>
</template>
