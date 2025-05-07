// src/main.ts

import { createApp } from 'vue';
// 1. Import the root Vue component (we'll create a new simple one in the next step)
import App from './App.vue';
// 2. Import the router configuration we created
import router from './router'; // Imports src/router/index.ts by default

// Create the Vue application instance
const app = createApp(App);

// 3. Tell the Vue app instance to use the router plugin
//    This makes <router-link> and <router-view> available globally
//    and connects the app to the routes we defined.
app.use(router);

// Mount the application to the #app element in index.html
app.mount('#app');

console.log("Vue app mounted with router."); // Updated log message
