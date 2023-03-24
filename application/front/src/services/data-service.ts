import { ListPatch } from '@/api-types';
import axios from 'axios';
import authHeader from "./auth-header";

class DataService {
    get_lists() {
        return axios.get('api/lists', { headers: authHeader() });
    }

    patch_list(patch: ListPatch) {
        return axios.patch('api/list', patch, {headers: authHeader()});
    }
}

export default new DataService();