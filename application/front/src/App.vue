<template>
  <div class="wrapper" v-bind:style="bgc">
    <div class="container">
      <MyHeader title="VooDoo Fist"/>
      <router-view></router-view>
      <MyFooter />
    </div>
  </div>
</template>

<script lang="ts">
import './assets/css/main.css';
import { defineComponent } from "vue"
import MyHeader from "./components/MyHeader.vue"
import MyFooter from "./components/MyFooter.vue"

export default defineComponent({
  name: 'App',
  components: {
    MyHeader,
    MyFooter,
  },
  data() {
    return {
      bgc: {
        backgroundColor: '#262A33',
      },
    }
  },
});
</script>

