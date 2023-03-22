<template>
  <nav>
    <h1>VooDoo</h1>
    <form >
      <input type="text" placeholder="login" v-model="login">
      <input type="password" placeholder="password" v-model="pw">
      <button @click="login_handle">Login</button>
    </form>
    <form >
      <input type="text" placeholder="login" v-model="login">
      <input type="text" placeholder="email" v-model="email">
      <input type="password" placeholder="password" v-model="pw">
      <button @click="register">Register</button>
    </form>
    <button @click="test">Test</button>
    <button @click="logout">Logout</button>
  </nav>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import axios from 'axios';
import { LoginUser, RegisterUser } from './services/auth-service';
import authHeader from './services/auth-header'

export default defineComponent({
  name: 'app',
  data() {
    return {
      login: '',
      pw: '',
      email: '',
    }
  },
  methods: {
    async login_handle(evt: Event) {
      evt.preventDefault();
      const user: LoginUser = {
        email: this.login,
        password: this.pw,
      };
      this.$store.dispatch('auth/login', user);
    },
    async register(evt: Event) {
      evt.preventDefault();
      const user: RegisterUser = {
        email: this.email,
        username: this.login,
        password: this.pw,
      };
      this.$store.dispatch('auth/register', user);
    },
    async test() {
      const config = {
        headers: authHeader()
      };
      axios.get( 
        'api/lists',
        config
      ).then(console.log).catch(console.log);
    },
    logout() {
      this.$store.dispatch('auth/logout');
    },
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
