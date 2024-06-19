<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

const retorno = ref(false);
async function handleClick(){
  retorno.value = await invoke("open"); // Update 'retorno' using the ref
}

</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <button @click="handleClick">teste</button>

  <div v-if="retorno">
    <p>Greeting successful!</p>
  </div>
  <div v-else>
    <p>Greeting failed!</p>
  </div>

  <p>{{ greetMsg }}</p>
</template>
