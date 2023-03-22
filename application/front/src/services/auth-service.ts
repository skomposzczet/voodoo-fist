import axios from 'axios';

export interface LoginUser {
    email: string,
    password: string,
}

export interface RegisterUser {
    email: string,
    username: string,
    password: string,
}

class AuthService {
    async login(user: LoginUser) {
        const response = await axios.post('api/login', {
                email: user.email,
                password: user.password
            })
        if (response.status === 200 && response.data.data.jwtoken) {
            localStorage.setItem('token', response.data.data.jwtoken);
        }
    }

    logout() {
        localStorage.removeItem('token');
    }

    async register(user: RegisterUser) {
        axios.post('api/register', {
            username: user.username,
            email: user.email,
            password: user.password
        });
    }
}

export default new AuthService();