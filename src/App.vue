<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { onMounted } from 'vue';
import { onBeforeUnmount  } from 'vue';
const greetMsg = ref("");
const name = ref("");
const x = ref('-');
const y = ref('-');
const keyPressed = ref('');
let unlisten = null;
let unlistenKey = null;
async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}


const handleFrontendKeydown = (e: KeyboardEvent) => {
  keyPressed.value = e.key; // 或 e.code / e.keyCode 根据你想要的显示方式
};

onMounted(async () => {
  await invoke("start_input_listener");
  invoke('set_complete', { task: 'frontend' });
  unlisten = await listen('mouse-move', event => {
    const [mouseX, mouseY] = event.payload;
    x.value = mouseX;
    y.value = mouseY;
  });
  unlistenKey = await listen('key-press', event => {
    keyPressed.value = event.payload || '';
  });
  // 前端键盘事件监听（浏览器内，输入框也能触发）
  window.addEventListener("keydown", handleFrontendKeydown, true); // 第三个参数为 true 捕获阶段
});

onBeforeUnmount(() => {
  // 组件销毁时取消监听，防止内存泄漏
  if (unlisten) {
    unlisten();
  }
  if (unlistenKey) unlistenKey();
  window.removeEventListener("keydown", handleFrontendKeydown, true);
});


</script>

<template>
  <main class="container">
    <h1>Welcome to Tauri + Vue</h1>

    <div class="row">
      <a href="https://vite.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>
    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>
    <div class="mouse-position">
    Mouse Position: (x: {{ x }}, y: {{ y }})
  </div>
  <input
      type="text"
      readonly
      :value="keyPressed"
      placeholder="当前按下的键盘按键"
      class="key-input"
    />
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}


a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

.key-input {
  width: 100%;
  padding: 6px;
  font-family: monospace;
  font-size: 1rem;
}
@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>