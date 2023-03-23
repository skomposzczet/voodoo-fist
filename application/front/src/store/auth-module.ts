import AuthService from '../services/auth-service';
import {LoginUser, RegisterUser} from '../services/auth-service'
import {Commit} from 'vuex'

interface State {
    status: boolean,
    token: string,
}

const token = localStorage.getItem('token');
const initialState: State = token
    ? { status: true, token: token } as State
    : { status: false, token: '' } as State;

export const auth = {
    namespaced: true,
    state: initialState,
    actions: {
        async login({ commit }: {commit: Commit}, user: LoginUser) {
            try {
                await AuthService.login(user);
                commit('loginSuccess');
            } catch(err) {
                commit('loginFailure');
                throw err;
            }
        },
        logout({ commit }: {commit: Commit}) {
            AuthService.logout();
            commit('logout');
        },
        async register({ commit }: {commit: Commit}, user: RegisterUser) {
            try {
                await AuthService.register(user)
                commit('registerSuccess');
            } catch(err) {
                commit('registerFailure');
                throw err;
            }
        }
    },
    mutations: {
        loginSuccess(state: State, token: string) {
            state.status = true;
            state.token = token;
        },
        loginFailure(state: State) {
            state.status = false;
            state.token = '';
        },
        logout(state: State) {
            state.status = false;
            state.token = '';
        },
        registerSuccess(state: State) {
            state.status = false;
        },
        registerFailure(state: State) {
            state.status = false;
        }
    }
};