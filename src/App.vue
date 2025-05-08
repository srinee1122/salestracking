<template>
    <div class="flex flex-col min-h-screen bg-gray-100 font-sans">
      <nav class="bg-indigo-700 text-white shadow-md sticky top-0 z-10">
        <div class="container mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
           <div class="flex justify-between items-center h-16">
              <div class="flex-shrink-0 flex items-center">
                 <span class="font-bold text-xl">Sales Tracker</span>
                 </div>
              <div class="hidden sm:ml-6 sm:flex sm:space-x-4">
                 <router-link
                   to="/"
                   class="px-3 py-2 rounded-md text-sm font-medium hover:bg-indigo-600 transition duration-150"
                   active-class="bg-indigo-800 font-semibold"
                   exact-active-class="bg-indigo-800 font-semibold"
                   >Salespeople</router-link>
                 <router-link
                   to="/products"
                   class="px-3 py-2 rounded-md text-sm font-medium hover:bg-indigo-600 transition duration-150"
                   active-class="bg-indigo-800 font-semibold"
                   exact-active-class="bg-indigo-800 font-semibold"
                   >Products</router-link>
                   <router-link
                   to="/saleentry"
                   class="px-3 py-2 rounded-md text-sm font-medium hover:bg-indigo-600 transition duration-150"
                   active-class="bg-indigo-800 font-semibold"
                   exact-active-class="bg-indigo-800 font-semibold"
                   >Sale Entry</router-link>
                 </div>
           </div>
        </div>
      </nav>
  
      <main class="flex-grow container mx-auto max-w-7xl py-6 sm:px-6 lg:px-8">
        <router-view v-slot="{ Component }">
          <transition name="fade" mode="out-in">
            <component :is="Component" />
          </transition>
        </router-view>
      </main>
  
      <footer class="bg-gray-200 text-center p-4 text-sm text-gray-600">
        &copy; {{ new Date().getFullYear() }} Sales Tracking App
      </footer>
    </div>
  </template>
  
  <script setup lang="ts">
  // This root component often doesn't need much specific script logic itself,
  // as the logic resides within the view components rendered by <router-view>.
  console.log("Root App.vue component setup.");
  
  // No imports needed here unless you add logic specific to the overall layout
  </script>
  
  <style>
  /* Global styles or base styles can go here */
  /* Ensure body and html take full height if needed for sticky footers etc. */
  /* We apply min-h-screen to the root div instead */
  
  /* Basic styles for page transitions (optional) */
  .fade-enter-active,
  .fade-leave-active {
    transition: opacity 0.2s ease-in-out;
  }
  
  .fade-enter-from,
  .fade-leave-to {
    opacity: 0;
  }
  
  /* Ensure Tailwind base styles are applied if not using a global CSS file */
  /* @tailwind base; */
  /* @tailwind components; */
  /* @tailwind utilities; */
  
  /* Ensure body has base font if not set elsewhere */
  body {
     font-family: 'Inter', sans-serif;
  }
  </style>
  