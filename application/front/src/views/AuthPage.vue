<template>
    <div v-if="logging">
        <form @submit="login">
            <input type="text" placeholder="email" v-model="email"/>
            <input type="password" placeholder="password" v-model="password"/>
        </form>
        <GreatButton @click="login" :style="{backgroundColor: 'green'}" text="Login"/>
    </div>
    <div v-else class="formdiv">
        <form @submit="register">
            <input type="text" placeholder="email" v-model="email"/>
            <input type="text" placeholder="username" v-model="username"/>
            <input type="password" placeholder="password" v-model="password"/>
        </form>
        <GreatButton @click="register" :style="{backgroundColor: 'green'}" text="Register"/>
    </div>
    <a @click="toggle" class="hvr"> {{logging ? 'Not a member? Create account.' : 'Already a member? Login.'}}</a>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import GreatButton from '../components/GreatButton.vue'

export default defineComponent({
    name: 'AuthPage',
    components: {
        GreatButton,
    },
    data() {
        return {
            logging: true,
            email: '',
            username: '',
            password: '',
        }
    },
    methods: {
        toggle() {
            this.logging = !this.logging;
            this.email = this.username = this.password = '';
        },
        login() {
            console.log('')
        },
        register() {
            console.log('')
        },
    },
});
</script>

<style>
.hvr:hover {
    color: gray;
    cursor: pointer;
}
</style>