<template>
  <nav>
    <h1>VooDoo</h1>
    <form @submit="foo">
      <input type="text" placeholder="login" v-model="login">
      <input type="password" placeholder="password" v-model="pw">
      <button type="submit">Login</button>
    </form>
    <button @click="bar()">Test</button>
  </nav>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import axios from 'axios';

export default defineComponent({
  name: 'app',
  data() {
    return {
      login: '',
      pw: '',
    }
  },
  methods: {
    async foo() {
      const url = 'api/login'
      const data = {
        email: this.login,
        password: this.pw,
      };
      const res = await axios.post(url, data, {
          headers: {
            Accept: "application/json",
            "Content-Type": "application/json;charset=UTF-8",
          },
        });
      localStorage.setItem('token', res.data.data.jwtoken);
    },
    async bar() {
      const config = {
          headers: { Authorization: `Bearer ${localStorage.getItem('token')}` }
      };
      axios.get( 
        'api/lists',
        config
      ).then(console.log).catch(console.log);
    }
  }
})

</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

nav {
  padding: 30px;
}

nav a {
  font-weight: bold;
  color: #2c3e50;
}

nav a.router-link-exact-active {
  color: #42b983;
}
</style>
