<template>
    <h3 v-if="error !== ''" style="color: red">{{ error }}</h3>
    <div v-if="log_reg_switch">
        <form @submit="login">
            <input type="text" placeholder="email" v-model="email"/>
            <input type="password" placeholder="password" v-model="password"/><br>
            <GreatButton :style="{backgroundColor: 'green'}" text="Login"/>
        </form>
    </div>
    <div v-else class="formdiv">
        <form @submit="register">
            <input type="text" placeholder="email" v-model="email"/>
            <input type="text" placeholder="username" v-model="username"/>
            <input type="password" placeholder="password" v-model="password"/><br>
            <GreatButton :style="{backgroundColor: 'green'}" text="Register"/>
        </form>
    </div>
    <a @click="toggle" class="hvr"> {{log_reg_switch ? 'Not a member? Create account.' : 'Already a member? Login.'}}</a>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import GreatButton from '../components/GreatButton.vue'
import { LoginUser, RegisterUser } from '@/services/auth-service';
import { AxiosError } from 'axios';

export default defineComponent({
    name: 'AuthPage',
    components: {
        GreatButton,
    },
    data() {
        return {
            log_reg_switch: true,
            error: '',
            email: '',
            username: '',
            password: '',
        }
    },
    created() {
        if (this.$store.state.auth.status) {
            this.$router.push("/");
        }
    },
    methods: {
        toggle() {
            this.log_reg_switch = !this.log_reg_switch;
            this.email = this.username = this.password = this.error = '';
        },
        async login() {
            const user: LoginUser = {email: this.email, password: this.password};
            try {
                await this.$store.dispatch('auth/login', user);
                this.$router.push('/');
            } catch(err) {
                if (!(err instanceof AxiosError)) {
                    this.error = 'Login failed';
                } else {
                    this.error = err.response?.status === 401 ? 'Bad email or password.' : 'Login failed';
                }
            }
        },
        async register() {
            const user: RegisterUser = {email: this.email, username: this.username, password: this.password};
            try {
                await this.$store.dispatch('auth/register', user);
                this.toggle();
            } catch(err) {
                this.error = 'Registering failed.';
            }
        },
    },
});
</script>