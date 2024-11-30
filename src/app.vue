<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Button, Card } from "primevue";
import 'primeflex/primeflex.css';

interface NetworkInfo {
  name: string;
  ips: string[];
  mac: string;
}

const networks = ref<NetworkInfo[]>([]);

onMounted(async () => {
  networks.value = await invoke<NetworkInfo[]>("get_network_info");
});

function loginNetwork() {
  const ip = networks.value[0].ips[0];
  window.open(`https://sso.dlut.edu.cn/cas/login?service=http%3A%2F%2F172.20.30.2%3A8080%2FSelf%2Fsso_login%3Fwlan_user_ip%3D${ip}%26authex_enable%3D%26type%3D1`, '_blank');
}

function logoutNetwork() {
  window.open("http://172.20.30.2:8080/Self/login/logout", '_blank');
}

function manageNetwork() {
  window.open("https://sso.dlut.edu.cn/cas/login?service=http%3A%2F%2F172.20.30.2%3A8080%2FSelf%2Fsso_login", '_blank');
}
</script>

<template>
  <Card>
    <template #title>网络信息</template>
    <template #content>
      <div>
        <ul>
          <li v-for="network in networks" :key="network.name">
            <strong>{{ network.name }}</strong>
            <ul>
              <li>IPs: {{ network.ips.join(", ") }}</li>
              <li>MAC: {{ network.mac }}</li>
            </ul>
          </li>
        </ul>
      </div>
    </template>
    <template #footer>
      <div class="flex gap-2">
        <Button @click="loginNetwork">登录校园网</Button>
        <Button @click="logoutNetwork" severity="danger">注销校园网</Button>
        <Button @click="manageNetwork" severity="info">网络自助</Button>
      </div>
    </template>
  </Card>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 14px;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}
</style>